use core::cmp::Ordering;

use crate::{
    battle::{
        Combatant, Side,
        state::{BattleState, StatStages},
    },
    data::Move,
};

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) enum TurnAction {
    Move(Option<usize>, &'static Move),
    AiChoice(u8),
    Skip,
}

impl TurnAction {
    pub(super) const fn new(selected_move: Option<(Option<usize>, &'static Move)>) -> Self {
        match selected_move {
            Some((slot, mv)) => Self::Move(slot, mv),
            None => Self::Skip,
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
            TurnAction::AiChoice(_) | TurnAction::Skip => unreachable!(),
        }
    }

    pub(super) fn crit_probability(&self, player: &Combatant<'_>, ai: &Combatant<'_>) -> f64 {
        let attacker = match self.side.expect("Critical-hit resolution must have an acting side") {
            Side::Player => player,
            Side::Ai => ai,
        };
        let (_, mv) = self.selected_move();
        attacker.crit_probability(mv)
    }

    pub(super) fn damage_rolls<'combatant>(
        &self,
        state: &BattleState,
        player: &'combatant Combatant<'_>,
        ai: &'combatant Combatant<'_>,
        crit: bool,
    ) -> &'combatant [(u16, f64)] {
        let (attacker, target, stages, target_stages) = match self.side.expect("Damage resolution must have an acting side") {
            Side::Player => (player, ai, &state.player.stages, &state.ai.stages),
            Side::Ai => (ai, player, &state.ai.stages, &state.player.stages),
        };
        let (_, mv) = self.selected_move();
        attacker.damage_rolls(target, stages, target_stages, mv, crit)
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

    pub(super) fn hit_probability(&self, state: &BattleState) -> f64 {
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
    pub(super) fn resolve_order(&self, state: &BattleState, player: &Combatant<'_>, ai: &Combatant<'_>) -> Option<Side> {
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
