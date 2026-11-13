use crate::data::{Effect, Move};

#[derive(Clone, Eq, PartialEq)]
pub enum Sleep {
    Inflicted(u8),
    Resting,
    WakingFromRest,
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BideState {
    pub turns_left: u8,
    pub damage: u16,
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

    pub(super) const fn from_key(turns_left: u8, damage: u16) -> Self {
        Self { turns_left, damage }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct DisableState {
    pub slot: u8,
    pub turns_elapsed: u8,
}

impl DisableState {
    pub(crate) const fn new(slot: usize) -> Self {
        Self {
            slot: slot as u8,
            turns_elapsed: 0,
        }
    }

    pub(super) const fn from_key(slot: u8, turns_elapsed: u8) -> Self {
        Self { slot, turns_elapsed }
    }

    pub(super) fn advance(&mut self) -> f64 {
        let probability = Effect::Disable.duration_end_probability(self.turns_elapsed);
        self.turns_elapsed += 1;
        probability
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct RampageState {
    pub slot: u8,
    pub turns_elapsed: u8,
    pub accuracy: u8,
}

impl RampageState {
    pub(super) const fn key(&self) -> u16 {
        u16::from_le_bytes([(self.turns_elapsed << 2) | self.slot, self.accuracy])
    }

    pub(super) const fn from_key(data: u16) -> Self {
        let [turns_and_slot, accuracy] = data.to_le_bytes();
        Self {
            slot: turns_and_slot & 3,
            turns_elapsed: turns_and_slot >> 2,
            accuracy,
        }
    }

    pub(super) const fn new(slot: u8, turns_elapsed: u8, accuracy: u8) -> Self {
        Self {
            slot,
            turns_elapsed,
            accuracy,
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TrappingState {
    pub mv: &'static Move,
    pub damage: u16,
    pub turns_elapsed: u8,
    pub ends_this_turn: bool,
}

impl TrappingState {
    pub(super) const fn end(&mut self) {
        self.ends_this_turn = true;
    }

    pub(super) const fn from_key(mv: &'static Move, damage: u16, turns_elapsed: u8, ends_this_turn: bool) -> Self {
        Self {
            mv,
            damage,
            turns_elapsed,
            ends_this_turn,
        }
    }

    pub(super) const fn is_finished(&self) -> bool {
        self.ends_this_turn
    }

    pub(super) const fn new(mv: &'static Move, damage: u16) -> Self {
        Self {
            mv,
            damage,
            turns_elapsed: 0,
            ends_this_turn: false,
        }
    }

    pub(super) fn advance(&mut self) -> (&'static Move, u16, f64) {
        let expiry_prob = self.mv.effect.duration_end_probability(self.turns_elapsed);
        self.turns_elapsed += 1;
        (self.mv, self.damage, expiry_prob)
    }
}
