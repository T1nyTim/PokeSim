use core::hash::{BuildHasherDefault, Hash, Hasher};
use std::collections::HashMap;

use crate::battle::state::{BattleState, BideState, CombatantState, DisableState, StatStages, TrappingState};

pub type StateMap<K> = HashMap<K, f64, BuildHasherDefault<StateHasher>>;

#[derive(Eq, Hash, PartialEq)]
pub struct BattleKey {
    player: CombatantKey,
    ai: CombatantKey,
    pp_damage_and_items: u64,
}

impl BattleKey {
    pub(crate) fn into_state(self) -> BattleState {
        let [pp1, pp2, pp3, pp4, dmg_low, dmg_high, ai_items_left, _] = self.pp_damage_and_items.to_le_bytes();
        BattleState::from_key(
            self.player.into_state(),
            self.ai.into_state(),
            [pp1, pp2, pp3, pp4],
            u16::from_le_bytes([dmg_low, dmg_high]),
            ai_items_left,
        )
    }

    pub(super) const fn new(player: CombatantKey, ai: CombatantKey, pp_damage_and_items: u64) -> Self {
        Self {
            player,
            ai,
            pp_damage_and_items,
        }
    }
}

#[derive(Default)]
pub struct StateHasher(u64);

#[expect(clippy::missing_trait_methods, reason = "Other integer methods use the standard byte fallback")]
impl Hasher for StateHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let (chunks, remainder) = bytes.as_chunks::<8>();
        for chunk in chunks {
            self.write_u64(u64::from_le_bytes(*chunk));
        }
        if !remainder.is_empty() {
            let mut word = [0; 8];
            word[..remainder.len()].copy_from_slice(remainder);
            self.write_u64(u64::from_le_bytes(word));
        }
    }

    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write_u64(u64::from(i));
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.write_u64(u64::from(i));
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(u64::from(i));
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x517C_C1B7_2722_0A95);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }
}

#[derive(Clone, Eq, PartialEq)]
pub(super) struct CombatantKey {
    flags: u64,
    status: u64,
    trapping: Option<TrappingState>,
}

#[expect(clippy::missing_trait_methods, reason = "hash_slice method not required")]
impl Hash for CombatantKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.flags);
        state.write_u64(self.status);
        if let Some(trapping) = &self.trapping {
            trapping.mv.hash(state);
            state.write_u32(u32::from(trapping.damage) | (u32::from(trapping.turns_elapsed) << 16) | (u32::from(trapping.ends_this_turn) << 24));
        }
    }
}

impl CombatantKey {
    pub(super) const fn new(flags: u64, status: u64, trapping: Option<TrappingState>) -> Self {
        Self { flags, status, trapping }
    }

    fn into_state(self) -> CombatantState {
        let [hp_low, hp_high, atk, def, spd, spc, acc, flags] = self.flags.to_le_bytes();
        let [sleep, confusion, disabled_slot, disabled_turns, bide_turns, dmg_low, dmg_high, present] = self.status.to_le_bytes();
        CombatantState::from_key(
            u16::from_le_bytes([hp_low, hp_high]),
            StatStages::new(
                atk.cast_signed(),
                def.cast_signed(),
                spd.cast_signed(),
                spc.cast_signed(),
                acc.cast_signed(),
            ),
            (flags & 4 != 0).then_some(BideState::from_key(bide_turns, u16::from_le_bytes([dmg_low, dmg_high]))),
            flags & 8 != 0,
            (present & 2 != 0).then_some(confusion),
            (present & 4 != 0).then_some(DisableState::from_key(disabled_slot, disabled_turns)),
            flags & 32 != 0,
            flags & 1 != 0,
            flags & 2 != 0,
            (present & 1 != 0).then_some(sleep),
            self.trapping,
        )
    }
}
