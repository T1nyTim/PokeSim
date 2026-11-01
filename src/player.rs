use crate::{
    data::BaseStats,
    matchup::MatchupKind,
    pokemon::{Pokemon, Stats},
};

pub struct Player {
    pub active: Pokemon,
    xp: u32,
    stat_exp: StatExp,
    leveled_up: bool,
}

impl Player {
    pub(super) fn new(active: Pokemon) -> Self {
        let xp = active.species.growth.experience_at(active.level);
        Self {
            active,
            xp,
            stat_exp: StatExp::default(),
            leveled_up: false,
        }
    }

    pub(super) fn finish_matchup(&mut self) {
        if self.leveled_up {
            self.leveled_up = false;
            self.active.try_evolve();
        }
    }

    pub(super) fn gain_experience(&mut self, pokemon: &Pokemon, kind: &MatchupKind) {
        let mut xp = u32::from(pokemon.species.base_exp) * u32::from(pokemon.level) / 7;
        if matches!(kind, MatchupKind::Boss | MatchupKind::Trainer) {
            xp += xp / 2;
        }
        self.stat_exp.gain(pokemon.base_stats());
        let growth = &self.active.species.growth;
        self.xp = (self.xp + xp).min(growth.experience_at(100));
        while self.active.level < 100 && self.xp >= growth.experience_at(self.active.level + 1) {
            self.active.level += 1;
            self.leveled_up = true;
            println!("Level up: Lv. {}", self.active.level);
            self.active.learn_level_up_moves();
        }
    }

    pub(super) fn stats(&self) -> Stats {
        Stats::from_base_stats(self.active.base_stats(), &self.active.dvs, self.active.level, &self.stat_exp)
    }
}

#[derive(Default)]
pub struct StatExp {
    pub hp: u16,
    pub atk: u16,
    pub def: u16,
    pub spd: u16,
    pub spc: u16,
}

impl StatExp {
    fn gain(&mut self, base: &BaseStats) {
        self.hp = self.hp.saturating_add(u16::from(base.hp));
        self.atk = self.atk.saturating_add(u16::from(base.atk));
        self.def = self.def.saturating_add(u16::from(base.def));
        self.spd = self.spd.saturating_add(u16::from(base.spd));
        self.spc = self.spc.saturating_add(u16::from(base.spc));
    }
}
