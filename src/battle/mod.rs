pub mod state;

use core::{cmp::Ordering, mem::take};
use std::collections::HashMap;

use crate::{
    battle::state::{BattleState, CombatantState, StatStages},
    data::{
        Move,
        effects::{Effect, EffectResolution},
        moves::{self, BIDE},
    },
    matchup::{GameAi, PlayerPolicy},
    player::Player,
    pokemon::{Pokemon, Stats},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum Side {
    Ai,
    Player,
}

pub struct Battle<'battle> {
    branches: HashMap<Branch, f64>,
    player: Combatant<'battle>,
    player_policy: &'battle PlayerPolicy,
    ai: Combatant<'battle>,
    game_ai: &'battle GameAi,
    outcome: BattleOutcome,
}

impl<'battle> Battle<'battle> {
    pub fn new(player: &'battle Player, player_policy: &'battle PlayerPolicy, ai: &'battle Pokemon, game_ai: &'battle GameAi) -> Self {
        let player_combatant = Combatant::new(&player.active, player.stats());
        let ai_combatant = Combatant::new(ai, ai.stats());
        Self {
            branches: HashMap::new(),
            player: player_combatant,
            player_policy,
            ai: ai_combatant,
            game_ai,
            outcome: BattleOutcome::default(),
        }
    }

    pub fn run(&mut self, max_depth: usize) -> BattleOutcome {
        self.branches.clear();
        self.outcome = BattleOutcome::default();
        let state = BattleState::new(&self.player, &self.ai);
        self.start_turn(state, 1.0);
        for _ in 0..max_depth {
            if self.branches.is_empty() {
                break;
            }
            #[expect(clippy::iter_over_hash_type, reason = "Order is not important here")]
            for (mut branch, mut probability) in take(&mut self.branches) {
                if probability == 0.0f64 {
                    continue;
                }
                match branch.continuation {
                    Continuation::MoveChosen => {
                        branch.continuation = Continuation::OrderDetermined;
                        if let Some(side) = branch.turn.resolve_order(&branch.state, &self.player, &self.ai) {
                            branch.turn.side = Some(side);
                        } else {
                            probability *= 0.5f64;
                            let mut alternative = branch.clone();
                            alternative.turn.side = Some(Side::Ai);
                            *self.branches.entry(alternative).or_insert(0.0f64) += probability;
                            branch.turn.side = Some(Side::Player);
                        }
                        *self.branches.entry(branch).or_insert(0.0f64) += probability;
                    }
                    Continuation::OrderDetermined => {
                        let side = branch.turn.side.expect("Executing a move requires an acting side");
                        if let Some(wake_prob) = branch.state.combatant_mut(side).advance_sleep() {
                            if wake_prob < 1.0f64 {
                                let sleeping = branch.clone();
                                self.finish_action(sleeping, probability * (1.0 - wake_prob));
                            }
                            branch.state.combatant_mut(side).wake_up();
                            self.finish_action(branch, probability * wake_prob);
                            continue;
                        }
                        if branch.state.is_trapped(side) {
                            self.finish_action(branch, probability);
                            continue;
                        }
                        if let Some(expiry_prob) = branch.state.combatant_mut(side).advance_disable() {
                            if expiry_prob < 1.0f64 {
                                let disabled = branch.clone();
                                self.begin_action(disabled, probability * (1.0 - expiry_prob));
                            }
                            branch.state.combatant_mut(side).end_disable();
                            probability *= expiry_prob;
                        }
                        self.begin_action(branch, probability);
                    }
                    Continuation::AccChecked { hit } => {
                        if !hit {
                            branch.state.miss_move();
                            self.finish_action(branch, probability);
                            continue;
                        }
                        let (_, mv) = branch.turn.selected_move();
                        match mv.effect.resolution() {
                            EffectResolution::Bide => {
                                let side = branch.turn.side.expect("Starting Bide requires an acting side");
                                let [(short, short_probability), (long, long_probability)] = mv.effect.bide_outcomes();
                                let mut alternative = branch.clone();
                                alternative.state.start_bide(side, short);
                                self.finish_action(alternative, probability * short_probability);
                                branch.state.start_bide(side, long);
                                self.finish_action(branch, probability * long_probability);
                            }
                            EffectResolution::Damage => {
                                let crit_probability = branch.turn.crit_probability(&self.player, &self.ai);
                                branch.continuation = Continuation::CritChecked { crit: false };
                                #[expect(clippy::float_cmp, reason = "crit_probability created by dividing by 256.0 which does require rounding")]
                                #[expect(clippy::else_if_without_else, reason = "Done that way to create branching")]
                                if crit_probability == 1.0f64 {
                                    branch.continuation = Continuation::CritChecked { crit: true };
                                } else if crit_probability > 0.0f64 {
                                    let critical = Branch::new(branch.state.clone(), branch.turn.clone(), Continuation::CritChecked { crit: true });
                                    *self.branches.entry(critical).or_insert(0.0f64) += probability * crit_probability;
                                    probability *= 1.0f64 - crit_probability;
                                }
                                *self.branches.entry(branch).or_insert(0.0f64) += probability;
                            }
                            EffectResolution::Disable => {
                                let (target_side, occupied_slots, pp) = match branch.turn.side.expect("Disable requires an acting side") {
                                    Side::Player => (Side::Ai, self.ai.pokemon.moves.map(|slot| slot.is_some()), None),
                                    Side::Ai => (
                                        Side::Player,
                                        self.player.pokemon.moves.map(|slot| slot.is_some()),
                                        Some(&branch.state.player_pp),
                                    ),
                                };
                                if !branch.state.combatant(target_side).can_be_disabled() {
                                    self.finish_action(branch, probability);
                                    continue;
                                }
                                let mut outcomes = mv.effect.disable_outcomes(occupied_slots, pp);
                                let Some((disabled, slot_prob)) = outcomes.pop() else {
                                    self.finish_action(branch, probability);
                                    continue;
                                };
                                for (disable_state, alt_probability) in outcomes {
                                    let mut alternative = branch.clone();
                                    alternative.state.combatant_mut(target_side).disable(disable_state);
                                    self.finish_action(alternative, probability * alt_probability);
                                }
                                branch.state.combatant_mut(target_side).disable(disabled);
                                self.finish_action(branch, probability * slot_prob);
                            }
                            EffectResolution::Sleep => {
                                let target_side = match branch.turn.side.expect("Sleep requires an acting side") {
                                    Side::Player => Side::Ai,
                                    Side::Ai => Side::Player,
                                };
                                if !branch.state.combatant(target_side).can_be_put_to_sleep() {
                                    self.finish_action(branch, probability);
                                    continue;
                                }
                                branch.state.combatant_mut(target_side).fall_asleep();
                                self.finish_action(branch, probability);
                            }
                            EffectResolution::Status => {
                                let side = branch.turn.side.expect("A successful move must have an acting side");
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
                    Continuation::CritChecked { crit } => {
                        let rolls = branch.turn.damage_rolls(&branch.state, &self.player, &self.ai, crit);
                        let (&(last_dmg, last_probability), alternatives) =
                            rolls.split_last().expect("A damaging move must produce at least one damage outcome");
                        for &(dmg, roll_probability) in alternatives {
                            let alternative = Branch::new(branch.state.clone(), branch.turn.clone(), Continuation::DmgRolled { dmg });
                            *self.branches.entry(alternative).or_insert(0.0f64) += probability * roll_probability;
                        }
                        branch.continuation = Continuation::DmgRolled { dmg: last_dmg };
                        *self.branches.entry(branch).or_insert(0.0f64) += probability * last_probability;
                    }
                    Continuation::DmgRolled { dmg } => {
                        let side = branch.turn.side.expect("Applying damage requires an acting side");
                        let (_, mv) = branch.turn.selected_move();
                        let dmg_dealt = branch.state.apply_damage(dmg, mv, side);
                        let max_hp = match side {
                            Side::Player => self.player.stats.hp,
                            Side::Ai => self.ai.stats.hp,
                        };
                        branch.state.combatant_mut(side).heal(mv.effect.drained_hp(dmg_dealt), max_hp);
                        if matches!(mv.effect, Effect::Trapping) && branch.state.winner().is_none() {
                            let outcomes = mv.effect.trapping_outcomes();
                            let (&(last_turns, last_probability), alternatives) = outcomes.split_last().expect("Trapping must have a duration");
                            for &(turns, duration_probability) in alternatives {
                                let mut alternative = branch.clone();
                                alternative.state.start_trapping(side, mv, dmg, turns);
                                self.finish_action(alternative, probability * duration_probability);
                            }
                            branch.state.start_trapping(side, mv, dmg, last_turns);
                            self.finish_action(branch, probability * last_probability);
                            continue;
                        }
                        let (target, target_types) = match side {
                            Side::Player => (&branch.state.ai, &self.ai.pokemon.species.types),
                            Side::Ai => (&branch.state.player, &self.player.pokemon.species.types),
                        };
                        let secondary_probability = mv.effect.secondary_probability(target, target_types, side == Side::Ai);
                        if secondary_probability > 0.0f64 {
                            let mut affected = branch.clone();
                            affected.state.apply_secondary(&mv.effect, side);
                            self.finish_action(affected, probability * secondary_probability);
                            probability *= 1.0f64 - secondary_probability;
                        }
                        self.finish_action(branch, probability);
                    }
                }
            }
        }
        self.outcome.unresolved = self.branches.values().copied().sum();
        self.branches.clear();
        self.outcome.clone()
    }

    fn begin_action(&mut self, mut branch: Branch, mut probability: f64) {
        let side = branch.turn.side.expect("Beginning an action requires an acting side");
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
            branch.state.continue_trapping(side);
            self.finish_action(branch, probability);
            return;
        }
        if branch.state.is_biding(side) {
            branch.state.continue_bide(side);
            self.finish_action(branch, probability);
            return;
        }
        branch.state.begin_move(side, slot, &mv.effect);
        let hit_prob = branch.turn.hit_probability(&branch.state);
        branch.continuation = Continuation::AccChecked { hit: true };
        #[expect(clippy::else_if_without_else, reason = "Done that way to create branching")]
        if hit_prob == 0.0f64 {
            branch.continuation = Continuation::AccChecked { hit: false };
        } else if hit_prob < 1.0f64 {
            let miss = Branch::new(branch.state.clone(), branch.turn.clone(), Continuation::AccChecked { hit: false });
            *self.branches.entry(miss).or_insert(0.0f64) += probability * (1.0f64 - hit_prob);
            probability *= hit_prob;
        }
        *self.branches.entry(branch).or_insert(0.0f64) += probability;
    }

    fn finish_action(&mut self, mut branch: Branch, probability: f64) {
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
            branch.continuation = Continuation::OrderDetermined;
            *self.branches.entry(branch).or_insert(0.0f64) += probability;
            return;
        }
        branch.state.finish_turn();
        self.start_turn(branch.state, probability);
    }

    fn start_turn(&mut self, state: BattleState, probability: f64) {
        let player_move = if state.is_trapped(Side::Player) {
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
        let ai_choices = if state.is_trapped(Side::Ai) {
            vec![(None, 1.0f64)]
        } else if let Some(mv) = state.ai.trapping_move() {
            vec![(Some((None, mv)), 1.0f64)]
        } else if state.is_biding(Side::Ai) {
            vec![(Some((None, &BIDE)), 1.0f64)]
        } else {
            self.game_ai
                .select(&self.ai.pokemon.moves, state.ai.disabled_slot(), state.player.has_status_condition())
                .into_iter()
                .map(|(mv, prob)| (Some(mv), prob))
                .collect()
        };
        let (&(last_move, last_probability), alternatives) = ai_choices.split_last().expect("The AI policy must produce at least one move choice");
        for &(ai_move, choice_probability) in alternatives {
            let alternative = Branch::new(
                state.clone(),
                TurnContext::new([Some(TurnAction::new(player_move)), Some(TurnAction::new(ai_move))], None),
                Continuation::MoveChosen,
            );
            *self.branches.entry(alternative).or_insert(0.0f64) += probability * choice_probability;
        }
        let branch = Branch::new(
            state,
            TurnContext::new([Some(TurnAction::new(player_move)), Some(TurnAction::new(last_move))], None),
            Continuation::MoveChosen,
        );
        *self.branches.entry(branch).or_insert(0.0f64) += probability * last_probability;
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
}

impl<'battle> Combatant<'battle> {
    const fn new(pokemon: &'battle Pokemon, stats: Stats) -> Self {
        Self { pokemon, stats }
    }

    fn crit_probability(&self, mv: &Move) -> f64 {
        let mut threshold = u16::from(self.pokemon.base_stats().spd / 2);
        if mv == &moves::RAZOR_LEAF || mv == &moves::SLASH {
            threshold = (threshold * 8).min(255);
        }
        f64::from(threshold) / 256.0
    }

    fn damage_rolls(&self, target: &Self, stages: &StatStages, target_stages: &StatStages, mv: &Move, crit: bool) -> Vec<(u16, f64)> {
        let (mut attack, mut defense) = if mv.mv_type.is_special() {
            (u32::from(self.stats.spc), u32::from(target.stats.spc))
        } else if crit {
            (u32::from(self.stats.atk), u32::from(target.stats.def))
        } else {
            (
                StatStages::apply(self.stats.atk, stages.atk),
                StatStages::apply(target.stats.def, target_stages.def),
            )
        };
        if attack >= 256 || defense >= 256 {
            attack = (attack / 4 % 256).max(1);
            defense = defense / 4 % 256;
        }
        let level = u32::from(self.pokemon.level) * if crit { 2 } else { 1 };
        let mut damage = ((2 * level / 5 + 2) * u32::from(mv.power) * attack)
            .checked_div(defense)
            .expect("The game does not account for 0 defense and crashes")
            / 50;
        damage = damage.min(997) + 2;
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
            for (damage, roll_probability) in self.damage_rolls(target, &state.stages, &target_state.stages, mv, crit) {
                expected_damage += f64::from(damage.min(target_state.hp)) * probability * roll_probability;
            }
        }
        expected_damage * hit_probability
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum Continuation {
    MoveChosen,
    OrderDetermined,
    AccChecked { hit: bool },
    CritChecked { crit: bool },
    DmgRolled { dmg: u16 },
}

#[derive(Clone, Eq, Hash, PartialEq)]
enum TurnAction {
    Move(Option<usize>, &'static Move),
    Trapped,
}

impl TurnAction {
    const fn new(selected_move: Option<(Option<usize>, &'static Move)>) -> Self {
        match selected_move {
            Some((slot, mv)) => Self::Move(slot, mv),
            None => Self::Trapped,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Branch {
    state: BattleState,
    turn: TurnContext,
    continuation: Continuation,
}

impl Branch {
    const fn new(state: BattleState, turn: TurnContext, continuation: Continuation) -> Self {
        Self { state, turn, continuation }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct TurnContext {
    actions: [Option<TurnAction>; 2],
    side: Option<Side>,
}

impl TurnContext {
    const fn new(actions: [Option<TurnAction>; 2], side: Option<Side>) -> Self {
        Self { actions, side }
    }

    const fn selected_move(&self) -> (Option<usize>, &'static Move) {
        let slot = match self.side.expect("The acting side must be determined before resolving its move") {
            Side::Player => 0,
            Side::Ai => 1,
        };
        match self.actions[slot].as_ref().expect("The acting combatant must have a pending action") {
            TurnAction::Move(idx, mv) => (*idx, *mv),
            TurnAction::Trapped => unreachable!(),
        }
    }

    fn crit_probability(&self, player: &Combatant<'_>, ai: &Combatant<'_>) -> f64 {
        let attacker = match self.side.expect("Critical-hit resolution must have an acting side") {
            Side::Player => player,
            Side::Ai => ai,
        };
        let (_, mv) = self.selected_move();
        attacker.crit_probability(mv)
    }

    fn damage_rolls(&self, state: &BattleState, player: &Combatant<'_>, ai: &Combatant<'_>, crit: bool) -> Vec<(u16, f64)> {
        let (attacker, target, stages, target_stages) = match self.side.expect("Damage resolution must have an acting side") {
            Side::Player => (player, ai, &state.player.stages, &state.ai.stages),
            Side::Ai => (ai, player, &state.ai.stages, &state.player.stages),
        };
        let (_, mv) = self.selected_move();
        attacker.damage_rolls(target, stages, target_stages, mv, crit)
    }

    fn finish_action(&mut self) {
        let slot = match self.side.expect("Finishing an action requires an acting side") {
            Side::Player => 0,
            Side::Ai => 1,
        };
        self.actions[slot] = None;
        self.side = match (self.actions[0].is_some(), self.actions[1].is_some()) {
            (true, false) => Some(Side::Player),
            (false, true) => Some(Side::Ai),
            (false, false) => None,
            (true, true) => unreachable!(),
        };
    }

    fn hit_probability(&self, state: &BattleState) -> f64 {
        let (_, mv) = self.selected_move();
        let side = self.side.expect("Accuracy resolution must have an acting side");
        let attacker = match side {
            Side::Player => &state.player,
            Side::Ai => &state.ai,
        };
        let probability = attacker.stages.hit_probability(mv);
        mv.effect.hit_probability(probability, side == Side::Ai)
    }

    #[expect(clippy::unused_self, reason = "To be fixed")]
    fn resolve_order(&self, state: &BattleState, player: &Combatant<'_>, ai: &Combatant<'_>) -> Option<Side> {
        let player_spd = StatStages::apply(player.stats.spd, state.player.stages.spd);
        let ai_spd = StatStages::apply(ai.stats.spd, state.ai.stages.spd);
        let order = player_spd.cmp(&ai_spd);
        match order {
            Ordering::Greater => Some(Side::Player),
            Ordering::Less => Some(Side::Ai),
            Ordering::Equal => None,
        }
    }
}
