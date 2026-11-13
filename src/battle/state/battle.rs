use core::hash::{Hash, Hasher};

use crate::{
    battle::{
        Combatant, Side,
        state::{BattleKey, BideState, CombatantFlags, CombatantKey, CombatantState},
    },
    data::{BIDE, Effect, Move, Type},
    journey::TrainerItem,
};

#[derive(Clone, Eq, PartialEq)]
pub struct BattleState {
    pub player: CombatantState,
    pub ai: CombatantState,
    pub player_pp: [u8; 4],
    last_damage: u16,
    pub ai_items_left: u8,
    pub last_player_move: Option<u8>,
    pub ai_copied_move: Option<u8>,
}

#[expect(clippy::missing_trait_methods, reason = "hash_slice method not required")]
impl Hash for BattleState {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.key().hash(state);
    }
}

impl BattleState {
    pub(crate) const fn can_share_damage_rolls(&self, side: Side) -> bool {
        self.player.bide.is_none() && self.ai.bide.is_none() && !self.combatant(side).flags.has_any(CombatantFlags::SEEDED)
    }

    pub(crate) const fn clear_unused_state(&mut self) {
        if self.player.bide.is_none() && self.ai.bide.is_none() {
            self.last_damage = 0;
        }
        if self.ai_copied_move.is_some() && self.ai.charging_slot.is_none() && self.ai.rampage.is_none() {
            self.ai_copied_move = None;
        }
    }

    pub(crate) const fn combatant(&self, side: Side) -> &CombatantState {
        match side {
            Side::Player => &self.player,
            Side::Ai => &self.ai,
        }
    }

    pub(crate) const fn combatant_mut(&mut self, side: Side) -> &mut CombatantState {
        match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        }
    }

    pub(crate) const fn is_biding(&self, side: Side) -> bool {
        match side {
            Side::Player => self.player.bide.is_some(),
            Side::Ai => self.ai.bide.is_some(),
        }
    }

    pub(crate) const fn is_trapped(&self, side: Side) -> bool {
        match side {
            Side::Player => self.ai.trapping.is_some(),
            Side::Ai => self.player.trapping.is_some(),
        }
    }

    pub(crate) const fn miss_move(&mut self) {
        self.last_damage = 0;
    }

    pub(crate) const fn start_bide(&mut self, side: Side, bide: BideState) {
        let combatant = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        combatant.start_bide(bide);
    }

    pub(crate) const fn start_trapping(&mut self, side: Side, mv: &'static Move, damage: u16) {
        let attacker = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        attacker.start_trapping(mv, damage);
    }

    pub(crate) const fn winner(&self) -> Option<Side> {
        if self.player.hp == 0 {
            Some(Side::Ai)
        } else if self.ai.hp == 0 {
            Some(Side::Player)
        } else {
            None
        }
    }

    pub(crate) fn apply_damage(&mut self, dmg: u16, mv: &Move, side: Side, hits: u16) -> u16 {
        let (attacker, target) = match side {
            Side::Player => (&mut self.player, &mut self.ai),
            Side::Ai => (&mut self.ai, &mut self.player),
        };
        let dmg_dealt = dmg.min(target.hp);
        target.hp -= dmg_dealt;
        if dmg_dealt > 0 && !matches!(mv.effect, Effect::Explode) {
            target.build_rage(hits);
        }
        let recoil = mv.recoil_damage(dmg_dealt);
        attacker.hp = attacker.hp.saturating_sub(recoil);
        self.last_damage = dmg_dealt;
        dmg_dealt
    }

    pub(crate) fn apply_residual_damage(&mut self, side: Side, max_hp: u16, opponent_max_hp: u16) {
        let (combatant, opponent) = match side {
            Side::Player => (&mut self.player, &mut self.ai),
            Side::Ai => (&mut self.ai, &mut self.player),
        };
        let dmg = CombatantState::residual_damage(max_hp);
        if combatant.flags.has_any(CombatantFlags::BURNED | CombatantFlags::POISONED) {
            combatant.hp = combatant.hp.saturating_sub(dmg);
        }
        if combatant.flags.has_any(CombatantFlags::SEEDED) {
            combatant.hp = combatant.hp.saturating_sub(dmg);
            opponent.heal(dmg, opponent_max_hp);
        }
    }

    pub(crate) fn apply_secondary(&mut self, effect: &Effect, side: Side) {
        let target = match side {
            Side::Player => &mut self.ai,
            Side::Ai => &mut self.player,
        };
        effect.apply_secondary(target);
    }

    pub(crate) fn apply_status_move(&mut self, effect: &Effect, side: Side, target_types: &[Type; 2]) {
        let target = match (side, effect.targets_user()) {
            (Side::Player, true) | (Side::Ai, false) => &mut self.player,
            (Side::Ai, true) | (Side::Player, false) => &mut self.ai,
        };
        effect.apply_primary(target, target_types);
    }

    pub(crate) fn begin_move(&mut self, side: Side, slot: Option<usize>, effect: &Effect) {
        if side == Side::Player
            && let Some(idx) = slot
        {
            let pp = &mut self.player_pp[idx];
            *pp = if *pp == 0 && matches!(effect, Effect::HyperBeam) {
                63
            } else {
                pp.checked_sub(1).expect("The player's selected move must have PP remaining")
            };
        }
        if !effect.preserves_last_damage() {
            self.last_damage = 0;
        }
        if matches!(effect, Effect::Explode) {
            self.combatant_mut(side).hp = 0;
        }
        if matches!(effect, Effect::Disable | Effect::Explode) {
            let target = match side {
                Side::Player => &mut self.ai,
                Side::Ai => &mut self.player,
            };
            target.build_rage(1);
        }
    }

    pub(crate) fn cap_surplus_pp(&mut self, remaining_turns: usize) {
        let max_pp = if remaining_turns > 1 && (self.player.is_asleep() || self.player.bide.is_some() || self.player.trapping.is_some()) {
            remaining_turns - 1
        } else {
            remaining_turns
        };
        for pp in &mut self.player_pp {
            *pp = usize::from(*pp).min(max_pp) as u8;
        }
    }

    pub(crate) fn continue_bide(&mut self, side: Side) {
        let combatant = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        if let Some(damage) = combatant.continue_bide(self.last_damage) {
            self.apply_damage(damage, &BIDE, side, 1);
        }
    }

    pub(crate) fn continue_trapping(&mut self, side: Side) -> f64 {
        let attacker = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        let (mv, damage, expiry_prob) = attacker.continue_trapping();
        self.apply_damage(damage, mv, side, 1);
        expiry_prob
    }

    pub(crate) fn finish_turn(&mut self) {
        self.player.finish_turn();
        self.ai.finish_turn();
    }

    pub(crate) fn hurt_in_confusion(&mut self, side: Side, damage: u16) {
        self.last_damage = self.combatant_mut(side).hurt_in_confusion(damage);
    }

    pub(crate) fn key(&self) -> BattleKey {
        let battle_flags = self.player.recharge_slot.map_or(0, |slot| slot + 1)
            | (self.ai.recharge_slot.map_or(0, |slot| slot + 1) << 3u8)
            | (u8::from(self.player.mist) << 6u8)
            | (u8::from(self.ai.mist) << 7u8);
        let [pp1, pp2, pp3, pp4] = self.player_pp;
        let dmg_and_moves = self.last_damage
            | (self.last_player_move.map_or(0, |slot| u16::from(slot) + 1) << 10u8)
            | (self.ai_copied_move.map_or(0, |slot| u16::from(slot) + 1) << 13u8);
        let [dmg_low, dmg_high] = dmg_and_moves.to_le_bytes();
        BattleKey::new(
            CombatantKey::new(&self.player),
            CombatantKey::new(&self.ai),
            u64::from_le_bytes([pp1, pp2, pp3, pp4, dmg_low, dmg_high, self.ai_items_left, battle_flags]),
        )
    }

    pub(crate) fn new(player: &Combatant<'_>, ai: &Combatant<'_>, ai_items_left: u8) -> Self {
        Self {
            player: CombatantState::new(player.stats.hp),
            ai: CombatantState::new(ai.stats.hp),
            player_pp: player.pokemon.move_pp(),
            last_damage: 0,
            ai_items_left,
            last_player_move: None,
            ai_copied_move: None,
        }
    }

    pub(crate) fn player_selection_key(&self, has_ohko_speed: bool) -> u64 {
        let disabled_slot = self.player.disabled_slot();
        let mut available_moves = 0;
        for (slot, &pp) in self.player_pp.iter().enumerate() {
            if pp > 0 && disabled_slot != Some(slot as u8) {
                available_moves |= 1 << slot;
            }
        }
        let flags = available_moves
            | (u8::from(self.player.attack_halved()) << 4u8)
            | (u8::from(self.player.flags.has_any(CombatantFlags::FOCUS_ENERGY)) << 5u8)
            | (u8::from(self.ai.flags.has_any(CombatantFlags::SEEDED)) << 6u8)
            | (u8::from(self.ai.has_status_condition()) << 7u8);
        let [hp_low, hp_high] = self.ai.hp.to_le_bytes();
        let acc_and_eva = (self.player.stages.acc as u8 & 15) | ((self.ai.stages.eva as u8) << 4u8);
        let def_and_flags = (self.ai.stages.def as u8 & 15)
            | (u8::from(self.ai.reflect) << 4u8)
            | (u8::from(has_ohko_speed) << 5u8)
            | (u8::from(self.ai.flags.has_any(CombatantFlags::LIGHT_SCREEN)) << 6u8);
        u64::from_le_bytes([
            hp_low,
            hp_high,
            self.player.stages.atk as u8,
            self.player.stages.spc as u8,
            acc_and_eva,
            def_and_flags,
            self.ai.stages.spc as u8,
            flags,
        ])
    }

    pub(crate) fn use_ai_item(&mut self, item: &TrainerItem, ai_types: &[Type; 2], max_hp: u16) {
        if let Some(amount) = item.heal_amount() {
            self.ai.heal(amount, max_hp);
        } else {
            let effect = item.effect().expect("A trainer item without healing must have a battle effect");
            effect.apply_primary(&mut self.ai, ai_types);
        }
        self.ai_items_left -= 1;
    }

    pub(super) const fn from_key(player: CombatantState, ai: CombatantState, player_pp: [u8; 4], dmg_and_moves: u16, ai_items_left: u8) -> Self {
        Self {
            player,
            ai,
            player_pp,
            last_damage: dmg_and_moves & 0x03FF,
            ai_items_left,
            last_player_move: (((dmg_and_moves >> 10) & 7) as u8).checked_sub(1),
            ai_copied_move: ((dmg_and_moves >> 13) as u8).checked_sub(1),
        }
    }
}
