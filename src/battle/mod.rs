mod combatant;
pub mod state;
mod turn;

use core::{
    array,
    mem::{self, take},
};

use crate::{
    battle::{
        combatant::Combatant,
        state::{
            BattleState,
            key::{BattleKey, StateMap},
        },
        turn::{Branch, TurnAction, TurnContext},
    },
    data::{
        effects::{Effect, EffectResolution},
        move_data::Move,
        moves::BIDE,
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
    states: StateMap<BattleKey>,
    branches: StateMap<Branch>,
    next_actions: StateMap<Branch>,
    player: Combatant<'battle>,
    player_policy: &'battle PlayerPolicy,
    ai: Combatant<'battle>,
    ai_choices: [AiChoices; 20],
    ai_item: Option<&'battle TrainerItem>,
    is_trainer: bool,
    remaining_turns: usize,
    outcome: BattleOutcome,
}

impl<'battle> Battle<'battle> {
    pub(super) fn new(player: &'battle Player, ai: &'battle Pokemon, matchup: &'battle MatchupGroup) -> Self {
        let player_combatant = Combatant::new(&player.active, player.stats());
        let ai_combatant = Combatant::new(ai, ai.stats());
        Self {
            states: StateMap::default(),
            branches: StateMap::default(),
            next_actions: StateMap::default(),
            player: player_combatant,
            player_policy: &player.policy,
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
            is_trainer: !matches!(matchup.kind, MatchupKind::Wild),
            remaining_turns: 0,
            outcome: BattleOutcome::default(),
        }
    }

    pub(super) fn run(&mut self, max_depth: usize) -> BattleOutcome {
        self.states.clear();
        self.branches.clear();
        self.next_actions.clear();
        self.remaining_turns = max_depth;
        self.outcome = BattleOutcome::default();
        self.outcome.unresolved = 0.0f64;
        let mut init_state = BattleState::new(&self.player, &self.ai, self.ai_item.map_or(0, |item| item.uses));
        init_state.cap_surplus_pp(self.remaining_turns);
        self.states.insert(init_state.key(), 1.0f64);
        let mut current_states = StateMap::default();
        let mut current_branches = take(&mut self.branches);
        let mut batch_turns = false;
        for idx in 0..max_depth {
            if self.states.is_empty() {
                break;
            }
            self.remaining_turns -= 1;
            mem::swap(&mut current_states, &mut self.states);
            let second_turn = idx % 256 == 1;
            batch_turns |= current_states.len() >= 4_096;
            if !batch_turns {
                #[expect(clippy::iter_over_hash_type, reason = "Order is not important here")]
                for (state_key, probability) in current_states.drain() {
                    self.start_state_turn(&state_key, probability, second_turn);
                    self.resolve_pending_actions(&mut current_branches);
                }
                continue;
            }
            let mut ordered_states: Vec<_> = current_states
                .iter()
                .map(|(state_key, &probability)| (state_key.action_batch_order(), state_key, probability))
                .collect();
            ordered_states.sort_unstable_by_key(|entry| entry.0);
            for batch in ordered_states.chunks(512) {
                for &(_, state_key, probabillity) in batch {
                    self.start_state_turn(state_key, probabillity, second_turn);
                }
                self.resolve_pending_actions(&mut current_branches);
            }
            current_states.clear();
        }
        self.branches = current_branches;
        self.outcome.unresolved += self.states.values().copied().fold(0.0f64, |total, prob| total + prob);
        self.states.clear();
        self.outcome.clone()
    }

    fn start_state_turn(&mut self, state_key: &BattleKey, probability: f64, second_turn: bool) {
        if probability == 0.0f64 {
            return;
        }
        let state = state_key.to_state();
        let player_move = if state.player.is_asleep() {
            None
        } else if let Some(mv) = state.player.trapping_move() {
            Some((None, mv))
        } else if state.is_biding(Side::Player) {
            Some((None, &BIDE))
        } else {
            Some(
                self.player_policy
                    .select(&self.player.pokemon.moves, state.player_pp, state.player.disabled_slot(), |mv| {
                        self.player.expected_damage(&self.ai, &state.player, &state.ai, mv)
                    }),
            )
        };
        let ai_action = if state.ai.is_asleep() {
            TurnAction::Skip
        } else if let Some(mv) = state.ai.trapping_move() {
            TurnAction::Move(None, mv)
        } else if state.is_biding(Side::Ai) {
            TurnAction::Move(None, &BIDE)
        } else {
            let key = state.ai.disabled_slot().map_or(0, |slot| slot + 1) * 2
                + u8::from(state.player.has_status_condition())
                + if second_turn { 10 } else { 0 };
            TurnAction::AiChoice(key, 0)
        };
        let turn_branch = Branch::new(state, TurnContext::new([Some(TurnAction::new(player_move)), Some(ai_action)], None));
        self.begin_turn(turn_branch, probability);
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
                alternative.turn.actions[1] = Some(TurnAction::Move(slot, mv));
                self.begin_action(alternative, probability * choice_prob);
            }
            let ((slot, mv), choice_prob) = self.ai_choices[choice_idx].moves[priority_idx][last];
            branch.turn.actions[1] = Some(TurnAction::Move(slot, mv));
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
        branch.state.begin_move(side, slot, &mv.effect);
        let resolution = mv.effect.resolution();
        if self.is_trainer && matches!(resolution, EffectResolution::SwitchOrEscape) {
            self.finish_action(branch, probability);
            return;
        }
        let (target, target_types) = match side {
            Side::Player => (&branch.state.ai, &self.ai.pokemon.species.types),
            Side::Ai => (&branch.state.player, &self.player.pokemon.species.types),
        };
        if matches!(resolution, EffectResolution::Disable) && !target.can_be_disabled()
            || matches!(resolution, EffectResolution::Sleep) && !target.can_be_put_to_sleep()
            || matches!(mv.effect, Effect::Paralyze) && !target.can_be_paralyzed()
        {
            self.finish_action(branch, probability);
            return;
        }
        let hit_prob = branch.turn.hit_probability(&branch.state, target_types);
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
            EffectResolution::Damage => self.resolve_damage_rolls(branch, probability),
            EffectResolution::Disable => {
                let (target_side, occupied_slots, pp) = match side {
                    Side::Player => (Side::Ai, self.ai.pokemon.moves.map(|known_mv| known_mv.is_some()), None),
                    Side::Ai => (
                        Side::Player,
                        self.player.pokemon.moves.map(|known_mv| known_mv.is_some()),
                        Some(&branch.state.player_pp),
                    ),
                };
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
            EffectResolution::Sleep => {
                let target_side = match side {
                    Side::Player => Side::Ai,
                    Side::Ai => Side::Player,
                };
                branch.state.combatant_mut(target_side).fall_asleep();
                self.finish_action(branch, probability);
            }
            EffectResolution::Status => {
                branch.state.apply_status_move(&mv.effect, side, target_types);
                self.finish_action(branch, probability);
            }
            EffectResolution::CopyMove | EffectResolution::FixedDamage | EffectResolution::SwitchOrEscape => todo!(),
        }
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

    fn finish_action(&mut self, mut branch: Branch, probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        branch.state.clear_unused_damage();
        if let Some(winner) = branch.state.winner() {
            self.outcome.record(winner, probability);
            return;
        }
        let side = branch.turn.side.expect("Finishing an action requires an acting side");
        let (max_hp, opponent_max_hp) = match side {
            Side::Player => (self.player.stats.hp, self.ai.stats.hp),
            Side::Ai => (self.ai.stats.hp, self.player.stats.hp),
        };
        branch.state.apply_residual_damage(side, max_hp, opponent_max_hp);
        if let Some(winner) = branch.state.winner() {
            self.outcome.record(winner, probability);
            return;
        }
        branch.turn.finish_action();
        if branch.turn.actions.iter().any(Option::is_some) {
            *self.next_actions.entry(branch).or_insert(0.0f64) += probability;
            return;
        }
        branch.state.finish_turn();
        if self.remaining_turns == 0 {
            self.outcome.unresolved += probability;
            return;
        }
        branch.state.cap_surplus_pp(self.remaining_turns);
        *self.states.entry(branch.state.key()).or_insert(0.0f64) += probability;
    }

    fn resolve_action(&mut self, mut branch: Branch, mut probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let side = branch.turn.side.expect("Executing an action requires an acting side");
        if side == Side::Ai
            && let Some(item) = self.ai_item
        {
            let item_prob = item.use_probability(branch.state.ai_items_left);
            if item_prob > 0.0f64 {
                let mut using_item = branch.clone();
                using_item.state.use_ai_item(item);
                self.finish_action(using_item, probability * item_prob);
                probability *= 1.0f64 - item_prob;
                if probability == 0.0f64 {
                    return;
                }
            }
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
        if branch.state.is_trapped(side) || branch.turn.skips_action() {
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
                let combatant = match side {
                    Side::Player => &self.player,
                    Side::Ai => &self.ai,
                };
                let damage = combatant.confusion_damage(self_hit.state.combatant(side));
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

    fn resolve_damage(&mut self, mut branch: Branch, mut probability: f64, dmg: u16) {
        if probability == 0.0f64 {
            return;
        }
        let side = branch.turn.side.expect("Applying damage requires an acting side");
        let (_, mv) = branch.turn.selected_move();
        let dmg_dealt = branch.state.apply_damage(dmg, mv, side);
        let max_hp = match side {
            Side::Player => self.player.stats.hp,
            Side::Ai => self.ai.stats.hp,
        };
        branch.state.combatant_mut(side).heal(mv.effect.drained_hp(dmg_dealt), max_hp);
        if matches!(mv.effect, Effect::Trapping) && branch.state.winner().is_none() {
            branch.state.start_trapping(side, mv, dmg);
            self.finish_action(branch, probability);
            return;
        }
        let (target, target_types) = match side {
            Side::Player => (&branch.state.ai, &self.ai.pokemon.species.types),
            Side::Ai => (&branch.state.player, &self.player.pokemon.species.types),
        };
        let secondary_prob = if matches!(mv.effect, Effect::Flinch1) && !branch.turn.can_flinch_target() {
            0.0f64
        } else {
            mv.effect.secondary_probability(target, target_types, &mv.mv_type, side == Side::Ai)
        };
        if secondary_prob > 0.0f64 {
            let mut affected = branch.clone();
            if matches!(mv.effect, Effect::Flinch1) {
                affected.turn.flinch_target();
            } else {
                affected.state.apply_secondary(&mv.effect, side);
            }
            self.finish_action(affected, probability * secondary_prob);
            probability *= 1.0f64 - secondary_prob;
        }
        self.finish_action(branch, probability);
    }

    fn resolve_damage_rolls(&mut self, branch: Branch, probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let cached_rolls = branch.turn.damage_outcomes(&branch.state, &self.player, &self.ai);
        let mut single_hit_rolls = [(0, 0.0f64); 78];
        let mut extended_rolls;
        let rolls: &mut [(u16, f64)] = if cached_rolls.len() <= single_hit_rolls.len() {
            &mut single_hit_rolls
        } else {
            extended_rolls = vec![(0, 0.0f64); cached_rolls.len()];
            &mut extended_rolls
        };
        let roll_count = {
            let target_hp = match branch.turn.side.expect("Damage resolution requires an acting side") {
                Side::Player => branch.state.ai.hp,
                Side::Ai => branch.state.player.hp,
            };
            let mut count = 0;
            for &(damage, roll_prob) in cached_rolls {
                let dmg_dealt = damage.min(target_hp);
                if count > 0 && rolls[count - 1].0 == dmg_dealt {
                    rolls[count - 1].1 += roll_prob;
                } else {
                    rolls[count] = (dmg_dealt, roll_prob);
                    count += 1;
                }
            }
            count
        };
        let (&(last_dmg, last_prob), alternatives) = rolls[..roll_count]
            .split_last()
            .expect("A damaging move must produce at least once damage outcome");
        for &(dmg, roll_prob) in alternatives {
            let alternative = branch.clone();
            self.resolve_damage(alternative, probability * roll_prob, dmg);
        }
        self.resolve_damage(branch, probability * last_prob, last_dmg);
    }

    fn resolve_pending_actions(&mut self, current_branches: &mut StateMap<Branch>) {
        while !self.next_actions.is_empty() {
            mem::swap(current_branches, &mut self.next_actions);
            #[expect(clippy::iter_over_hash_type, reason = "Order is not important here")]
            for (branch, probability) in current_branches.drain() {
                self.resolve_action(branch, probability);
            }
        }
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

type AiChoice = ((Option<usize>, &'static Move), f64);

struct AiChoices {
    moves: [Vec<AiChoice>; 2],
    probabilities: [f64; 2],
}

impl AiChoices {
    fn new(choices: Vec<AiChoice>) -> Self {
        let mut result = Self {
            moves: array::from_fn(|_| Vec::new()),
            probabilities: [0.0f64; 2],
        };
        for choice in choices {
            let ((_, mv), probability) = choice;
            let priority = usize::from(mv.priority());
            result.probabilities[priority] += probability;
            result.moves[priority].push(choice);
        }
        for (moves, probability) in result.moves.iter_mut().zip(result.probabilities) {
            for (_, choice_prob) in moves {
                *choice_prob /= probability;
            }
        }
        result
    }
}
