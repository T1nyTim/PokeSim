extern crate alloc;

use alloc::collections::BTreeMap;
use core::{array, cell::OnceCell};
use std::collections::HashMap;

use crate::{
    battle::state::{CombatantState, StatStages},
    data::{Move, effects::Effect, moves::STRUGGLE},
    pokemon::{Pokemon, Stats},
};

type StagedDamageRolls = [[OnceCell<Vec<(u16, f64)>>; 13]; 13];
type StagedExpectedDamage = [[OnceCell<Vec<f64>>; 13]; 13];

pub struct Combatant<'battle> {
    pub pokemon: &'battle Pokemon,
    pub stats: Stats,
    damage_rolls: HashMap<&'static Move, DamageRollCache>,
}

impl<'battle> Combatant<'battle> {
    pub(super) fn confusion_damage(&self, state: &CombatantState) -> u16 {
        self.pokemon.base_damage(
            40,
            state.physical_attack(self.stats.atk),
            StatStages::apply(self.stats.def, state.stages.def),
            false,
        ) as u16
    }

    pub(super) fn damage_outcomes(&self, target: &Self, state: &CombatantState, target_state: &CombatantState, mv: &Move) -> &[(u16, f64)] {
        let cache = self.damage_rolls.get(mv).expect("A selected move must have a damage-roll cache");
        let (attack, defense) = Self::damage_stage_indices(state, target_state, mv);
        let outcomes = &cache.staged(state, mv).outcomes[attack][defense];
        outcomes.get_or_init(|| {
            let crit_prob = self.crit_probability(mv);
            let normal = self.damage_rolls(target, state, target_state, mv, false);
            let mut combined: Vec<_> = normal
                .iter()
                .map(|&(damage, probability)| (damage, probability * (1.0f64 - crit_prob)))
                .filter(|&(_, probability)| probability > 0.0f64)
                .collect();
            if crit_prob > 0.0f64 {
                for &(damage, probability) in self.damage_rolls(target, state, target_state, mv, true) {
                    let crit_roll_prob = probability * crit_prob;
                    match combined.binary_search_by_key(&damage, |&(rolled_dmg, _)| rolled_dmg) {
                        Ok(idx) => combined[idx].1 += crit_roll_prob,
                        Err(idx) => combined.insert(idx, (damage, crit_roll_prob)),
                    }
                }
            }
            if matches!(mv.effect, Effect::TwoToFiveAttacks) {
                let mut totals = BTreeMap::new();
                for (damage, dmg_prob) in combined {
                    for &(hits, hit_count_prob) in mv.effect.hit_count_outcomes() {
                        let total_dmg = damage.saturating_mul(hits);
                        *totals.entry(total_dmg).or_insert(0.0f64) += dmg_prob * hit_count_prob;
                    }
                }
                return totals.into_iter().collect();
            }
            combined
        })
    }

    pub(super) fn expected_damage(&self, target: &Self, state: &CombatantState, target_state: &CombatantState, mv: &Move) -> f64 {
        let hit_probability = state.stages.hit_probability(mv);
        if matches!(mv.effect, Effect::LeechSeed | Effect::Poison) {
            let can_apply = if matches!(mv.effect, Effect::LeechSeed) {
                target_state.can_be_seeded(&target.pokemon.species.types)
            } else {
                target_state.can_be_poisoned(&target.pokemon.species.types)
            };
            if !can_apply {
                return 0.0;
            }
            return f64::from(CombatantState::residual_damage(target.stats.hp).min(target_state.hp)) * hit_probability;
        }
        if mv.power == 0 {
            return 0.0;
        }
        let cache = self.damage_rolls.get(mv).expect("A selected move must have a damage-roll cache");
        let (attack, defense) = Self::damage_stage_indices(state, target_state, mv);
        let cached_scores = &cache.staged(state, mv).expected[attack][defense];
        let scores = cached_scores.get_or_init(|| {
            let outcomes = self.damage_outcomes(target, state, target_state, mv);
            let max_damage = outcomes.last().map_or(0, |&(damage, _)| damage);
            let max_hp = target.stats.hp.min(max_damage);
            (0..=max_hp)
                .map(|hp| {
                    outcomes
                        .iter()
                        .map(|&(damage, probability)| f64::from(damage.min(hp)) * probability)
                        .sum()
                })
                .collect()
        });
        scores[usize::from(target_state.hp).min(scores.len() - 1)] * hit_probability
    }

    pub(super) fn new(pokemon: &'battle Pokemon, stats: Stats) -> Self {
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

    const fn damage_stage_indices(state: &CombatantState, target_state: &CombatantState, mv: &Move) -> (usize, usize) {
        let (attack, defense) = if mv.mv_type.is_special() {
            (state.stages.spc, target_state.stages.spc)
        } else {
            (state.stages.atk, target_state.stages.def)
        };
        ((attack + 6) as usize, (defense + 6) as usize)
    }

    fn crit_probability(&self, mv: &Move) -> f64 {
        mv.crit_probability(self.pokemon.base_stats().spd)
    }

    fn calculate_damage_rolls(&self, target: &Self, state: &CombatantState, target_state: &CombatantState, mv: &Move, crit: bool) -> Vec<(u16, f64)> {
        let (attack, defense) = if mv.mv_type.is_special() {
            if crit {
                (u32::from(self.stats.spc), u32::from(target.stats.spc))
            } else {
                (
                    StatStages::apply(self.stats.spc, state.stages.spc),
                    StatStages::apply(target.stats.spc, target_state.stages.spc),
                )
            }
        } else if crit {
            (u32::from(self.stats.atk), u32::from(target.stats.def))
        } else {
            (
                state.physical_attack(self.stats.atk),
                StatStages::apply(target.stats.def, target_state.stages.def),
            )
        };
        let mut damage = self.pokemon.base_damage(mv.power, attack, defense, crit);
        if self.pokemon.species.types.contains(&mv.mv_type) {
            damage += damage / 2;
        }
        let types = &target.pokemon.species.types;
        damage = damage * mv.mv_type.effectiveness(&types[0]) / 10;
        if types[1] != types[0] {
            damage = damage * mv.mv_type.effectiveness(&types[1]) / 10;
        }
        if damage <= 1 {
            return vec![(damage as u16, 1.0f64)];
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

    fn damage_rolls(&self, target: &Self, state: &CombatantState, target_state: &CombatantState, mv: &Move, crit: bool) -> &[(u16, f64)] {
        let cache = self.damage_rolls.get(mv).expect("A selected move must have a damage-roll cache");
        let rolls = if crit {
            &cache.critical
        } else {
            let (attack, defense) = Self::damage_stage_indices(state, target_state, mv);
            &cache.staged(state, mv).normal[attack][defense]
        };
        rolls.get_or_init(|| self.calculate_damage_rolls(target, state, target_state, mv, crit))
    }
}

struct DamageRollCache {
    normal: StagedDamageCache,
    burned: OnceCell<Box<StagedDamageCache>>,
    critical: OnceCell<Vec<(u16, f64)>>,
}

impl DamageRollCache {
    fn new() -> Self {
        Self {
            normal: StagedDamageCache::new(),
            burned: OnceCell::new(),
            critical: OnceCell::new(),
        }
    }

    fn staged(&self, state: &CombatantState, mv: &Move) -> &StagedDamageCache {
        if state.burned && !mv.mv_type.is_special() {
            self.burned.get_or_init(|| Box::new(StagedDamageCache::new()))
        } else {
            &self.normal
        }
    }
}

struct StagedDamageCache {
    normal: StagedDamageRolls,
    outcomes: StagedDamageRolls,
    expected: StagedExpectedDamage,
}

impl StagedDamageCache {
    fn new() -> Self {
        Self {
            normal: array::from_fn(|_| array::from_fn(|_| OnceCell::new())),
            outcomes: array::from_fn(|_| array::from_fn(|_| OnceCell::new())),
            expected: array::from_fn(|_| array::from_fn(|_| OnceCell::new())),
        }
    }
}
