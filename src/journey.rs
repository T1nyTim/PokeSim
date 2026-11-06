use core::array;

use crate::{
    data::{Item, Type, effects::Effect, move_data::Move, moves::STRUGGLE},
    pokemon::Pokemon,
};

// pub const AGATHA: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const BEAUTY: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const BIKER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const BIRD_KEEPER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const BLACKBELT: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const BLAINE: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const BROCK: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const BRUNO: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
pub const BUG_CATCHER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const BURGLAR: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const CHANNELER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const COOLTRAINER_F: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const COOLTRAINER_M: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const CUE_BALL: &[AiPolicy] = &[];
pub const ENGINEER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const ERIKA: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const FISHER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const GAMBLER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
pub const GENTLEMAN: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::IncreaseEffect];
// pub const GIOVANNI: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const HIKER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
pub const JR_TRAINER_F: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
pub const JR_TRAINER_M: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const JUGGLER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const KOGA: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const LANCE: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const LASS: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const LORELEI: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::IncreaseEffect, AiPolicy::TypeEffectiveness];
pub const LT_SURGE: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const MISTY: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const POKEMANIC: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::IncreaseEffect, AiPolicy::TypeEffectiveness];
// pub const PSYCHIC: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::IncreaseEffect];
pub const RIVAL1: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
pub const RIVAL2: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const RIVAL3: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const ROCKER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const ROCKET: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
// pub const SABRINA: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
pub const SAILOR: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const SCIENTIST: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::IncreaseEffect];
pub const SUPER_NERD: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::IncreaseEffect];
pub const SWIMMER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition, AiPolicy::TypeEffectiveness];
// pub const TAMER: &[AiPolicy] = &[AiPolicy::DecreaseStatusCondition];
pub const YOUNGSTER: &[AiPolicy] = &[];

pub enum AiPolicy {
    DecreaseStatusCondition,
    IncreaseEffect,
    TypeEffectiveness,
}

impl AiPolicy {
    fn score(&self, mv: &Move, moves: &[Option<&Move>; 4], target_types: &[Type; 2], target_has_status: bool, second_turn: bool) -> i8 {
        match *self {
            Self::DecreaseStatusCondition if mv.power == 0 && mv.effect.inflicts_status_condition() && target_has_status => 5,
            Self::IncreaseEffect if second_turn && mv.effect.encouraged_by_ai() => -1,
            Self::TypeEffectiveness => match mv.mv_type.ai_effectiveness(target_types) {
                20 => -1,
                0 | 5
                    if moves.iter().flatten().any(|other| {
                        matches!(other.effect, Effect::SpecialDamage | Effect::SuperFang) || (other.power > 0 && other.mv_type != mv.mv_type)
                    }) =>
                {
                    1
                }
                _ => 0,
            },
            Self::DecreaseStatusCondition | Self::IncreaseEffect => 0,
        }
    }
}

pub enum JourneyStep {
    Matchup(MatchupGroup),
    Item(Item),
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
    pub(super) fn learn<F>(&self, moves: &[Option<&'static Move>; 4], new_move: &'static Move, damage: F) -> Option<usize>
    where
        F: Fn(&Move) -> f64,
    {
        if moves.contains(&Some(new_move)) {
            return None;
        }
        if let Some(slot) = moves.iter().position(Option::is_none) {
            return Some(slot);
        }
        match self {
            Self::MostDmg => {
                let mut choice = None;
                let mut least_dmg = damage(new_move);
                for (slot, mv) in moves.iter().enumerate() {
                    let expected_dmg = damage(mv.expect("All move slots are occupied"));
                    if expected_dmg < least_dmg {
                        choice = Some(slot);
                        least_dmg = expected_dmg;
                    }
                }
                choice
            }
        }
    }

    pub(super) fn select<F>(
        &self,
        moves: &[Option<&'static Move>; 4],
        pp: [u8; 4],
        disabled_slot: Option<u8>,
        damage: F,
    ) -> (Option<usize>, &'static Move)
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
    pub(super) const fn new(pokemon: &'static Pokemon) -> Self {
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
    item: Option<TrainerItem>,
}

impl GameAi {
    pub(super) const fn item(&self) -> Option<&TrainerItem> {
        self.item.as_ref()
    }

    pub(super) const fn new(policies: &'static [AiPolicy]) -> Self {
        Self { policies, item: None }
    }

    pub(super) fn select(
        &self,
        moves: &[Option<&'static Move>; 4],
        target_types: &[Type; 2],
        disabled_slot: Option<u8>,
        target_has_status: bool,
        second_turn: bool,
    ) -> Vec<((Option<usize>, &'static Move), f64)> {
        let scores: [i8; 4] = array::from_fn(|slot| {
            moves[slot].map_or(0, |mv| {
                self.policies
                    .iter()
                    .map(|policy| policy.score(mv, moves, target_types, target_has_status, second_turn))
                    .sum()
            })
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

    const fn with_item(&mut self, item: &Item, probability: f64, uses: u8) {
        self.item = Some(TrainerItem::new(item, probability, uses));
    }
}

pub struct MatchupGroup {
    pub kind: MatchupKind,
    pub party: Vec<Encounter>,
    pub ai: GameAi,
}

impl MatchupGroup {
    pub(super) const fn with_item(mut self, item: &Item, probability: f64, uses: u8) -> Self {
        self.ai.with_item(item, probability, uses);
        self
    }

    pub(super) fn new_boss(party: Vec<&'static Pokemon>, policies: &'static [AiPolicy]) -> Self {
        Self::new(MatchupKind::Boss, party, policies)
    }

    pub(super) fn new_trainer(party: Vec<&'static Pokemon>, policies: &'static [AiPolicy]) -> Self {
        Self::new(MatchupKind::Trainer, party, policies)
    }

    pub(super) fn new_wild(pokemon: &'static Pokemon) -> Self {
        Self::new(MatchupKind::Wild, vec![pokemon], &[])
    }

    pub(super) fn with_encounter_depth(mut self, party_index: usize, max_depth: usize) -> Self {
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

pub struct TrainerItem {
    pub effect: &'static Effect,
    probability: f64,
    pub uses: u8,
}

impl TrainerItem {
    pub(super) const fn use_probability(&self, remaining: u8) -> f64 {
        if remaining == 0 { 0.0 } else { self.probability }
    }

    const fn new(item: &Item, probability: f64, uses: u8) -> Self {
        Self {
            effect: item.battle_effect().expect("A trainer item must have a supported battled effect"),
            probability,
            uses,
        }
    }
}

const DEFAULT_MAX_DEPTH: usize = 20;
