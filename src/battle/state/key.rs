use core::hash::{BuildHasherDefault, Hash, Hasher};
use std::collections::HashMap;

use crate::{
    battle::state::{BattleState, BideState, CombatantState, DisableState, StatStages, TrappingState},
    data::move_data::Move,
};

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
    trapping_mv: Option<&'static Move>,
}

#[expect(clippy::missing_trait_methods, reason = "hash_slice method not required")]
impl Hash for CombatantKey {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        state.write_u64(self.flags);
        state.write_u64(self.status);
        if let Some(mv) = &self.trapping_mv {
            mv.hash(state);
        }
    }
}

impl CombatantKey {
    pub(super) fn new(combatant: &CombatantState) -> Self {
        let [hp_low, hp_high] = combatant.hp.to_le_bytes();
        let flags = u8::from(combatant.poisoned)
            | (u8::from(combatant.seeded) << 1u8)
            | (u8::from(combatant.bide.is_some()) << 2u8)
            | (u8::from(combatant.burned) << 3u8)
            | (u8::from(combatant.trapping.is_some()) << 4u8)
            | (u8::from(combatant.paralyzed) << 5u8)
            | (u8::from(combatant.focus_energy) << 6u8);
        let disabled = combatant.disabled.as_ref();
        let bide = combatant.bide.as_ref();
        let trapping = combatant.trapping.as_ref();
        let status = u64::from(combatant.sleep_elapsed.unwrap_or(15))
            | (u64::from(combatant.confusion_elapsed.unwrap_or(7)) << 4u64)
            | (u64::from(disabled.map_or(7, |active| active.slot)) << 7u64)
            | (u64::from(disabled.map_or(0, |active| active.turns_elapsed)) << 10u64)
            | (u64::from(bide.map_or(0, |active| active.turns_left)) << 14u64)
            | (u64::from(bide.map_or(0, |active| active.damage)) << 16u64)
            | (u64::from(trapping.map_or(0, |active| active.damage)) << 32u64)
            | (u64::from(trapping.map_or(0, |active| active.turns_elapsed)) << 48u64)
            | (u64::from(trapping.is_some_and(|active| active.ends_this_turn)) << 51u64);
        Self {
            flags: u64::from_le_bytes([
                hp_low,
                hp_high,
                combatant.stages.atk as u8,
                combatant.stages.def as u8,
                combatant.stages.spd as u8,
                combatant.stages.spc as u8,
                combatant.stages.acc as u8,
                flags,
            ]),
            status,
            trapping_mv: trapping.map(|active| active.mv),
        }
    }

    fn into_state(self) -> CombatantState {
        let [hp_low, hp_high, atk, def, spd, spc, acc, flags] = self.flags.to_le_bytes();
        let sleep = (self.status & 15) as u8;
        let confusion = ((self.status >> 4u64) & 7) as u8;
        let disabled_slot = ((self.status >> 7u64) & 7) as u8;
        CombatantState::from_key(
            u16::from_le_bytes([hp_low, hp_high]),
            StatStages::new(
                atk.cast_signed(),
                def.cast_signed(),
                spd.cast_signed(),
                spc.cast_signed(),
                acc.cast_signed(),
            ),
            (flags & 4 != 0).then_some(BideState::from_key(((self.status >> 14) & 3) as u8, (self.status >> 16) as u16)),
            flags & 8 != 0,
            (confusion != 7).then_some(confusion),
            (disabled_slot != 7).then_some(DisableState::from_key(disabled_slot, ((self.status >> 10) & 15) as u8)),
            flags & 64 != 0,
            flags & 32 != 0,
            flags & 1 != 0,
            flags & 2 != 0,
            (sleep != 15).then_some(sleep),
            self.trapping_mv.map(|mv| {
                TrappingState::from_key(
                    mv,
                    (self.status >> 32) as u16,
                    ((self.status >> 48) & 7) as u8,
                    self.status & (1 << 51u64) != 0,
                )
            }),
        )
    }
}
