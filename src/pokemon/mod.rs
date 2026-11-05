pub mod data;

use crate::{
    battle::state::CombatantState,
    data::{BaseStats, Species, effects::Effect, move_data::Move},
    journey::PlayerPolicy,
    player::StatExp,
};

#[derive(Default)]
pub struct Dvs {
    hp: u8,
    atk: u8,
    def: u8,
    spd: u8,
    spc: u8,
}

impl Dvs {
    pub(super) const AVERAGE: Self = Self::new(7, 8, 8, 8);
    pub(super) const TRAINER: Self = Self::new(9, 8, 8, 8);
    pub(super) const WILD: Self = Self::new(0, 0, 0, 0);

    const fn new(atk: u8, def: u8, spd: u8, spc: u8) -> Self {
        Self {
            hp: ((atk & 1) << 3) | ((def & 1) << 2) | ((spd & 1) << 1) | (spc & 1),
            atk,
            def,
            spd,
            spc,
        }
    }
}

pub struct Pokemon {
    pub species: &'static Species,
    pub dvs: Dvs,
    pub level: u8,
    pub moves: [Option<&'static Move>; 4],
}

impl Pokemon {
    pub(super) const fn base_stats(&self) -> &BaseStats {
        &self.species.base_stats
    }

    pub(super) const fn name(&self) -> &'static str {
        self.species.name()
    }

    pub(super) fn base_damage(&self, power: u8, mut attack: u32, mut defense: u32, crit: bool) -> u32 {
        if attack >= 256 || defense >= 256 {
            attack = (attack / 4 % 256).max(1);
            defense = defense / 4 % 256;
        }
        let level = u32::from(self.level) * if crit { 2 } else { 1 };
        let damage = ((2 * level / 5 + 2) * u32::from(power) * attack)
            .checked_div(defense)
            .expect("The game does not account for 0 defense and crashes")
            / 50;
        damage.min(997) + 2
    }

    pub(super) fn from_species(species: &'static Species, dvs: Dvs, level: u8) -> Self {
        let mut moves = species.learnset;
        for &(lv, mv) in species.evo_moves {
            if lv > level {
                break;
            }
            if moves.contains(&Some(mv)) {
                continue;
            }
            if let Some(slot) = moves.iter_mut().find(|slot| slot.is_none()) {
                *slot = Some(mv);
            } else {
                moves.rotate_left(1);
                moves[3] = Some(mv);
            }
        }
        Self { species, dvs, level, moves }
    }

    pub(super) fn learn_level_up_moves(&mut self, policy: &PlayerPolicy, stat_exp: &StatExp) {
        let stats = Stats::from_base_stats(self.base_stats(), &self.dvs, self.level, stat_exp);
        for &(level, mv) in self.species.evo_moves {
            if level == self.level {
                self.learn_move(mv, policy, &stats);
            }
        }
    }

    pub(super) fn learn_tm(&mut self, mv: &'static Move, policy: &PlayerPolicy, stat_exp: &StatExp) -> bool {
        if !self.species.can_learn_tm(mv) {
            return false;
        }
        let stats = Stats::from_base_stats(self.base_stats(), &self.dvs, self.level, stat_exp);
        self.learn_move(mv, policy, &stats)
    }

    pub(super) fn level_up(&mut self, policy: &PlayerPolicy, stat_exp: &StatExp) {
        if self.level >= 100 {
            return;
        }
        self.level += 1;
        println!("Level up: Lv. {}", self.level);
        self.learn_level_up_moves(policy, stat_exp);
    }

    pub(super) fn stats(&self) -> Stats {
        Stats::from_base_stats(self.base_stats(), &self.dvs, self.level, &StatExp::default())
    }

    pub(super) fn try_evolve(&mut self, policy: &PlayerPolicy, stat_exp: &StatExp) {
        if let Some(species) = self.species.evolution_at(self.level) {
            self.species = species;
            println!("Evolved to {}", self.name());
            self.learn_level_up_moves(policy, stat_exp);
        }
    }

    const fn with_move(mut self, mv: &'static Move) -> Self {
        self.moves[2] = Some(mv);
        self
    }

    fn expected_neutral_damage(&self, stats: &Stats, mv: &Move) -> f64 {
        let hit_prob = f64::from(mv.accuracy()) / 256.0f64;
        if matches!(mv.effect, Effect::LeechSeed | Effect::Poison) {
            return f64::from(CombatantState::residual_damage(64)) * hit_prob;
        }
        if mv.power == 0 {
            return 0.0;
        }
        let (attack, defense) = if mv.mv_type.is_special() {
            (u32::from(stats.spc), 68)
        } else {
            (u32::from(stats.atk), 72)
        };
        let crit_prob = mv.crit_probability(self.base_stats().spd, false);
        let mut expected_dmg = 0.0f64;
        for (crit, probability) in [(false, 1.0f64 - crit_prob), (true, crit_prob)] {
            let mut damage = self.base_damage(mv.power, attack, defense, crit);
            if self.species.types.contains(&mv.mv_type) {
                damage += damage / 2;
            }
            let avg_dmg = if damage <= 1 {
                f64::from(damage)
            } else {
                (217..=255).map(|roll| f64::from(damage * roll / 255)).sum::<f64>() / 39.0f64
            };
            expected_dmg += avg_dmg * probability;
        }
        let expected_hits: f64 = mv
            .effect
            .hit_count_outcomes()
            .iter()
            .map(|&(hits, probabillity)| f64::from(hits) * probabillity)
            .sum();
        expected_dmg * expected_hits * hit_prob
    }

    fn learn_move(&mut self, mv: &'static Move, policy: &PlayerPolicy, stats: &Stats) -> bool {
        let Some(slot) = policy.learn(&self.moves, mv, |candidate| self.expected_neutral_damage(stats, candidate)) else {
            return false;
        };
        if let Some(forgotten) = self.moves[slot] {
            println!("Forgot: {}", forgotten.name());
        }
        self.moves[slot] = Some(mv);
        println!("Learnt: {}", mv.name());
        true
    }
}

pub struct Stats {
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    pub spd: u16,
    pub spc: u16,
}

impl Stats {
    pub(super) fn from_base_stats(base_stats: &BaseStats, dvs: &Dvs, level: u8, stat_exp: &StatExp) -> Self {
        let lv = u16::from(level);
        Self {
            hp: ((u16::from(base_stats.hp) + u16::from(dvs.hp)) * 2 + (f32::from(stat_exp.hp).sqrt().ceil() as u16).min(255) / 4) * lv / 100
                + lv
                + 10,
            atk: Self::calc_stat(u16::from(base_stats.atk), u16::from(dvs.atk), lv, f32::from(stat_exp.atk)),
            def: Self::calc_stat(u16::from(base_stats.def), u16::from(dvs.def), lv, f32::from(stat_exp.def)),
            spd: Self::calc_stat(u16::from(base_stats.spd), u16::from(dvs.spd), lv, f32::from(stat_exp.spd)),
            spc: Self::calc_stat(u16::from(base_stats.spc), u16::from(dvs.spc), lv, f32::from(stat_exp.spc)),
        }
    }

    fn calc_stat(stat: u16, dv: u16, level: u16, stat_exp: f32) -> u16 {
        ((stat + dv) * 2 + (stat_exp.sqrt().ceil() as u16).min(255) / 4) * level / 100 + 5
    }
}
