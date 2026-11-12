mod combatant;
pub mod state;
mod turn;

use core::{
    array, iter,
    mem::{self},
};

use crate::{
    battle::{
        combatant::Combatant,
        state::{
            accumulator::{StateAccumulator, StateFrontier},
            battle::BattleState,
            key::{BattleKey, StateMap},
        },
        turn::{AiChoices, Branch, PendingAction, TurnAction, TurnContext},
    },
    data::{
        effects::{Effect, EffectResolution},
        move_data::Move,
        moves::{BIDE, RAGE, STRUGGLE},
    },
    journey::{MatchupGroup, MatchupKind, PlayerPolicy, TrainerItem},
    player::Player,
    pokemon::Pokemon,
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Side {
    Ai,
    Player,
}

pub struct Battle<'battle> {
    states: StateAccumulator,
    next_actions: StateMap<PendingAction>,
    player: Combatant<'battle>,
    player_has_ohko: bool,
    player_policy: &'battle PlayerPolicy,
    player_choices: StateMap<u64, (Option<usize>, &'static Move)>,
    ai: Combatant<'battle>,
    ai_choices: [AiChoices; 20],
    ai_item: Option<&'battle TrainerItem>,
    ai_has_mirror_move: bool,
    is_trainer: bool,
    batch_turns: bool,
    remaining_turns: usize,
    outcome: BattleOutcome,
}

impl<'battle> Battle<'battle> {
    const STRUGGLE_SLOT: u8 = 4;

    pub(super) fn new(player: &'battle Player, ai: &'battle Pokemon, matchup: &'battle MatchupGroup) -> Self {
        let ai_has_mirror_move = ai.moves.iter().flatten().any(|mv| matches!(mv.effect, Effect::MirrorMove));
        let copyable_moves = if ai_has_mirror_move { player.active.moves.as_slice() } else { &[] };
        let player_combatant = Combatant::new(&player.active, player.stats(), &[]);
        let ai_combatant = Combatant::new(ai, ai.stats(), copyable_moves);
        Self {
            states: StateAccumulator::default(),
            next_actions: StateMap::default(),
            player: player_combatant,
            player_has_ohko: player.active.moves.iter().flatten().any(|mv| matches!(mv.effect, Effect::Ohko)),
            player_policy: &player.policy,
            player_choices: StateMap::default(),
            ai: ai_combatant,
            ai_choices: array::from_fn(|key| {
                let disabled_slot = (key % 10 / 2).checked_sub(1).map(|slot| slot as u8);
                AiChoices::new(
                    matchup
                        .ai
                        .select(&ai.moves, &player.active.species.types, disabled_slot, key % 2 != 0, key >= 10),
                )
            }),
            ai_item: matchup.ai.item(),
            ai_has_mirror_move,
            is_trainer: !matches!(matchup.kind, MatchupKind::Wild),
            batch_turns: false,
            remaining_turns: 0,
            outcome: BattleOutcome::default(),
        }
    }

    pub(super) fn run(&mut self, max_depth: usize) -> BattleOutcome {
        self.states.clear();
        self.next_actions.clear();
        self.batch_turns = false;
        self.remaining_turns = max_depth;
        self.outcome = BattleOutcome::default();
        self.outcome.unresolved = 0.0f64;
        let mut init_state = BattleState::new(&self.player, &self.ai, self.ai_item.map_or(0, |item| item.uses));
        init_state.cap_surplus_pp(self.remaining_turns);
        self.states.frontier.entries.insert(init_state.key(), 1.0f64);
        let mut current_states = StateFrontier::default();
        for idx in 0..max_depth {
            if self.states.frontier.is_empty() {
                break;
            }
            self.remaining_turns -= 1;
            mem::swap(&mut current_states, &mut self.states.frontier);
            if self.remaining_turns > 0 && current_states.len() >= 0x0001_0000 {
                self.states.enable_cache();
            }
            self.player_choices.clear();
            let second_turn = idx % 256 == 1;
            self.batch_turns |= current_states.len() >= 4_096;
            if !self.batch_turns {
                #[expect(clippy::iter_over_hash_type, reason = "Order is not important here")]
                for (state_key, probability) in current_states.entries.drain() {
                    self.start_state_turn(&state_key, state_key.hp_pair(), probability, second_turn);
                    self.resolve_pending_actions();
                }
                continue;
            }
            let ordered_states = current_states.action_ordered();
            for batch in ordered_states.chunks(512) {
                for &(order, state_key, probabillity) in batch {
                    self.start_state_turn(state_key, order as u32, probabillity, second_turn);
                }
                self.resolve_pending_actions();
            }
            current_states.clear();
            self.states.flush();
        }
        self.outcome.unresolved += self.states.frontier.total_probability();
        self.states.clear();
        self.outcome.clone()
    }

    fn begin_action(&mut self, mut branch: Branch, mut probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let side = branch.turn.side.expect("Beginning an action requires an acting side");
        if let Some(TurnAction::AiChoice(key, priority)) = branch.turn.actions[1].as_ref()
            && side == Side::Ai
        {
            let choice_idx = usize::from(*key);
            let priority_idx = usize::from(*priority);
            let last = self.ai_choices[choice_idx].moves[priority_idx].len() - 1;
            for idx in 0..last {
                let ((slot, mv), choice_prob) = self.ai_choices[choice_idx].moves[priority_idx][idx];
                let mut alternative = branch.clone();
                alternative.turn.actions[1] = Some(TurnAction::new(Some((slot, mv))));
                self.begin_action(alternative, probability * choice_prob);
            }
            let ((slot, mv), choice_prob) = self.ai_choices[choice_idx].moves[priority_idx][last];
            branch.turn.actions[1] = Some(TurnAction::new(Some((slot, mv))));
            probability *= choice_prob;
        }
        let (slot, mv) = branch.turn.selected_move();
        let pokemon = match side {
            Side::Player => self.player.pokemon,
            Side::Ai => self.ai.pokemon,
        };
        let move_slot = slot.or_else(|| pokemon.moves.iter().position(|known| *known == Some(mv)));
        let attacker = branch.state.combatant(side);
        if move_slot.is_some_and(|idx| attacker.is_disabled(idx)) {
            branch.state.combatant_mut(side).charging_slot = None;
            self.finish_action(branch, probability);
            return;
        }
        let paralysis_prob = attacker.full_paralysis_probability();
        if paralysis_prob > 0.0f64 {
            let mut immobilized = branch.clone();
            immobilized.state.combatant_mut(side).interrupt_move();
            self.finish_action(immobilized, probability * paralysis_prob);
            probability *= 1.0f64 - paralysis_prob;
        }
        if attacker.trapping_move().is_some() {
            let expiry_prob = branch.state.continue_trapping(side);
            if expiry_prob < 1.0f64 {
                let continuing = branch.clone();
                self.finish_action(continuing, probability * (1.0 - expiry_prob));
            }
            branch.state.combatant_mut(side).end_trapping();
            self.finish_action(branch, probability * expiry_prob);
            return;
        }
        if branch.state.is_biding(side) {
            branch.state.continue_bide(side);
            self.finish_action(branch, probability);
            return;
        }
        if matches!(mv.effect, Effect::ThrashPetalDance)
            && let Some(expiry_prob) = branch.state.combatant_mut(side).advance_rampage()
        {
            if expiry_prob >= 1.0f64 {
                self.execute_move(branch, probability, true);
                return;
            }
            if expiry_prob > 0.0f64 {
                self.execute_move(branch.clone(), probability * expiry_prob, true);
                probability *= 1.0f64 - expiry_prob;
            }
        }
        self.execute_move(branch, probability, false);
    }

    fn begin_turn(&mut self, mut branch: Branch, mut probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        if let Some(TurnAction::AiChoice(key_ref, _)) = branch.turn.actions[1].as_ref() {
            let key = *key_ref;
            let [norm_prob, priority_prob] = self.ai_choices[usize::from(key)].probabilities;
            if priority_prob > 0.0f64 {
                if norm_prob > 0.0f64 {
                    let normal = branch.clone();
                    self.resolve_turn_order(normal, probability * norm_prob);
                }
                branch.turn.actions[1] = Some(TurnAction::AiChoice(key, 1));
                probability *= priority_prob;
            }
        }
        self.resolve_turn_order(branch, probability);
    }

    fn execute_move(&mut self, mut branch: Branch, mut probability: f64, ending_rampage: bool) {
        let side = branch.turn.side.expect("Executing a move requires an acting side");
        let (slot, selected_mv) = branch.turn.selected_move();
        let mv = if side == Side::Ai && matches!(selected_mv.effect, Effect::MirrorMove) {
            let Some(copied_slot) = branch.state.last_player_move else {
                self.finish_action(branch, probability);
                return;
            };
            let copied_mv = self.player_move_at(copied_slot);
            if matches!(copied_mv.effect, Effect::MirrorMove) {
                self.finish_action(branch, probability);
                return;
            }
            branch.turn.actions[1] = Some(TurnAction::new(Some((slot, copied_mv))));
            if matches!(copied_mv.effect, Effect::Charge | Effect::ThrashPetalDance) {
                branch.state.ai_copied_move = Some(copied_slot);
            }
            copied_mv
        } else {
            selected_mv
        };
        let resolution = mv.effect.resolution();
        if matches!(resolution, EffectResolution::Charge) {
            let charging_combatant = branch.state.combatant_mut(side);
            if charging_combatant.charging_slot.is_none() {
                charging_combatant.start_charging(slot.expect("A charging move must have a move slot"), mv);
                self.finish_action(branch, probability);
                return;
            }
            charging_combatant.finish_charging();
        }
        let mut pp_slot = slot;
        if matches!(mv.effect, Effect::ThrashPetalDance) {
            let attacker = branch.state.combatant_mut(side);
            if attacker.rampage.is_some() {
                pp_slot = None;
            } else {
                attacker.start_rampage(slot.expect("A rampage move must have a move slot"), mv.accuracy());
            }
        }
        if self.ai_has_mirror_move && side == Side::Player {
            branch.state.last_player_move = if mv == &STRUGGLE {
                Some(Self::STRUGGLE_SLOT)
            } else {
                slot.or_else(|| self.player.pokemon.moves.iter().position(|known_mv| *known_mv == Some(mv)))
                    .map(|mv_slot| mv_slot as u8)
            };
        }
        branch.state.begin_move(side, pp_slot, &mv.effect);
        if self.is_trainer && matches!(resolution, EffectResolution::SwitchOrEscape) {
            self.finish_action(branch, probability);
            return;
        }
        let (attacker_combatant, target_combatant, target_side) = match side {
            Side::Player => (&self.player, &self.ai, Side::Ai),
            Side::Ai => (&self.ai, &self.player, Side::Player),
        };
        if matches!(mv.effect, Effect::Trapping) {
            branch.state.combatant_mut(target_side).recharge_slot = None;
        }
        let target = branch.state.combatant(target_side);
        let target_pokemon = target_combatant.pokemon;
        let target_types = &target_pokemon.species.types;
        if matches!(resolution, EffectResolution::Disable) && !target.can_be_disabled()
            || matches!(resolution, EffectResolution::Sleep) && !target.can_be_put_to_sleep()
            || matches!(mv.effect, Effect::Paralyze) && !target.can_be_paralyzed()
        {
            self.finish_action(branch, probability);
            return;
        }
        let target_hp = target.hp;
        let attacker_lv = attacker_combatant.pokemon.level;
        let attacker_max_hp = attacker_combatant.stats.hp;
        let hit_prob = if matches!(resolution, EffectResolution::Sleep) && target.recharge_slot.is_some() {
            1.0f64
        } else if matches!(resolution, EffectResolution::Ohko)
            && !attacker_combatant.can_ohko(target_combatant, branch.state.combatant(side), target, mv)
        {
            0.0f64
        } else {
            branch.turn.hit_probability(&mut branch.state, target_types)
        };
        if ending_rampage {
            branch.state.combatant_mut(side).finish_rampage();
        }
        if hit_prob == 0.0f64 {
            branch.state.miss_move();
            self.finish_action(branch, probability);
            return;
        }
        if hit_prob < 1.0f64 {
            let mut miss = branch.clone();
            miss.state.miss_move();
            self.finish_action(miss, probability * (1.0 - hit_prob));
            probability *= hit_prob;
        }
        if probability == 0.0f64 {
            return;
        }
        match resolution {
            EffectResolution::Bide => {
                let [(short, short_prob), (long, long_prob)] = mv.effect.bide_outcomes();
                let mut alternative = branch.clone();
                alternative.state.start_bide(side, short);
                self.finish_action(alternative, probability * short_prob);
                branch.state.start_bide(side, long);
                self.finish_action(branch, probability * long_prob);
            }
            EffectResolution::Charge | EffectResolution::Damage => self.resolve_damage_rolls(branch, probability),
            EffectResolution::Disable => {
                let occupied_slots = target_pokemon.moves.map(|known_mv| known_mv.is_some());
                let pp = (target_side == Side::Player).then_some(&branch.state.player_pp);
                let mut outcomes = mv.effect.disable_outcomes(occupied_slots, pp);
                let Some((disabled, slot_prob)) = outcomes.next_back() else {
                    self.finish_action(branch, probability);
                    return;
                };
                for (disable_state, alt_prob) in outcomes {
                    let mut alternative = branch.clone();
                    alternative.state.combatant_mut(target_side).disable(disable_state);
                    self.finish_action(alternative, probability * alt_prob);
                }
                branch.state.combatant_mut(target_side).disable(disabled);
                self.finish_action(branch, probability * slot_prob);
            }
            EffectResolution::FixedDamage => {
                let damage = mv.fixed_damage(attacker_lv).unwrap_or_else(|| todo!("Fixed damage for {}", mv.name()));
                self.resolve_damage(branch, probability, damage, 1);
            }
            EffectResolution::Heal => {
                branch.state.combatant_mut(side).heal_with_move(attacker_max_hp, mv);
                self.finish_action(branch, probability);
            }
            EffectResolution::Ohko => self.resolve_damage(branch, probability, target_hp, 1),
            EffectResolution::Sleep => {
                branch.state.combatant_mut(target_side).fall_asleep();
                self.finish_action(branch, probability);
            }
            EffectResolution::Status => {
                branch.state.apply_status_move(&mv.effect, side, target_types);
                self.finish_action(branch, probability);
            }
            EffectResolution::CopyMove | EffectResolution::SwitchOrEscape => todo!(),
        }
    }

    fn finish_action(&mut self, branch: Branch, probability: f64) {
        if probability != 0.0f64 {
            self.finish_action_rolls(branch, iter::once((0, probability)));
        }
    }

    fn finish_action_rolls(&mut self, mut branch: Branch, outcomes: impl Iterator<Item = (u16, f64)>) {
        let rolls = outcomes.filter(|&(_, probability)| probability != 0.0f64);
        branch.state.clear_unused_state();
        if let Some(winner) = branch.state.winner() {
            for (_, probability) in rolls {
                self.outcome.record(winner, probability);
            }
            return;
        }
        let side = branch.turn.side.expect("Finishing an action requires an acting side");
        let (max_hp, opponent_max_hp, target_side) = match side {
            Side::Player => (self.player.stats.hp, self.ai.stats.hp, Side::Ai),
            Side::Ai => (self.ai.stats.hp, self.player.stats.hp, Side::Player),
        };
        branch.state.apply_residual_damage(side, max_hp, opponent_max_hp);
        if let Some(winner) = branch.state.winner() {
            for (_, probability) in rolls {
                self.outcome.record(winner, probability);
            }
            return;
        }
        if let Some((next_side, action)) = branch.turn.next_action() {
            let state_key = branch.state.key();
            for (extra_dmg, probability) in rolls {
                let pending_key = state_key.clone().with_hp_loss(target_side, extra_dmg);
                let pending = PendingAction::new(pending_key, next_side, action.clone());
                *self.next_actions.entry(pending).or_insert(0.0f64) += probability;
            }
            return;
        }
        branch.state.finish_turn();
        if self.remaining_turns == 0 {
            for (_, probability) in rolls {
                self.outcome.unresolved += probability;
            }
            return;
        }
        branch.state.cap_surplus_pp(self.remaining_turns);
        let state_key = branch.state.key();
        for (extra_dmg, probability) in rolls {
            self.states
                .accumulate(state_key.clone().with_hp_loss(target_side, extra_dmg), probability);
        }
    }

    fn player_move_at(&self, slot: u8) -> &'static Move {
        if slot == Self::STRUGGLE_SLOT {
            &STRUGGLE
        } else {
            self.player.pokemon.moves[usize::from(slot)].expect("A copied move must still be known")
        }
    }

    fn resolve_action(&mut self, mut branch: Branch, mut probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let side = branch.turn.side.expect("Executing an action requires an acting side");
        if side == Side::Ai
            && branch.state.ai.rage_accuracy.is_none()
            && let Some(item) = self.ai_item
        {
            let max_hp = self.ai.stats.hp;
            let item_prob = item.use_probability(branch.state.ai_items_left, branch.state.ai.hp, max_hp);
            if item_prob > 0.0f64 {
                let mut using_item = branch.clone();
                using_item.state.use_ai_item(item, &self.ai.pokemon.species.types, max_hp);
                self.finish_action(using_item, probability * item_prob);
                probability *= 1.0f64 - item_prob;
                if probability == 0.0f64 {
                    return;
                }
            }
        }
        if self.ai_has_mirror_move && side == Side::Player && (branch.state.player.is_asleep() || branch.state.player.frozen) {
            branch.state.last_player_move = None;
        }
        if let Some(wake_prob) = branch.state.combatant_mut(side).advance_sleep() {
            if wake_prob < 1.0f64 {
                let sleeping = branch.clone();
                self.finish_action(sleeping, probability * (1.0 - wake_prob));
            }
            branch.state.combatant_mut(side).wake_up();
            self.finish_action(branch, probability * wake_prob);
            return;
        }
        if branch.state.combatant(side).frozen || branch.state.is_trapped(side) || branch.turn.skips_action() {
            self.finish_action(branch, probability);
            return;
        }
        if branch.state.combatant_mut(side).recharge_slot.take().is_some() {
            self.finish_action(branch, probability);
            return;
        }
        if let Some(expiry_prob) = branch.state.combatant_mut(side).advance_disable() {
            if expiry_prob < 1.0f64 {
                let disabled = branch.clone();
                self.resolve_confusion(disabled, probability * (1.0 - expiry_prob));
            }
            branch.state.combatant_mut(side).end_disable();
            probability *= expiry_prob;
        }
        self.resolve_confusion(branch, probability);
    }

    fn resolve_confusion(&mut self, mut branch: Branch, mut probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let side = branch.turn.side.expect("Checking confusion requires an acting side");
        if let Some(expiry_prob) = branch.state.combatant_mut(side).advance_confusion() {
            if expiry_prob < 1.0f64 {
                let continuing_prob = probability * (1.0f64 - expiry_prob) * 0.5f64;
                let mut self_hit = branch.clone();
                let (combatant, opponent_reflect) = match side {
                    Side::Player => (&self.player, self_hit.state.ai.reflect),
                    Side::Ai => (&self.ai, self_hit.state.player.reflect),
                };
                let damage = combatant.confusion_damage(self_hit.state.combatant(side), opponent_reflect);
                self_hit.state.hurt_in_confusion(side, damage);
                self.finish_action(self_hit, continuing_prob);
                if expiry_prob == 0.0f64 {
                    self.begin_action(branch, continuing_prob);
                    return;
                }
                self.begin_action(branch.clone(), continuing_prob);
            }
            branch.state.combatant_mut(side).end_confusion();
            probability *= expiry_prob;
        }
        self.begin_action(branch, probability);
    }

    fn resolve_damage(&mut self, branch: Branch, probability: f64, dmg: u16, hits: u16) {
        if probability != 0.0f64 {
            self.resolve_damage_outcomes(branch, iter::once((0, probability)), dmg, hits);
        }
    }

    fn resolve_damage_outcomes(&mut self, mut branch: Branch, rolls: impl Iterator<Item = (u16, f64)> + Clone, dmg: u16, hits: u16) {
        let side = branch.turn.side.expect("Applying damage requires an acting side");
        let (slot, mv) = branch.turn.selected_move();
        let dmg_dealt = branch.state.apply_damage(dmg, mv, side, hits);
        if matches!(mv.effect, Effect::Rage) && dmg_dealt > 0 {
            branch.state.combatant_mut(side).start_rage();
        }
        if matches!(mv.effect, Effect::HyperBeam) && dmg_dealt > 0 && branch.state.winner().is_none() {
            branch.state.combatant_mut(side).recharge_slot = slot.map(|idx| idx as u8);
        }
        let max_hp = match side {
            Side::Player => self.player.stats.hp,
            Side::Ai => self.ai.stats.hp,
        };
        branch.state.combatant_mut(side).heal(mv.effect.drained_hp(dmg_dealt), max_hp);
        if matches!(mv.effect, Effect::Trapping) && branch.state.winner().is_none() {
            branch.state.start_trapping(side, mv, dmg);
            self.finish_action_rolls(branch, rolls);
            return;
        }
        let (target_side, target_types) = match side {
            Side::Player => (Side::Ai, &self.ai.pokemon.species.types),
            Side::Ai => (Side::Player, &self.player.pokemon.species.types),
        };
        let target = branch.state.combatant(target_side);
        let is_flinch = matches!(mv.effect, Effect::Flinch1 | Effect::Flinch2);
        let secondary_prob = if is_flinch && (dmg_dealt == 0 || (!branch.turn.can_flinch_target() && target.recharge_slot.is_none())) {
            0.0f64
        } else {
            mv.effect.secondary_probability(target, target_types, &mv.mv_type, side == Side::Ai)
        };
        if secondary_prob > 0.0f64 {
            let mut affected = branch.clone();
            if is_flinch {
                affected.turn.flinch_target();
                affected.state.combatant_mut(target_side).recharge_slot = None;
            } else {
                affected.state.apply_secondary(&mv.effect, side);
            }
            self.finish_action_rolls(
                affected,
                rolls.clone().map(|(extra_dmg, probability)| (extra_dmg, probability * secondary_prob)),
            );
            let unaffected_prob = 1.0f64 - secondary_prob;
            if unaffected_prob > 0.0f64 {
                self.finish_action_rolls(branch, rolls.map(|(extra_dmg, probability)| (extra_dmg, probability * unaffected_prob)));
            }
            return;
        }
        self.finish_action_rolls(branch, rolls);
    }

    fn resolve_damage_rolls(&mut self, branch: Branch, probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let (_, mv) = branch.turn.selected_move();
        let side = branch.turn.side.expect("Damage resolution requires an acting side");
        let (attacker, target, attacker_state, target_state) = match side {
            Side::Player => (&self.player, &self.ai, &branch.state.player, &branch.state.ai),
            Side::Ai => (&self.ai, &self.player, &branch.state.ai, &branch.state.player),
        };
        if matches!(mv.effect, Effect::TwoToFiveAttacks) && target_state.rage_accuracy.is_some() {
            let single_hits = attacker.single_hit_outcomes(target, attacker_state, target_state, mv);
            for &(hits, count_prob) in mv.effect.hit_count_outcomes() {
                for &(damage, roll_prob) in &single_hits {
                    let alternative = branch.clone();
                    self.resolve_damage(alternative, probability * count_prob * roll_prob, damage.saturating_mul(hits), hits);
                }
            }
            return;
        }
        let rolls = branch.turn.damage_outcomes(&branch.state, &self.player, &self.ai);
        let target_hp = target_state.hp;
        let (&(max_dmg, max_prob), lower_rolls) = rolls.split_last().expect("A damaging move must produce at least one damage outcome");
        let (alternatives, last_dmg, last_prob) = if max_dmg <= target_hp {
            (lower_rolls, max_dmg, max_prob)
        } else {
            let (nonlethal, lethal) = rolls.split_at(rolls.partition_point(|&(damage, _)| damage < target_hp));
            (nonlethal, target_hp, lethal.iter().map(|&(_, roll_prob)| roll_prob).sum::<f64>())
        };
        let nonlethal = if max_dmg < target_hp { rolls.as_ref() } else { alternatives };
        if self.batch_turns
            && matches!(mv.effect, Effect::NoEffect | Effect::ConfusionSide)
            && let [(min_dmg, _), _, ..] = nonlethal
            && *min_dmg > 0
            && branch.state.can_share_damage_rolls(side)
        {
            let outcomes = nonlethal.iter().map(|&(dmg, roll_prob)| (dmg - *min_dmg, probability * roll_prob));
            if max_dmg < target_hp {
                self.resolve_damage_outcomes(branch, outcomes, *min_dmg, 1);
                return;
            }
            self.resolve_damage_outcomes(branch.clone(), outcomes, *min_dmg, 1);
        } else {
            for &(dmg, roll_prob) in alternatives {
                self.resolve_damage(branch.clone(), probability * roll_prob, dmg, 1);
            }
        }
        self.resolve_damage(branch, probability * last_prob, last_dmg, 1);
    }

    fn resolve_pending_actions(&mut self) {
        if self.next_actions.is_empty() {
            return;
        }
        let mut pending_actions = mem::take(&mut self.next_actions);
        #[expect(clippy::iter_over_hash_type, reason = "Order is not important here")]
        for (pending, probability) in pending_actions.drain() {
            self.resolve_action(pending.into_branch(), probability);
        }
        self.next_actions = pending_actions;
    }

    fn resolve_turn_order(&mut self, mut branch: Branch, mut probability: f64) {
        if let Some(side) = branch.turn.resolve_order(&branch.state, &self.player, &self.ai) {
            branch.turn.side = Some(side);
        } else {
            probability *= 0.5f64;
            let mut alternative = branch.clone();
            alternative.turn.side = Some(Side::Ai);
            self.resolve_action(alternative, probability);
            branch.turn.side = Some(Side::Player);
        }
        self.resolve_action(branch, probability);
    }

    fn start_state_turn(&mut self, state_key: &BattleKey, hp_pair: u32, probability: f64, second_turn: bool) {
        if probability == 0.0f64 {
            return;
        }
        let state = state_key.to_state_with_hp(hp_pair);
        let player_move = if state.player.is_asleep() {
            None
        } else if let Some(mv) = state.player.trapping_move() {
            Some((None, mv))
        } else if state.is_biding(Side::Player) {
            Some((None, &BIDE))
        } else if let Some(slot) = state
            .player
            .recharge_slot
            .or(state.player.charging_slot)
            .or_else(|| state.player.rampage.as_ref().map(|rampage| rampage.slot))
        {
            let idx = usize::from(slot);
            Some((Some(idx), self.player.pokemon.moves[idx].expect("A locked move must still be known")))
        } else if state.player.rage_accuracy.is_some() {
            Some((None, &RAGE))
        } else {
            let has_ohko_speed = self.player_has_ohko && self.player.has_ohko_speed(&self.ai, &state.player, &state.ai);
            let selection_key = state.player_selection_key(has_ohko_speed);
            Some(*self.player_choices.entry(selection_key).or_insert_with(|| {
                self.player_policy
                    .select(&self.player.pokemon.moves, state.player_pp, state.player.disabled_slot(), |mv| {
                        self.player.expected_damage(&self.ai, &state.player, &state.ai, mv)
                    })
            }))
        };
        let ai_action = if state.ai.is_asleep() {
            TurnAction::Skip
        } else if let Some(mv) = state.ai.trapping_move() {
            TurnAction::Move(None, mv)
        } else if state.is_biding(Side::Ai) {
            TurnAction::Move(None, &BIDE)
        } else if let Some(copied_slot) = state.ai_copied_move {
            TurnAction::Move(None, self.player_move_at(copied_slot))
        } else if let Some(slot) = state
            .ai
            .recharge_slot
            .or(state.ai.charging_slot)
            .or_else(|| state.ai.rampage.as_ref().map(|rampage| rampage.slot))
        {
            let idx = usize::from(slot);
            TurnAction::Move(Some(slot), self.ai.pokemon.moves[idx].expect("A locked move must still be known"))
        } else if state.ai.rage_accuracy.is_some() {
            TurnAction::Move(None, &RAGE)
        } else {
            let key = state.ai.disabled_slot().map_or(0, |slot| slot + 1) * 2
                + u8::from(state.player.has_status_condition())
                + if second_turn { 10 } else { 0 };
            TurnAction::AiChoice(key, 0)
        };
        let turn_branch = Branch::new(state, TurnContext::new([Some(TurnAction::new(player_move)), Some(ai_action)], None));
        self.begin_turn(turn_branch, probability);
    }
}

#[derive(Clone)]
pub struct BattleOutcome {
    pub win: f64,
    pub lose: f64,
    pub unresolved: f64,
}

impl Default for BattleOutcome {
    fn default() -> Self {
        Self {
            win: 0.0,
            lose: 0.0,
            unresolved: 1.0,
        }
    }
}

impl BattleOutcome {
    fn record(&mut self, winner: Side, probability: f64) {
        match winner {
            Side::Player => self.win += probability,
            Side::Ai => self.lose += probability,
        }
    }
}
