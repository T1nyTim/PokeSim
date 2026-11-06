use core::cmp::Ordering;

use crate::{
    battle::{Combatant, Side, state::BattleState},
    data::{Type, effects::Effect, move_data::Move},
};

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) enum TurnAction {
    Move(Option<usize>, &'static Move),
    AiChoice(u8, u8),
    Skip,
}

impl TurnAction {
    pub(super) const fn new(selected_move: Option<(Option<usize>, &'static Move)>) -> Self {
        match selected_move {
            Some((slot, mv)) => Self::Move(slot, mv),
            None => Self::Skip,
        }
    }

    const fn priority(&self) -> u8 {
        match self {
            Self::Move(_, mv) => mv.priority(),
            Self::AiChoice(_, priority) => *priority,
            Self::Skip => 0,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct Branch {
    pub state: BattleState,
    pub turn: TurnContext,
}

impl Branch {
    pub(super) const fn new(state: BattleState, turn: TurnContext) -> Self {
        Self { state, turn }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) struct TurnContext {
    pub actions: [Option<TurnAction>; 2],
    pub side: Option<Side>,
}

impl TurnContext {
    pub(super) const fn can_flinch_target(&self) -> bool {
        let slot = match self.side.expect("Flinching requires an acting side") {
            Side::Player => 1,
            Side::Ai => 0,
        };
        matches!(self.actions[slot], Some(TurnAction::Move(_, _) | TurnAction::AiChoice(_, _)))
    }

    pub(super) const fn flinch_target(&mut self) {
        let slot = match self.side.expect("Flinching requires an acting side") {
            Side::Player => 1,
            Side::Ai => 0,
        };
        if self.actions[slot].is_some() {
            self.actions[slot] = Some(TurnAction::Skip);
        }
    }

    pub(super) const fn new(actions: [Option<TurnAction>; 2], side: Option<Side>) -> Self {
        Self { actions, side }
    }

    pub(super) const fn selected_move(&self) -> (Option<usize>, &'static Move) {
        let slot = match self.side.expect("The acting side must be determined before resolving its move") {
            Side::Player => 0,
            Side::Ai => 1,
        };
        match self.actions[slot].as_ref().expect("The acting combatant must have a pending action") {
            TurnAction::Move(idx, mv) => (*idx, *mv),
            TurnAction::AiChoice(_, _) | TurnAction::Skip => unreachable!(),
        }
    }

    pub(super) const fn skips_action(&self) -> bool {
        let slot = match self.side.expect("Checking an action requires an acting side") {
            Side::Player => 0,
            Side::Ai => 1,
        };
        matches!(self.actions[slot], Some(TurnAction::Skip))
    }

    pub(super) fn damage_outcomes<'combatant>(
        &self,
        state: &BattleState,
        player: &'combatant Combatant<'_>,
        ai: &'combatant Combatant<'_>,
    ) -> &'combatant [(u16, f64)] {
        let (attacker, target, attacker_state, target_state) = match self.side.expect("Damage resolution must have an acting side") {
            Side::Player => (player, ai, &state.player, &state.ai),
            Side::Ai => (ai, player, &state.ai, &state.player),
        };
        let (_, mv) = self.selected_move();
        attacker.damage_outcomes(target, attacker_state, target_state, mv)
    }

    pub(super) fn finish_action(&mut self) {
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

    pub(super) fn hit_probability(&self, state: &BattleState, target_types: &[Type; 2]) -> f64 {
        let (_, mv) = self.selected_move();
        let side = self.side.expect("Accuracy resolution must have an acting side");
        let (attacker, target) = match side {
            Side::Player => (&state.player, &state.ai),
            Side::Ai => (&state.ai, &state.player),
        };
        if target.underground && !mv.effect.targets_user() && !matches!(mv.effect, Effect::Swift | Effect::SwitchAndTeleport) {
            return 0.0;
        }
        let probability = attacker.stages.hit_probability(mv);
        mv.hit_probability(probability, target_types, side == Side::Ai)
    }

    pub(super) fn resolve_order(&self, state: &BattleState, player: &Combatant<'_>, ai: &Combatant<'_>) -> Option<Side> {
        let player_priority = self.actions[0].as_ref().map_or(0, TurnAction::priority);
        let ai_priority = self.actions[1].as_ref().map_or(0, TurnAction::priority);
        let player_spd = state.player.speed(player.stats.spd);
        let ai_spd = state.ai.speed(ai.stats.spd);
        let order = player_priority.cmp(&ai_priority).then_with(|| player_spd.cmp(&ai_spd));
        match order {
            Ordering::Greater => Some(Side::Player),
            Ordering::Less => Some(Side::Ai),
            Ordering::Equal => None,
        }
    }
}
