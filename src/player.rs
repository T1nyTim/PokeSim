use crate::{
    data::{BaseStats, Item},
    journey::{MatchupKind, PlayerPolicy},
    pokemon::{Pokemon, Stats},
};

pub struct Player {
    pub active: Pokemon,
    pub policy: PlayerPolicy,
    xp: u32,
    stat_exp: StatExp,
    leveled_up: bool,
}

impl Player {
    pub(super) fn new(active: Pokemon, policy: PlayerPolicy) -> Self {
        let xp = active.species.growth.experience_at(active.level);
        Self {
            active,
            policy,
            xp,
            stat_exp: StatExp::default(),
            leveled_up: false,
        }
    }

    pub(super) fn finish_matchup(&mut self) {
        if self.leveled_up {
            self.leveled_up = false;
            self.active.try_evolve(&self.policy, &self.stat_exp);
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
            self.active.level_up(&self.policy, &self.stat_exp);
            self.leveled_up = true;
        }
    }

    pub(super) fn stats(&self) -> Stats {
        Stats::from_base_stats(self.active.base_stats(), &self.active.dvs, self.active.level, &self.stat_exp)
    }

    pub(super) fn use_item(&mut self, item: &Item) -> bool {
        // Doesn't feel like the right place to keep this
        let used = match item {
            Item::HpUp => self.stat_exp.use_hp_up(),
            Item::RareCandy => {
                if self.active.level >= 100 {
                    return false;
                }
                self.active.level_up(&self.policy, &self.stat_exp);
                self.xp = self.active.species.growth.experience_at(self.active.level);
                self.leveled_up = true;
                self.finish_matchup();
                true
            }
            Item::Tm(mv) => self.active.learn_tm(mv, &self.policy, &self.stat_exp),
            Item::XDefend => false,
            Item::FireStone | Item::LeafStone | Item::MoonStone | Item::ThunderStone | Item::WaterStone => todo!(),
        };
        if used {
            if matches!(item, Item::Tm(_)) {
                println!("Used TM: {}", item.name());
            } else {
                println!("Used: {}", item.name());
            }
        }
        used
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
    const fn use_hp_up(&mut self) -> bool {
        if self.hp >= 25_600 {
            return false;
        }
        self.hp += 2_560;
        true
    }

    fn gain(&mut self, base: &BaseStats) {
        self.hp = self.hp.saturating_add(u16::from(base.hp));
        self.atk = self.atk.saturating_add(u16::from(base.atk));
        self.def = self.def.saturating_add(u16::from(base.def));
        self.spd = self.spd.saturating_add(u16::from(base.spd));
        self.spc = self.spc.saturating_add(u16::from(base.spc));
    }
}
