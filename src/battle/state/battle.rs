use core::hash::{Hash, Hasher};

use crate::{
    battle::{
        Side,
        combatant::Combatant,
        state::{
            BideState, CombatantState,
            key::{BattleKey, CombatantKey},
        },
    },
    data::{Type, effects::Effect, move_data::Move, moves::BIDE},
    journey::TrainerItem,
};

#[derive(Clone, Eq, PartialEq)]
pub struct BattleState {
    pub player: CombatantState,
    pub ai: CombatantState,
    pub player_pp: [u8; 4],
    last_damage: u16,
    pub ai_items_left: u8,
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
    pub(crate) const fn clear_unused_damage(&mut self) {
        if self.player.bide.is_none() && self.ai.bide.is_none() {
            self.last_damage = 0;
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

    pub(crate) fn apply_damage(&mut self, dmg: u16, mv: &Move, side: Side) -> u16 {
        let (attacker, target) = match side {
            Side::Player => (&mut self.player, &mut self.ai),
            Side::Ai => (&mut self.ai, &mut self.player),
        };
        let dmg_dealt = dmg.min(target.hp);
        target.hp -= dmg_dealt;
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
        if combatant.burned || combatant.poisoned {
            combatant.hp = combatant.hp.saturating_sub(dmg);
        }
        if combatant.seeded {
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
            self.player_pp[idx] = self.player_pp[idx]
                .checked_sub(1)
                .expect("The player's selected move must have PP remaining");
        }
        if !effect.preserves_last_damage() {
            self.last_damage = 0;
        }
        if matches!(effect, Effect::Explode) {
            self.combatant_mut(side).hp = 0;
        }
    }

    pub(crate) fn cap_surplus_pp(&mut self, remaining_turns: usize) {
        let max_pp = if remaining_turns > 1 && (self.player.is_asleep() || self.player.bide.is_some() || self.player.trapping.is_some()) {
            remaining_turns - 1
        } else {
            remaining_turns
        };
        for pp in &mut self.player_pp {
            *pp = (*pp).min(max_pp as u8);
        }
    }

    pub(crate) fn continue_bide(&mut self, side: Side) {
        let combatant = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        if let Some(damage) = combatant.continue_bide(self.last_damage) {
            self.apply_damage(damage, &BIDE, side);
        }
    }

    pub(crate) fn continue_trapping(&mut self, side: Side) -> f64 {
        let attacker = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        let (mv, damage, expiry_prob) = attacker.continue_trapping();
        self.apply_damage(damage, mv, side);
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
        BattleKey::new(
            CombatantKey::new(&self.player),
            CombatantKey::new(&self.ai),
            u64::from(u32::from_le_bytes(self.player_pp)) | (u64::from(self.last_damage) << 32) | (u64::from(self.ai_items_left) << 48),
        )
    }

    pub(crate) fn new(player: &Combatant<'_>, ai: &Combatant<'_>, ai_items_left: u8) -> Self {
        Self {
            player: CombatantState::new(player.stats.hp),
            ai: CombatantState::new(ai.stats.hp),
            player_pp: player.pokemon.moves.map(|slot| slot.map_or(0, |mv| mv.pp)),
            last_damage: 0,
            ai_items_left,
        }
    }

    pub(crate) fn player_selection_key(&self) -> u64 {
        let disabled_slot = self.player.disabled_slot();
        let mut available_moves = 0;
        for (slot, &pp) in self.player_pp.iter().enumerate() {
            if pp > 0 && disabled_slot != Some(slot as u8) {
                available_moves |= 1 << slot;
            }
        }
        let flags = available_moves
            | (u8::from(self.player.burned) << 4u8)
            | (u8::from(self.player.focus_energy) << 5u8)
            | (u8::from(self.ai.seeded) << 6u8)
            | (u8::from(self.ai.has_status_condition()) << 7u8);
        let [hp_low, hp_high] = self.ai.hp.to_le_bytes();
        u64::from_le_bytes([
            hp_low,
            hp_high,
            self.player.stages.atk as u8,
            self.player.stages.spc as u8,
            self.player.stages.acc as u8,
            self.ai.stages.def as u8,
            self.ai.stages.spc as u8,
            flags,
        ])
    }

    pub(crate) fn use_ai_item(&mut self, item: &TrainerItem) {
        item.effect.apply_status(&mut self.ai.stages);
        self.ai_items_left -= 1;
    }

    pub(super) const fn from_key(player: CombatantState, ai: CombatantState, player_pp: [u8; 4], last_damage: u16, ai_items_left: u8) -> Self {
        Self {
            player,
            ai,
            player_pp,
            last_damage,
            ai_items_left,
        }
    }
}
