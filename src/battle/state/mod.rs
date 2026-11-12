pub(super) mod accumulator;
pub(super) mod battle;
pub mod combatant;
pub(super) mod key;

use core::hash::Hash;

use crate::data::{effects::Effect, move_data::Move};

#[derive(Clone, Eq, PartialEq)]
pub enum Sleep {
    Inflicted(u8),
    Resting,
    WakingFromRest,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BideState {
    turns_left: u8,
    damage: u16,
}

impl BideState {
    pub(crate) const fn new(turns_left: u8) -> Self {
        Self { turns_left, damage: 0 }
    }

    pub(super) const fn advance(&mut self, last_damage: u16) -> Option<u16> {
        self.damage = self.damage.wrapping_add(last_damage);
        self.turns_left -= 1;
        if self.turns_left == 0 { Some(self.damage.wrapping_mul(2)) } else { None }
    }

    const fn from_key(turns_left: u8, damage: u16) -> Self {
        Self { turns_left, damage }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct DisableState {
    slot: u8,
    turns_elapsed: u8,
}

impl DisableState {
    pub(crate) const fn new(slot: usize) -> Self {
        Self {
            slot: slot as u8,
            turns_elapsed: 0,
        }
    }

    fn advance(&mut self) -> f64 {
        let probability = Effect::Disable.duration_end_probability(self.turns_elapsed);
        self.turns_elapsed += 1;
        probability
    }

    const fn from_key(slot: u8, turns_elapsed: u8) -> Self {
        Self { slot, turns_elapsed }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RampageState {
    pub slot: u8,
    turns_elapsed: u8,
    accuracy: u8,
}

impl RampageState {
    const fn key(&self) -> u16 {
        u16::from_le_bytes([(self.turns_elapsed << 2) | self.slot, self.accuracy])
    }

    const fn from_key(data: u16) -> Self {
        let [turns_and_slot, accuracy] = data.to_le_bytes();
        Self {
            slot: turns_and_slot & 3,
            turns_elapsed: turns_and_slot >> 2,
            accuracy,
        }
    }

    const fn new(slot: u8, turns_elapsed: u8, accuracy: u8) -> Self {
        Self {
            slot,
            turns_elapsed,
            accuracy,
        }
    }
}

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct StatStages {
    pub atk: i8,
    pub def: i8,
    pub spd: i8,
    pub spc: i8,
    pub acc: i8,
    pub eva: i8,
}

impl StatStages {
    pub(super) fn apply(stat: u16, stage: i8) -> u32 {
        Self::scale(u32::from(stat), stage).clamp(1, 999)
    }

    pub(super) fn hit_probability(&self, mv: &Move, target_eva: i8) -> f64 {
        f64::from(self.hit_threshold(mv.accuracy(), target_eva)) / 256.0
    }

    const fn new(atk: i8, def: i8, spd: i8, spc: i8, acc: i8, eva: i8) -> Self {
        Self {
            atk,
            def,
            spd,
            spc,
            acc,
            eva,
        }
    }

    fn hit_threshold(&self, accuracy: u8, target_eva: i8) -> u8 {
        let adjusted = Self::scale(u32::from(accuracy), self.acc).max(1);
        Self::scale(adjusted, -target_eva).clamp(1, 255) as u8
    }

    fn scale(stat: u32, stage: i8) -> u32 {
        let (numerator, denominator) = match stage {
            -6 => (25, 100),
            -5 => (28, 100),
            -4 => (33, 100),
            -3 => (40, 100),
            -2 => (50, 100),
            -1 => (66, 100),
            0 => (1, 1),
            1 => (15, 10),
            2 => (2, 1),
            3 => (25, 10),
            4 => (3, 1),
            5 => (35, 10),
            6 => (4, 1),
            _ => unreachable!("Stat stages must be between -6 and 6"),
        };
        stat * numerator / denominator
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TrappingState {
    mv: &'static Move,
    damage: u16,
    turns_elapsed: u8,
    ends_this_turn: bool,
}

impl TrappingState {
    const fn from_key(mv: &'static Move, damage: u16, turns_elapsed: u8, ends_this_turn: bool) -> Self {
        Self {
            mv,
            damage,
            turns_elapsed,
            ends_this_turn,
        }
    }

    const fn new(mv: &'static Move, damage: u16) -> Self {
        Self {
            mv,
            damage,
            turns_elapsed: 0,
            ends_this_turn: false,
        }
    }

    const fn end(&mut self) {
        self.ends_this_turn = true;
    }

    const fn is_finished(&self) -> bool {
        self.ends_this_turn
    }

    fn advance(&mut self) -> (&'static Move, u16, f64) {
        let expiry_prob = self.mv.effect.duration_end_probability(self.turns_elapsed);
        self.turns_elapsed += 1;
        (self.mv, self.damage, expiry_prob)
    }
}
