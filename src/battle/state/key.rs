use core::hash::{BuildHasherDefault, Hash, Hasher};
use std::collections::HashMap;

use crate::{
    battle::state::{BideState, CombatantState, DisableState, RampageState, Sleep, StatStages, TrappingState, battle::BattleState},
    data::move_data::Move,
};

pub type StateMap<K, V = f64> = HashMap<K, V, BuildHasherDefault<StateHasher>>;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct BattleKey {
    player: CombatantKey,
    ai: CombatantKey,
    battle_data: u64,
}

impl BattleKey {
    pub(crate) fn action_batch_order(&self) -> u64 {
        let mut hasher = StateHasher::default();
        self.player.hash_action_group(&mut hasher);
        self.ai.hash_action_group(&mut hasher);
        hasher.write_u64(self.battle_data);
        let group = hasher.finish() & (u64::from(u32::MAX) << u32::BITS);
        group | u64::from(self.hp_pair())
    }

    pub(crate) fn hp_pair(&self) -> u32 {
        let hp_mask = u64::from(u16::MAX);
        (((self.player.flags & hp_mask) << u16::BITS) | (self.ai.flags & hp_mask)) as u32
    }

    pub(crate) fn to_state(&self) -> BattleState {
        self.to_state_with_hp(self.hp_pair())
    }

    pub(crate) fn to_state_with_hp(&self, hp_pair: u32) -> BattleState {
        let [pp1, pp2, pp3, pp4, dmg_low, dmg_high, ai_items_left, battle_flags] = self.battle_data.to_le_bytes();
        BattleState::from_key(
            self.player
                .to_state((hp_pair >> u16::BITS) as u16, (battle_flags & 7).checked_sub(1), battle_flags & 0x40 != 0),
            self.ai
                .to_state(hp_pair as u16, ((battle_flags >> 3) & 7).checked_sub(1), battle_flags & 0x80 != 0),
            [pp1, pp2, pp3, pp4],
            u16::from_le_bytes([dmg_low, dmg_high]),
            ai_items_left,
        )
    }

    pub(super) const fn batch_order_with_hp_offset(order: u64, block_idx: usize, cell_idx: usize) -> u64 {
        let player_hp = (block_idx & 0b1100) | (cell_idx >> 2usize);
        let ai_hp = ((block_idx & 3) << 2usize) | (cell_idx & 3);
        order | ((player_hp as u64) << u16::BITS) | ai_hp as u64
    }

    pub(super) const fn into_hp_tile(mut self) -> (Self, usize, usize) {
        let player_hp = self.player.flags as usize & 15;
        let ai_hp = self.ai.flags as usize & 15;
        let block_idx = (player_hp & 0b1100) | (ai_hp >> 2usize);
        let cell_idx = ((player_hp & 3) << 2usize) | (ai_hp & 3);
        self.player.flags &= !15;
        self.ai.flags &= !15;
        (self, block_idx, cell_idx)
    }

    pub(super) const fn new(player: CombatantKey, ai: CombatantKey, battle_data: u64) -> Self {
        Self { player, ai, battle_data }
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
        self.hash_fields(state, self.flags, self.status);
    }
}

impl CombatantKey {
    const RESTING: u8 = 7;
    const WAKING_FROM_REST: u8 = 8;
    const FROZEN: u8 = 14;

    pub(super) fn new(combatant: &CombatantState) -> Self {
        let [hp_low, hp_high] = combatant.hp.to_le_bytes();
        let flags = u8::from(combatant.poisoned)
            | (u8::from(combatant.seeded) << 1u8)
            | (u8::from(combatant.bide.is_some()) << 2u8)
            | (u8::from(combatant.burned) << 3u8)
            | (u8::from(combatant.trapping.is_some()) << 4u8)
            | (u8::from(combatant.paralyzed) << 5u8)
            | (u8::from(combatant.focus_energy) << 6u8)
            | (u8::from(combatant.underground) << 7u8);
        let disabled = combatant.disabled.as_ref();
        let bide = combatant.bide.as_ref();
        let locked_move_data = bide.map_or_else(|| combatant.rampage.as_ref().map_or(0, RampageState::key), |active| active.damage);
        let trapping = combatant.trapping.as_ref();
        let acc_and_eva = (combatant.stages.acc as u8 & 15) | ((combatant.stages.eva as u8) << 4u8);
        let spd_and_flags = (combatant.stages.spd as u8 & 15)
            | (u8::from(combatant.rest_atk_penalty) << 4u8)
            | (u8::from(combatant.rest_spd_penalty) << 5u8)
            | (u8::from(combatant.light_screen) << 6u8);
        let sleep = if combatant.frozen {
            Self::FROZEN
        } else {
            match combatant.sleep {
                Some(Sleep::Inflicted(turns_elapsed)) => turns_elapsed,
                Some(Sleep::Resting) => Self::RESTING,
                Some(Sleep::WakingFromRest) => Self::WAKING_FROM_REST,
                None => 15,
            }
        };
        let status = u64::from(sleep)
            | (u64::from(combatant.confusion_elapsed.unwrap_or(7)) << 4u64)
            | (u64::from(disabled.map_or(7, |active| active.slot)) << 7u64)
            | (u64::from(disabled.map_or(0, |active| active.turns_elapsed)) << 10u64)
            | (u64::from(bide.map_or(0, |active| active.turns_left)) << 14u64)
            | (u64::from(locked_move_data) << 16u64)
            | (u64::from(trapping.map_or(0, |active| active.damage)) << 32u64)
            | (u64::from(trapping.map_or(0, |active| active.turns_elapsed)) << 48u64)
            | (u64::from(trapping.is_some_and(|active| active.ends_this_turn)) << 51u64)
            | (u64::from(combatant.charging_slot.map_or(0, |slot| slot + 1)) << 52u64)
            | (u64::from(combatant.rage_accuracy.unwrap_or(0)) << 55u64)
            | (u64::from(combatant.reflect) << 63u64);
        Self {
            flags: u64::from_le_bytes([
                hp_low,
                hp_high,
                combatant.stages.atk as u8,
                combatant.stages.def as u8,
                spd_and_flags,
                combatant.stages.spc as u8,
                acc_and_eva,
                flags,
            ]),
            status,
            trapping_mv: trapping.map(|active| active.mv),
        }
    }

    fn hash_action_group(&self, state: &mut impl Hasher) {
        let timer_mask = 0b111_1111 | (0b1111 << 10u8);
        let frozen = u64::from(self.status & 15 == u64::from(Self::FROZEN));
        self.hash_fields(state, self.flags >> u16::BITS, (self.status & !timer_mask) | frozen);
    }

    fn hash_fields(&self, state: &mut impl Hasher, flags: u64, status: u64) {
        state.write_u64(flags);
        state.write_u64(status);
        if let Some(mv) = &self.trapping_mv {
            mv.hash(state);
        }
    }

    fn to_state(&self, hp: u16, recharge_slot: Option<u8>, mist: bool) -> CombatantState {
        let [_, _, atk, def, spd_and_flags, spc, acc_and_eva, flags] = self.flags.to_le_bytes();
        let sleep_code = (self.status & 15) as u8;
        let sleep = match sleep_code {
            0..=6 => Some(Sleep::Inflicted(sleep_code)),
            Self::RESTING => Some(Sleep::Resting),
            Self::WAKING_FROM_REST => Some(Sleep::WakingFromRest),
            _ => None,
        };
        let confusion = ((self.status >> 4u64) & 7) as u8;
        let disabled_slot = ((self.status >> 7u64) & 7) as u8;
        let rage_acc = (self.status >> 55u64) as u8;
        let locked_move_data = (self.status >> 16u64) as u16;
        let is_biding = flags & 4 != 0;
        CombatantState::from_key(
            hp,
            StatStages::new(
                atk.cast_signed(),
                def.cast_signed(),
                (spd_and_flags << 4u8).cast_signed() >> 4u8,
                spc.cast_signed(),
                (acc_and_eva << 4u8).cast_signed() >> 4u8,
                acc_and_eva.cast_signed() >> 4u8,
            ),
            is_biding.then_some(BideState::from_key(((self.status >> 14) & 3) as u8, locked_move_data)),
            flags & 8 != 0,
            (((self.status >> 52) & 7) as u8).checked_sub(1),
            (confusion != 7).then_some(confusion),
            (disabled_slot != 7).then_some(DisableState::from_key(disabled_slot, ((self.status >> 10) & 15) as u8)),
            flags & 64 != 0,
            sleep_code == Self::FROZEN,
            spd_and_flags & 0x40 != 0,
            mist,
            flags & 32 != 0,
            flags & 1 != 0,
            (rage_acc != 0).then_some(rage_acc),
            (!is_biding && locked_move_data != 0).then_some(RampageState::from_key(locked_move_data)),
            recharge_slot,
            self.status >> 63u64 != 0,
            spd_and_flags & 0x10 != 0,
            spd_and_flags & 0x20 != 0,
            flags & 2 != 0,
            sleep,
            self.trapping_mv.map(|mv| {
                TrappingState::from_key(
                    mv,
                    (self.status >> 32) as u16,
                    ((self.status >> 48) & 7) as u8,
                    self.status & (1 << 51u64) != 0,
                )
            }),
            flags & 128 != 0,
        )
    }
}
