use core::array;

use crate::{
    data::{Move, moves::STRUGGLE},
    pokemon::Pokemon,
};

// pub const BROCK: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const BUG_CATCHER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const HIKER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const JR_TRAINER_M: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const LASS: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
pub const RIVAL1: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const ROCKET: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const SUPER_NERD: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::IncreaseEffect];
// pub const YOUNGSTER: &[AiPolicy] = &[];

pub enum AiPolicy {
    DecreaseStatusCondition,
    // IncreaseEffect,
    // IncreaseSuperEffective,
}

impl AiPolicy {
    const fn score(&self, status_move: bool, target_has_status: bool) -> u8 {
        match *self {
            Self::DecreaseStatusCondition if status_move && target_has_status => 5,
            Self::DecreaseStatusCondition => 0,
        }
    }
}

pub enum MatchupKind {
    Boss,
    Trainer,
    Wild,
}

pub enum PlayerPolicy {
    MostDmg,
}

impl PlayerPolicy {
    pub fn select<F>(&self, moves: &[Option<&'static Move>; 4], pp: [u8; 4], disabled_slot: Option<u8>, damage: F) -> (Option<usize>, &'static Move)
    where
        F: Fn(&Move) -> f64,
    {
        let choices = moves
            .iter()
            .enumerate()
            .filter_map(|(slot, &mv)| mv.filter(|_| pp[slot] > 0 && disabled_slot != Some(slot as u8)).map(|tmp| (slot, tmp)));
        match self {
            &Self::MostDmg => {
                let mut choice = None;
                let mut most_damage = 0.0;
                for (slot, mv) in choices {
                    let expected_damage = damage(mv);
                    if choice.is_none() || expected_damage > most_damage {
                        choice = Some((Some(slot), mv));
                        most_damage = expected_damage;
                    }
                }
                choice.unwrap_or((None, &STRUGGLE))
            }
        }
    }
}

pub struct Encounter {
    pub pokemon: &'static Pokemon,
    pub max_depth: usize,
}

impl Encounter {
    pub const fn new(pokemon: &'static Pokemon) -> Self {
        Self {
            pokemon,
            max_depth: DEFAULT_MAX_DEPTH,
        }
    }

    const fn set_max_depth(&mut self, max_depth: usize) {
        self.max_depth = max_depth;
    }
}

pub struct GameAi {
    policies: &'static [AiPolicy],
}

impl GameAi {
    pub const fn new(policies: &'static [AiPolicy]) -> Self {
        Self { policies }
    }

    pub fn select(
        &self,
        moves: &[Option<&'static Move>; 4],
        disabled_slot: Option<u8>,
        target_has_status: bool,
    ) -> Vec<((Option<usize>, &'static Move), f64)> {
        let scores: [u8; 4] = array::from_fn(|slot| {
            let status_move = moves[slot].is_some_and(|mv| mv.power == 0 && mv.effect.inflicts_status_condition());
            self.policies.iter().map(|policy| policy.score(status_move, target_has_status)).sum()
        });
        let best_score = moves
            .iter()
            .enumerate()
            .filter_map(|(slot, mv)| mv.filter(|_| disabled_slot != Some(slot as u8)).map(|_| scores[slot]))
            .min();
        let Some(min_score) = best_score else {
            return vec![((None, &STRUGGLE), 1.0)];
        };
        let choices: Vec<_> = moves
            .iter()
            .enumerate()
            .filter_map(|(slot, &mv)| {
                mv.filter(|_| disabled_slot != Some(slot as u8) && scores[slot] == min_score)
                    .map(|tmp| (Some(slot), tmp))
            })
            .collect();
        let probability = 1.0f64 / choices.len() as f64;
        choices.into_iter().map(|choice| (choice, probability)).collect()
    }
}

pub struct MatchupGroup {
    pub kind: MatchupKind,
    pub party: Vec<Encounter>,
    pub ai: GameAi,
}

impl MatchupGroup {
    pub fn new_boss(party: Vec<&'static Pokemon>, policies: &'static [AiPolicy]) -> Self {
        Self::new(MatchupKind::Boss, party, policies)
    }

    pub fn new_trainer(party: Vec<&'static Pokemon>, policies: &'static [AiPolicy]) -> Self {
        Self::new(MatchupKind::Trainer, party, policies)
    }

    pub fn new_wild(pokemon: &'static Pokemon) -> Self {
        Self::new(MatchupKind::Wild, vec![pokemon], &[])
    }

    pub fn with_encounter_depth(mut self, party_index: usize, max_depth: usize) -> Self {
        self.party
            .get_mut(party_index)
            .expect("A depth override must refer to an existing party member")
            .set_max_depth(max_depth);
        self
    }

    fn new(kind: MatchupKind, party: Vec<&'static Pokemon>, policies: &'static [AiPolicy]) -> Self {
        Self {
            kind,
            party: party.into_iter().map(Encounter::new).collect(),
            ai: GameAi::new(policies),
        }
    }
}

const DEFAULT_MAX_DEPTH: usize = 90;
