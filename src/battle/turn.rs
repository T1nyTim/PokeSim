use core::{array, cmp::Ordering};

use crate::{
    battle::{
        Combatant, Side,
        combatant::DamageOutcomes,
        state::{battle::BattleState, key::BattleKey},
    },
    data::{Type, effects::Effect, move_data::Move},
};

#[derive(Clone, Eq, Hash, PartialEq)]
pub(super) enum TurnAction {
    Move(Option<u8>, &'static Move),
    AiChoice(u8, u8),
    Skip,
}

impl TurnAction {
    pub(super) fn new(selected_move: Option<(Option<usize>, &'static Move)>) -> Self {
        match selected_move {
            Some((slot, mv)) => Self::Move(slot.map(|idx| idx as u8), mv),
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

#[derive(Clone)]
pub(super) struct Branch {
    pub state: BattleState,
    pub turn: TurnContext,
}

impl Branch {
    pub(super) const fn new(state: BattleState, turn: TurnContext) -> Self {
        Self { state, turn }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub(super) struct PendingAction {
    state: BattleKey,
    action: TurnAction,
    side: Side,
}

impl PendingAction {
    pub(super) fn into_branch(self) -> Branch {
        let actions = match self.side {
            Side::Player => [Some(self.action), None],
            Side::Ai => [None, Some(self.action)],
        };
        Branch::new(self.state.to_state(), TurnContext::new(actions, Some(self.side)))
    }

    pub(super) fn new(state: &BattleState, side: Side, action: TurnAction) -> Self {
        Self {
            state: state.key(),
            action,
            side,
        }
    }
}

#[derive(Clone)]
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

    pub(super) const fn skips_action(&self) -> bool {
        let slot = match self.side.expect("Checking an action requires an acting side") {
            Side::Player => 0,
            Side::Ai => 1,
        };
        matches!(self.actions[slot], Some(TurnAction::Skip))
    }

    pub(super) fn damage_outcomes(&self, state: &BattleState, player: &Combatant<'_>, ai: &Combatant<'_>) -> DamageOutcomes {
        let (attacker, target, attacker_state, target_state) = match self.side.expect("Damage resolution must have an acting side") {
            Side::Player => (player, ai, &state.player, &state.ai),
            Side::Ai => (ai, player, &state.ai, &state.player),
        };
        let (_, mv) = self.selected_move();
        attacker.damage_outcomes(target, attacker_state, target_state, mv)
    }

    pub(super) fn hit_probability(&self, state: &mut BattleState, target_types: &[Type; 2]) -> f64 {
        let (_, mv) = self.selected_move();
        let side = self.side.expect("Accuracy resolution must have an acting side");
        let (attacker, target) = match side {
            Side::Player => (&mut state.player, &state.ai),
            Side::Ai => (&mut state.ai, &state.player),
        };
        if target.underground && !mv.effect.targets_user() && !matches!(mv.effect, Effect::Swift | Effect::SwitchAndTeleport) {
            return 0.0;
        }
        let probability = if matches!(mv.effect, Effect::Rage) {
            attacker.rage_hit_probability(target.stages.eva)
        } else if matches!(mv.effect, Effect::ThrashPetalDance) {
            attacker.rampage_hit_probability(target.stages.eva)
        } else {
            attacker.stages.hit_probability(mv, target.stages.eva)
        };
        mv.hit_probability(probability, target_types, side == Side::Ai)
    }

    pub(super) fn next_action(self) -> Option<(Side, TurnAction)> {
        let [player_action, ai_action] = self.actions;
        match self.side.expect("Finishing an action requires an acting side") {
            Side::Player => ai_action.map(|action| (Side::Ai, action)),
            Side::Ai => player_action.map(|action| (Side::Player, action)),
        }
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

    pub(super) fn selected_move(&self) -> (Option<usize>, &'static Move) {
        let slot = match self.side.expect("The acting side must be determined before resolving its move") {
            Side::Player => 0,
            Side::Ai => 1,
        };
        match self.actions[slot].as_ref().expect("The acting combatant must have a pending action") {
            TurnAction::Move(idx, mv) => (idx.map(usize::from), *mv),
            TurnAction::AiChoice(_, _) | TurnAction::Skip => unreachable!(),
        }
    }
}

type AiChoice = ((Option<usize>, &'static Move), f64);

pub(super) struct AiChoices {
    pub moves: [Vec<AiChoice>; 2],
    pub probabilities: [f64; 2],
}

impl AiChoices {
    pub(super) fn new(choices: Vec<AiChoice>) -> Self {
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
