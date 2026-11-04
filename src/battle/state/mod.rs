pub(super) mod key;

use core::hash::{Hash, Hasher};

use crate::{
    battle::{
        Combatant, Side,
        state::key::{BattleKey, CombatantKey},
    },
    data::{Move, Type, effects::Effect, moves::BIDE},
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
    pub(super) fn new(player: &Combatant<'_>, ai: &Combatant<'_>, ai_items_left: u8) -> Self {
        Self {
            player: CombatantState::new(player.stats.hp),
            ai: CombatantState::new(ai.stats.hp),
            player_pp: player.pokemon.moves.map(|slot| slot.map_or(0, |mv| mv.pp)),
            last_damage: 0,
            ai_items_left,
        }
    }

    pub(super) const fn clear_unused_damage(&mut self) {
        if self.player.bide.is_none() && self.ai.bide.is_none() {
            self.last_damage = 0;
        }
    }

    pub(super) const fn combatant(&self, side: Side) -> &CombatantState {
        match side {
            Side::Player => &self.player,
            Side::Ai => &self.ai,
        }
    }

    pub(super) const fn combatant_mut(&mut self, side: Side) -> &mut CombatantState {
        match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        }
    }

    pub(super) const fn is_biding(&self, side: Side) -> bool {
        match side {
            Side::Player => self.player.bide.is_some(),
            Side::Ai => self.ai.bide.is_some(),
        }
    }

    pub(super) const fn is_trapped(&self, side: Side) -> bool {
        match side {
            Side::Player => self.ai.trapping.is_some(),
            Side::Ai => self.player.trapping.is_some(),
        }
    }

    pub(super) const fn miss_move(&mut self) {
        self.last_damage = 0;
    }

    pub(super) const fn start_bide(&mut self, side: Side, bide: BideState) {
        let combatant = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        combatant.start_bide(bide);
    }

    pub(super) const fn start_trapping(&mut self, side: Side, mv: &'static Move, damage: u16) {
        let attacker = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        attacker.start_trapping(mv, damage);
    }

    pub(super) const fn winner(&self) -> Option<Side> {
        if self.player.hp == 0 {
            Some(Side::Ai)
        } else if self.ai.hp == 0 {
            Some(Side::Player)
        } else {
            None
        }
    }

    pub(super) fn apply_damage(&mut self, dmg: u16, mv: &Move, side: Side) -> u16 {
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

    pub(super) fn apply_residual_damage(&mut self, side: Side, max_hp: u16, opponent_max_hp: u16) {
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

    pub(super) fn apply_secondary(&mut self, effect: &Effect, side: Side) {
        let target = match side {
            Side::Player => &mut self.ai,
            Side::Ai => &mut self.player,
        };
        effect.apply_secondary(target);
    }

    pub(super) fn apply_status_move(&mut self, effect: &Effect, side: Side, target_types: &[Type; 2]) {
        let target = match (side, effect.targets_user()) {
            (Side::Player, true) | (Side::Ai, false) => &mut self.player,
            (Side::Ai, true) | (Side::Player, false) => &mut self.ai,
        };
        effect.apply_primary(target, target_types);
    }

    pub(super) fn begin_move(&mut self, side: Side, slot: Option<usize>, effect: &Effect) {
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
    }

    pub(super) fn cap_surplus_pp(&mut self, remaining_turns: usize) {
        for pp in &mut self.player_pp {
            *pp = (*pp).min(remaining_turns as u8);
        }
    }

    pub(super) fn continue_bide(&mut self, side: Side) {
        let combatant = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        if let Some(damage) = combatant.continue_bide(self.last_damage) {
            self.apply_damage(damage, &BIDE, side);
        }
    }

    pub(super) fn continue_trapping(&mut self, side: Side) -> f64 {
        let attacker = match side {
            Side::Player => &mut self.player,
            Side::Ai => &mut self.ai,
        };
        let (mv, damage, expiry_prob) = attacker.continue_trapping();
        self.apply_damage(damage, mv, side);
        expiry_prob
    }

    pub(super) fn finish_turn(&mut self) {
        self.player.finish_turn();
        self.ai.finish_turn();
    }

    pub(super) fn hurt_in_confusion(&mut self, side: Side, damage: u16) {
        self.last_damage = self.combatant_mut(side).hurt_in_confusion(damage);
    }

    pub(super) fn key(&self) -> BattleKey {
        BattleKey::new(
            self.player.key(),
            self.ai.key(),
            u64::from(u32::from_le_bytes(self.player_pp)) | (u64::from(self.last_damage) << 32) | (u64::from(self.ai_items_left) << 48),
        )
    }

    pub(super) fn use_ai_item(&mut self, item: &TrainerItem) {
        item.effect.apply_status(&mut self.ai.stages);
        self.ai_items_left -= 1;
    }

    const fn from_key(player: CombatantState, ai: CombatantState, player_pp: [u8; 4], last_damage: u16, ai_items_left: u8) -> Self {
        Self {
            player,
            ai,
            player_pp,
            last_damage,
            ai_items_left,
        }
    }
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

#[derive(Clone, Eq, PartialEq)]
pub struct CombatantState {
    pub hp: u16,
    pub stages: StatStages,
    bide: Option<BideState>,
    pub burned: bool,
    confusion_elapsed: Option<u8>,
    disabled: Option<DisableState>,
    paralyzed: bool,
    poisoned: bool,
    seeded: bool,
    sleep_elapsed: Option<u8>,
    trapping: Option<TrappingState>,
}

impl CombatantState {
    pub(crate) const fn burn(&mut self) {
        self.burned = true;
    }

    pub(crate) const fn confuse(&mut self) {
        if self.can_be_confused() {
            self.confusion_elapsed = Some(0);
        }
    }

    pub(crate) const fn poison(&mut self) {
        self.poisoned = true;
    }

    pub(crate) const fn can_be_confused(&self) -> bool {
        self.hp > 0 && self.confusion_elapsed.is_none()
    }

    pub(crate) const fn can_be_paralyzed(&self) -> bool {
        self.hp > 0 && !self.has_status_condition()
    }

    pub(crate) const fn paralyze(&mut self) {
        self.paralyzed = true;
    }

    pub(crate) fn can_be_burned(&self, types: &[Type; 2]) -> bool {
        self.hp > 0 && !self.has_status_condition() && !types.contains(&Type::Fire)
    }

    pub(crate) fn can_be_poisoned(&self, types: &[Type; 2]) -> bool {
        self.hp > 0 && !self.has_status_condition() && !types.contains(&Type::Poison)
    }

    pub(crate) fn residual_damage(max_hp: u16) -> u16 {
        (max_hp / 16).max(1)
    }

    pub(crate) fn seed(&mut self, types: &[Type; 2]) {
        if self.can_be_seeded(types) {
            self.seeded = true;
        }
    }

    pub(super) const fn can_be_disabled(&self) -> bool {
        self.hp > 0 && self.disabled.is_none()
    }

    pub(super) const fn can_be_put_to_sleep(&self) -> bool {
        self.hp > 0 && !self.has_status_condition()
    }

    pub(super) const fn disable(&mut self, disabled: DisableState) {
        self.disabled = Some(disabled);
    }

    pub(super) const fn end_confusion(&mut self) {
        self.confusion_elapsed = None;
    }

    pub(super) const fn end_disable(&mut self) {
        self.disabled = None;
    }

    pub(super) const fn end_trapping(&mut self) {
        self.trapping.as_mut().expect("Ending a trapping move requires an active trap").end();
    }

    pub(super) const fn fall_asleep(&mut self) {
        self.sleep_elapsed = Some(0);
    }

    pub(super) const fn full_paralysis_probability(&self) -> f64 {
        if self.paralyzed { 63.0 / 256.0 } else { 0.0 }
    }

    pub(super) const fn has_status_condition(&self) -> bool {
        self.burned || self.paralyzed || self.poisoned || self.sleep_elapsed.is_some()
    }

    pub(super) const fn interrupt_move(&mut self) {
        self.bide = None;
        self.trapping = None;
    }

    pub(super) const fn is_asleep(&self) -> bool {
        self.sleep_elapsed.is_some()
    }

    pub(super) const fn wake_up(&mut self) {
        self.sleep_elapsed = None;
    }

    pub(super) fn advance_confusion(&mut self) -> Option<f64> {
        #[expect(clippy::question_mark_used, reason = "handled before end of function")]
        let turns_elapsed = self.confusion_elapsed.as_mut()?;
        let probability = Effect::Confusion.duration_end_probability(*turns_elapsed);
        *turns_elapsed += 1;
        Some(probability)
    }

    pub(super) fn advance_disable(&mut self) -> Option<f64> {
        self.disabled.as_mut().map(DisableState::advance)
    }

    pub(super) fn advance_sleep(&mut self) -> Option<f64> {
        #[expect(clippy::question_mark_used, reason = "handled before end of function")]
        let turns_elapsed = self.sleep_elapsed.as_mut()?;
        let probability = Effect::Sleep.duration_end_probability(*turns_elapsed);
        *turns_elapsed += 1;
        Some(probability)
    }

    pub(super) fn can_be_seeded(&self, types: &[Type; 2]) -> bool {
        !self.seeded && !types.contains(&Type::Grass)
    }

    pub(super) fn disabled_slot(&self) -> Option<u8> {
        self.disabled.as_ref().map(|disabled| disabled.slot)
    }

    pub(super) fn heal(&mut self, amount: u16, max_hp: u16) {
        self.hp = self.hp.saturating_add(amount).min(max_hp);
    }

    pub(super) fn is_disabled(&self, slot: usize) -> bool {
        self.disabled.as_ref().is_some_and(|disabled| disabled.slot == slot as u8)
    }

    pub(super) fn physical_attack(&self, base_atk: u16) -> u32 {
        let attack = StatStages::apply(base_atk, self.stages.atk);
        if self.burned { (attack / 2).max(1) } else { attack }
    }

    pub(super) fn speed(&self, base_spd: u16) -> u32 {
        let speed = StatStages::apply(base_spd, self.stages.spd);
        if self.paralyzed { (speed / 4).max(1) } else { speed }
    }

    pub(super) fn trapping_move(&self) -> Option<&'static Move> {
        self.trapping.as_ref().map(|trapping| trapping.mv)
    }

    #[expect(clippy::too_many_arguments, reason = "to be looked at")]
    #[expect(clippy::fn_params_excessive_bools, reason = "to be looked at")]
    const fn from_key(
        hp: u16,
        stages: StatStages,
        bide: Option<BideState>,
        burned: bool,
        confusion_elapsed: Option<u8>,
        disabled: Option<DisableState>,
        paralyzed: bool,
        poisoned: bool,
        seeded: bool,
        sleep_elapsed: Option<u8>,
        trapping: Option<TrappingState>,
    ) -> Self {
        Self {
            hp,
            stages,
            bide,
            burned,
            confusion_elapsed,
            disabled,
            paralyzed,
            poisoned,
            seeded,
            sleep_elapsed,
            trapping,
        }
    }

    const fn start_bide(&mut self, bide: BideState) {
        self.bide = Some(bide);
    }

    const fn start_trapping(&mut self, mv: &'static Move, damage: u16) {
        self.trapping = Some(TrappingState::new(mv, damage));
    }

    #[expect(clippy::question_mark_used, reason = "handled before end of function")]
    fn continue_bide(&mut self, last_damage: u16) -> Option<u16> {
        let damage = self.bide.as_mut()?.advance(last_damage)?;
        self.bide = None;
        Some(damage)
    }

    fn continue_trapping(&mut self) -> (&'static Move, u16, f64) {
        self.trapping
            .as_mut()
            .expect("Continuing a trapping move requires an active trap")
            .advance()
    }

    fn finish_turn(&mut self) {
        if self.trapping.as_ref().is_some_and(TrappingState::is_finished) {
            self.trapping = None;
        }
    }

    fn hurt_in_confusion(&mut self, damage: u16) -> u16 {
        let dmg_dealt = damage.min(self.hp);
        self.hp -= dmg_dealt;
        self.interrupt_move();
        dmg_dealt
    }

    fn key(&self) -> CombatantKey {
        let [hp_low, hp_high] = self.hp.to_le_bytes();
        let flags = u8::from(self.poisoned)
            | (u8::from(self.seeded) << 1u8)
            | (u8::from(self.bide.is_some()) << 2u8)
            | (u8::from(self.burned) << 3u8)
            | (u8::from(self.trapping.is_some()) << 4u8)
            | (u8::from(self.paralyzed) << 5u8);
        let present =
            u8::from(self.sleep_elapsed.is_some()) | (u8::from(self.confusion_elapsed.is_some()) << 1u8) | (u8::from(self.disabled.is_some()) << 2u8);
        let bide = self.bide.clone().unwrap_or(BideState::new(0));
        let [damage_low, damage_high] = bide.damage.to_le_bytes();
        CombatantKey::new(
            u64::from_le_bytes([
                hp_low,
                hp_high,
                self.stages.atk as u8,
                self.stages.def as u8,
                self.stages.spd as u8,
                self.stages.spc as u8,
                self.stages.acc as u8,
                flags,
            ]),
            u64::from_le_bytes([
                self.sleep_elapsed.unwrap_or(u8::MAX),
                self.confusion_elapsed.unwrap_or(u8::MAX),
                self.disabled.clone().map_or(u8::MAX, |disabled| disabled.slot),
                self.disabled.clone().map_or(u8::MAX, |disabled| disabled.turns_elapsed),
                bide.turns_left,
                damage_low,
                damage_high,
                present,
            ]),
            self.trapping.clone(),
        )
    }

    fn new(hp: u16) -> Self {
        Self {
            hp,
            stages: StatStages::default(),
            bide: None,
            burned: false,
            confusion_elapsed: None,
            disabled: None,
            paralyzed: false,
            poisoned: false,
            seeded: false,
            sleep_elapsed: None,
            trapping: None,
        }
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

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct StatStages {
    pub atk: i8,
    pub def: i8,
    pub spd: i8,
    pub spc: i8,
    pub acc: i8,
}

impl StatStages {
    pub(super) fn apply(stat: u16, stage: i8) -> u32 {
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
        u32::from(stat * numerator / denominator).clamp(1, 999)
    }

    pub(super) fn hit_probability(&self, mv: &Move) -> f64 {
        let threshold = Self::apply(u16::from(mv.accuracy()), self.acc).min(255);
        f64::from(threshold) / 256.0
    }

    const fn new(atk: i8, def: i8, spd: i8, spc: i8, acc: i8) -> Self {
        Self { atk, def, spd, spc, acc }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct TrappingState {
    mv: &'static Move,
    damage: u16,
    turns_elapsed: u8,
    ends_this_turn: bool,
}

impl TrappingState {
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
