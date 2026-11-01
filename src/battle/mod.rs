pub mod state;
mod turn;

use core::{
    array,
    cell::OnceCell,
    mem::{self, take},
};
use std::collections::HashMap;

use crate::{
    battle::{
        state::{BattleState, CombatantState, StatStages},
        turn::{Branch, TurnAction, TurnContext},
    },
    data::{
        Move,
        effects::{Effect, EffectResolution},
        moves::{self, BIDE, STRUGGLE},
    },
    journey::{GameAi, PlayerPolicy},
    player::Player,
    pokemon::{Pokemon, Stats},
};

type AiChoice = ((Option<usize>, &'static Move), f64);
type StagedDamageRolls = [[OnceCell<Vec<(u16, f64)>>; 13]; 13];

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Side {
    Ai,
    Player,
}

pub struct Battle<'battle> {
    states: HashMap<BattleState, f64>,
    branches: HashMap<Branch, f64>,
    next_actions: HashMap<Branch, f64>,
    player: Combatant<'battle>,
    player_policy: &'battle PlayerPolicy,
    ai: Combatant<'battle>,
    ai_choices: [Vec<AiChoice>; 10],
    remaining_turns: usize,
    outcome: BattleOutcome,
}

impl<'battle> Battle<'battle> {
    pub(super) fn new(player: &'battle Player, player_policy: &'battle PlayerPolicy, ai: &'battle Pokemon, game_ai: &GameAi) -> Self {
        let player_combatant = Combatant::new(&player.active, player.stats());
        let ai_combatant = Combatant::new(ai, ai.stats());
        Self {
            states: HashMap::new(),
            branches: HashMap::new(),
            next_actions: HashMap::new(),
            player: player_combatant,
            player_policy,
            ai: ai_combatant,
            ai_choices: array::from_fn(|key| {
                let disabled_slot = (key / 2).checked_sub(1).map(|slot| slot as u8);
                game_ai.select(&ai.moves, disabled_slot, key % 2 != 0)
            }),
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
        let mut init_state = BattleState::new(&self.player, &self.ai);
        init_state.cap_surplus_pp(self.remaining_turns);
        self.states.insert(init_state, 1.0f64);
        let mut current_states = HashMap::new();
        let mut current_branches = take(&mut self.branches);
        for _ in 0..max_depth {
            if self.states.is_empty() {
                break;
            }
            self.remaining_turns -= 1;
            mem::swap(&mut current_states, &mut self.states);
            #[expect(clippy::iter_over_hash_type, reason = "Order is not important here")]
            for (state, probability) in current_states.drain() {
                if probability == 0.0f64 {
                    continue;
                }
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
                    let key = state.ai.disabled_slot().map_or(0, |slot| slot + 1) * 2 + u8::from(state.player.has_status_condition());
                    TurnAction::AiChoice(key)
                };
                let turn_branch = Branch::new(state, TurnContext::new([Some(TurnAction::new(player_move)), Some(ai_action)], None));
                self.begin_turn(turn_branch, probability);
                while !self.next_actions.is_empty() {
                    mem::swap(&mut current_branches, &mut self.next_actions);
                    #[expect(clippy::iter_over_hash_type, reason = "Order is not important here")]
                    for (branch, branch_prob) in current_branches.drain() {
                        self.resolve_action(branch, branch_prob);
                    }
                }
            }
        }
        self.branches = current_branches;
        self.outcome.unresolved += self.states.values().copied().fold(0.0f64, |total, prob| total + prob);
        self.states.clear();
        self.outcome.clone()
    }

    fn begin_action(&mut self, mut branch: Branch, mut probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let side = branch.turn.side.expect("Beginning an action requires an acting side");
        if let Some(TurnAction::AiChoice(key)) = branch.turn.actions[1].as_ref()
            && side == Side::Ai
        {
            let choice_idx = usize::from(*key);
            let last = self.ai_choices[choice_idx].len() - 1;
            for idx in 0..last {
                let ((slot, mv), choice_prob) = self.ai_choices[choice_idx][idx];
                let mut alternative = branch.clone();
                alternative.turn.actions[1] = Some(TurnAction::Move(slot, mv));
                self.begin_action(alternative, probability * choice_prob);
            }
            let ((slot, mv), choice_prob) = self.ai_choices[choice_idx][last];
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
        let hit_prob = branch.turn.hit_probability(&branch.state);
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
        match mv.effect.resolution() {
            EffectResolution::Bide => {
                let [(short, short_prob), (long, long_prob)] = mv.effect.bide_outcomes();
                let mut alternative = branch.clone();
                alternative.state.start_bide(side, short);
                self.finish_action(alternative, probability * short_prob);
                branch.state.start_bide(side, long);
                self.finish_action(branch, probability * long_prob);
            }
            EffectResolution::Damage => {
                let crit_prob = branch.turn.crit_probability(&self.player, &self.ai);
                #[expect(clippy::float_cmp, reason = "Critical-hit probabilities are exact multiples of 1/256")]
                if crit_prob == 1.0f64 {
                    self.resolve_damage_rolls(branch, probability, true);
                } else {
                    if crit_prob > 0.0f64 {
                        let critical = branch.clone();
                        self.resolve_damage_rolls(critical, probability * crit_prob, true);
                        probability *= 1.0f64 - crit_prob;
                    }
                    self.resolve_damage_rolls(branch, probability, false);
                }
            }
            EffectResolution::Disable => {
                let (target_side, occupied_slots, pp) = match side {
                    Side::Player => (Side::Ai, self.ai.pokemon.moves.map(|known_mv| known_mv.is_some()), None),
                    Side::Ai => (
                        Side::Player,
                        self.player.pokemon.moves.map(|known_mv| known_mv.is_some()),
                        Some(&branch.state.player_pp),
                    ),
                };
                if !branch.state.combatant(target_side).can_be_disabled() {
                    self.finish_action(branch, probability);
                    return;
                }
                let mut outcomes = mv.effect.disable_outcomes(occupied_slots, pp);
                let Some((disabled, slot_prob)) = outcomes.pop() else {
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
                if !branch.state.combatant(target_side).can_be_put_to_sleep() {
                    self.finish_action(branch, probability);
                    return;
                }
                branch.state.combatant_mut(target_side).fall_asleep();
                self.finish_action(branch, probability);
            }
            EffectResolution::Status => {
                let target_types = match side {
                    Side::Player => &self.ai.pokemon.species.types,
                    Side::Ai => &self.player.pokemon.species.types,
                };
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
        *self.states.entry(branch.state).or_insert(0.0f64) += probability;
    }

    fn resolve_action(&mut self, mut branch: Branch, mut probability: f64) {
        if probability == 0.0f64 {
            return;
        }
        let side = branch.turn.side.expect("Executing a move requires an acting side");
        if let Some(wake_prob) = branch.state.combatant_mut(side).advance_sleep() {
            if wake_prob < 1.0f64 {
                let sleeping = branch.clone();
                self.finish_action(sleeping, probability * (1.0 - wake_prob));
            }
            branch.state.combatant_mut(side).wake_up();
            self.finish_action(branch, probability * wake_prob);
            return;
        }
        if branch.state.is_trapped(side) {
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
                let damage = combatant.confusion_damage(&self_hit.state.combatant(side).stages);
                self_hit.state.hurt_in_confusion(side, damage);
                self.finish_action(self_hit, continuing_prob);
                if expiry_prob == 0.0f64 {
                    self.begin_action(branch.clone(), continuing_prob);
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
        let secondary_prob = mv.effect.secondary_probability(target, target_types, side == Side::Ai);
        if secondary_prob > 0.0f64 {
            let mut affected = branch.clone();
            affected.state.apply_secondary(&mv.effect, side);
            self.finish_action(affected, probability * secondary_prob);
            probability *= 1.0f64 - secondary_prob;
        }
        self.finish_action(branch, probability);
    }

    fn resolve_damage_rolls(&mut self, branch: Branch, probability: f64, crit: bool) {
        if probability == 0.0f64 {
            return;
        }
        let mut rolls = [(0, 0.0f64); 39];
        let roll_count = {
            let cached_rolls = branch.turn.damage_rolls(&branch.state, &self.player, &self.ai, crit);
            rolls[..cached_rolls.len()].copy_from_slice(cached_rolls);
            cached_rolls.len()
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

pub struct Combatant<'battle> {
    pokemon: &'battle Pokemon,
    stats: Stats,
    damage_rolls: HashMap<&'static Move, DamageRollCache>,
}

impl<'battle> Combatant<'battle> {
    fn new(pokemon: &'battle Pokemon, stats: Stats) -> Self {
        Self {
            pokemon,
            stats,
            damage_rolls: pokemon
                .moves
                .into_iter()
                .flatten()
                .chain([&STRUGGLE])
                .map(|mv| (mv, DamageRollCache::new()))
                .collect(),
        }
    }

    fn base_damage(&self, power: u8, mut attack: u32, mut defense: u32, crit: bool) -> u32 {
        if attack >= 256 || defense >= 256 {
            attack = (attack / 4 % 256).max(1);
            defense = defense / 4 % 256;
        }
        let level = u32::from(self.pokemon.level) * if crit { 2 } else { 1 };
        let damage = ((2 * level / 5 + 2) * u32::from(power) * attack)
            .checked_div(defense)
            .expect("The game does not account for 0 defense and crashes")
            / 50;
        damage.min(997) + 2
    }

    fn crit_probability(&self, mv: &Move) -> f64 {
        let mut threshold = u16::from(self.pokemon.base_stats().spd / 2);
        // wrong place for this to be being managed
        if mv == &moves::RAZOR_LEAF || mv == &moves::SLASH {
            threshold = (threshold * 8).min(255);
        }
        f64::from(threshold) / 256.0
    }

    fn calculate_damage_rolls(&self, target: &Self, stages: &StatStages, target_stages: &StatStages, mv: &Move, crit: bool) -> Vec<(u16, f64)> {
        let (attack, defense) = if mv.mv_type.is_special() {
            (u32::from(self.stats.spc), u32::from(target.stats.spc))
        } else if crit {
            (u32::from(self.stats.atk), u32::from(target.stats.def))
        } else {
            (
                StatStages::apply(self.stats.atk, stages.atk),
                StatStages::apply(target.stats.def, target_stages.def),
            )
        };
        let mut damage = self.base_damage(mv.power, attack, defense, crit);
        if self.pokemon.species.types.contains(&mv.mv_type) {
            damage += damage / 2;
        }
        let types = &target.pokemon.species.types;
        damage = damage * mv.mv_type.effectiveness(&types[0]) / 10;
        if types[1] != types[0] {
            damage = damage * mv.mv_type.effectiveness(&types[1]) / 10;
        }
        if damage <= 1 {
            return vec![(damage as u16, 1.0)];
        }
        let mut rolls: Vec<(u16, u8)> = Vec::new();
        for roll in 217..=255 {
            let rolled_damage = (damage * roll / 255) as u16;
            if let Some((previous_damage, count)) = rolls.last_mut()
                && *previous_damage == rolled_damage
            {
                *count += 1u8;
                continue;
            }
            rolls.push((rolled_damage, 1u8));
        }
        rolls.into_iter().map(|(dmg, count)| (dmg, f64::from(count) / 39.0f64)).collect()
    }

    fn confusion_damage(&self, stages: &StatStages) -> u16 {
        self.base_damage(
            40,
            StatStages::apply(self.stats.atk, stages.atk),
            StatStages::apply(self.stats.def, stages.def),
            false,
        ) as u16
    }

    fn damage_rolls(&self, target: &Self, stages: &StatStages, target_stages: &StatStages, mv: &Move, crit: bool) -> &[(u16, f64)] {
        let cache = self.damage_rolls.get(mv).expect("A selected move must have a damage-roll cache");
        let rolls = if crit {
            &cache.critical
        } else if mv.mv_type.is_special() {
            &cache.normal[6][6]
        } else {
            &cache.normal[(stages.atk + 6) as usize][(target_stages.def + 6) as usize]
        };
        rolls.get_or_init(|| self.calculate_damage_rolls(target, stages, target_stages, mv, crit))
    }

    fn expected_damage(&self, target: &Self, state: &CombatantState, target_state: &CombatantState, mv: &Move) -> f64 {
        let hit_probability = state.stages.hit_probability(mv);
        if matches!(mv.effect, Effect::LeechSeed) {
            if !target_state.can_be_seeded(&target.pokemon.species.types) {
                return 0.0;
            }
            return f64::from(CombatantState::residual_damage(target.stats.hp).min(target_state.hp)) * hit_probability;
        }
        if mv.power == 0 {
            return 0.0;
        }
        let crit_probability = self.crit_probability(mv);
        let mut expected_damage = 0.0f64;
        for (crit, probability) in [(false, 1.0f64 - crit_probability), (true, crit_probability)] {
            for &(damage, roll_probability) in self.damage_rolls(target, &state.stages, &target_state.stages, mv, crit) {
                expected_damage += f64::from(damage.min(target_state.hp)) * probability * roll_probability;
            }
        }
        expected_damage * hit_probability
    }
}

struct DamageRollCache {
    normal: StagedDamageRolls,
    critical: OnceCell<Vec<(u16, f64)>>,
}

impl DamageRollCache {
    fn new() -> Self {
        Self {
            normal: array::from_fn(|_| array::from_fn(|_| OnceCell::new())),
            critical: OnceCell::new(),
        }
    }
}
