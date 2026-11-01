pub mod data;

use crate::{
    data::{BaseStats, Move, Species},
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

    pub(super) const fn base_stats(&self) -> &BaseStats {
        &self.species.base_stats
    }

    pub(super) const fn name(&self) -> &'static str {
        self.species.name()
    }

    pub(super) fn learn_level_up_moves(&mut self) {
        for &(level, mv) in self.species.evo_moves {
            if level != self.level || self.moves.contains(&Some(mv)) {
                continue;
            }
            if let Some(slot) = self.moves.iter_mut().find(|slot| slot.is_none()) {
                *slot = Some(mv);
                println!("Learnt: {}", mv.name());
            }
        }
    }

    pub(super) fn level_up(&mut self) {
        if self.level >= 100 {
            return;
        }
        self.level += 1;
        println!("Level up: Lv. {}", self.level);
        self.learn_level_up_moves();
    }

    pub(super) fn stats(&self) -> Stats {
        Stats::from_base_stats(self.base_stats(), &self.dvs, self.level, &StatExp::default())
    }

    pub(super) fn try_evolve(&mut self) {
        if let Some(species) = self.species.evolution_at(self.level) {
            self.species = species;
            println!("Evolved to {}", self.name());
            self.learn_level_up_moves();
        }
    }

    const fn with_move(mut self, mv: &'static Move) -> Self {
        self.moves[2] = Some(mv);
        self
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
