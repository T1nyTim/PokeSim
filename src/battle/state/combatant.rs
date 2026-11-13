use crate::{
    battle::state::{BideState, DisableState, RampageState, Sleep, StatStages, TrappingState},
    data::{DIG, Effect, Move, RAGE, REST, Type},
};

#[derive(Clone, Default, Eq, PartialEq)]
pub struct CombatantFlags(u16);

impl CombatantFlags {
    pub(crate) const FOCUS_ENERGY: u16 = 1 << 6;
    pub(crate) const LIGHT_SCREEN: u16 = 1 << 14;
    pub(crate) const UNDERGROUND: u16 = 1 << 7;
    pub(super) const BURNED: u16 = 1 << 3;
    pub(super) const POISONED: u16 = 1 << 0;
    pub(super) const SEEDED: u16 = 1 << 1;
    const PARALYZED: u16 = 1 << 5;
    const REST_ATK_PENALTY: u16 = 1 << 12;
    const REST_SPD_PENALTY: u16 = 1 << 13;

    pub(crate) const fn has_any(&self, mask: u16) -> bool {
        self.0 & mask != 0
    }

    pub(crate) const fn insert(&mut self, mask: u16) {
        self.0 |= mask;
    }

    pub(super) const fn from_key(flags: u8, spd_and_flags: u8) -> Self {
        let mask = Self::POISONED
            | Self::SEEDED
            | Self::BURNED
            | Self::PARALYZED
            | Self::FOCUS_ENERGY
            | Self::UNDERGROUND
            | Self::REST_ATK_PENALTY
            | Self::REST_SPD_PENALTY
            | Self::LIGHT_SCREEN;
        Self(u16::from_le_bytes([flags, spd_and_flags]) & mask)
    }

    pub(super) const fn key_bytes(&self) -> [u8; 2] {
        self.0.to_le_bytes()
    }

    const fn remove(&mut self, mask: u16) {
        self.0 &= !mask;
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct CombatantState {
    pub hp: u16,
    pub flags: CombatantFlags,
    pub stages: StatStages,
    pub bide: Option<BideState>,
    pub charging_slot: Option<u8>,
    pub confusion_elapsed: Option<u8>,
    pub disabled: Option<DisableState>,
    pub frozen: bool,
    pub mist: bool,
    pub rage_accuracy: Option<u8>,
    pub rampage: Option<RampageState>,
    pub recharge_slot: Option<u8>,
    pub reflect: bool,
    pub sleep: Option<Sleep>,
    pub trapping: Option<TrappingState>,
}

impl CombatantState {
    pub(crate) const fn attack_halved(&self) -> bool {
        self.flags.has_any(CombatantFlags::BURNED | CombatantFlags::REST_ATK_PENALTY)
    }

    pub(crate) const fn burn(&mut self) {
        self.flags.insert(CombatantFlags::BURNED);
    }

    pub(crate) const fn can_be_confused(&self) -> bool {
        self.hp > 0 && self.confusion_elapsed.is_none()
    }

    pub(crate) const fn can_be_disabled(&self) -> bool {
        self.hp > 0 && self.disabled.is_none()
    }

    pub(crate) const fn can_be_paralyzed(&self) -> bool {
        self.hp > 0 && !self.has_status_condition()
    }

    pub(crate) const fn can_be_put_to_sleep(&self) -> bool {
        self.hp > 0 && (self.recharge_slot.is_some() || !self.has_status_condition())
    }

    pub(crate) const fn confuse(&mut self) {
        if self.can_be_confused() {
            self.confusion_elapsed = Some(0);
        }
    }

    pub(crate) const fn disable(&mut self, disabled: DisableState) {
        self.disabled = Some(disabled);
    }

    pub(crate) const fn end_confusion(&mut self) {
        self.confusion_elapsed = None;
    }

    pub(crate) const fn end_disable(&mut self) {
        self.disabled = None;
    }

    pub(crate) const fn end_trapping(&mut self) {
        self.trapping.as_mut().expect("Ending a trapping move requires an active trap").end();
    }

    pub(crate) const fn fall_asleep(&mut self) {
        if self.recharge_slot.take().is_some() {
            self.flags
                .remove(CombatantFlags::BURNED | CombatantFlags::PARALYZED | CombatantFlags::POISONED);
            self.frozen = false;
        }
        self.sleep = Some(Sleep::Inflicted(0));
    }

    pub(crate) const fn finish_charging(&mut self) {
        self.charging_slot = None;
        self.flags.remove(CombatantFlags::UNDERGROUND);
    }

    pub(crate) const fn finish_rampage(&mut self) {
        self.rampage = None;
        self.confusion_elapsed = Some(0);
    }

    pub(crate) const fn full_paralysis_probability(&self) -> f64 {
        if self.flags.has_any(CombatantFlags::PARALYZED) {
            63.0 / 256.0
        } else {
            0.0
        }
    }

    pub(crate) const fn has_status_condition(&self) -> bool {
        self.flags
            .has_any(CombatantFlags::BURNED | CombatantFlags::PARALYZED | CombatantFlags::POISONED)
            || self.frozen
            || self.is_asleep()
    }

    pub(crate) const fn interrupt_move(&mut self) {
        self.bide = None;
        self.charging_slot = None;
        self.rampage = None;
        self.trapping = None;
    }

    pub(crate) const fn is_asleep(&self) -> bool {
        self.sleep.is_some()
    }

    pub(crate) const fn paralyze(&mut self) {
        self.flags.insert(CombatantFlags::PARALYZED);
    }

    pub(crate) const fn poison(&mut self) {
        self.flags.insert(CombatantFlags::POISONED);
    }

    pub(crate) const fn start_rampage(&mut self, slot: usize, accuracy: u8) {
        self.rampage = Some(RampageState::new(slot as u8, 1, accuracy));
    }

    pub(crate) const fn wake_up(&mut self) {
        self.sleep = None;
    }

    pub(crate) fn advance_confusion(&mut self) -> Option<f64> {
        #[expect(clippy::question_mark_used, reason = "handled before end of function")]
        let turns_elapsed = self.confusion_elapsed.as_mut()?;
        let probability = Effect::Confusion.duration_end_probability(*turns_elapsed);
        *turns_elapsed += 1;
        Some(probability)
    }

    pub(crate) fn advance_disable(&mut self) -> Option<f64> {
        self.disabled.as_mut().map(DisableState::advance)
    }

    pub(crate) fn advance_rampage(&mut self) -> Option<f64> {
        self.rampage.as_mut().map(|rampage| {
            rampage.turns_elapsed += 1;
            match rampage.turns_elapsed {
                2 => 0.0f64,
                3 => 0.5f64,
                4 => 1.0f64,
                _ => unreachable!("A rampage lasts three or four attacks"),
            }
        })
    }

    pub(crate) fn advance_sleep(&mut self) -> Option<f64> {
        #[expect(clippy::question_mark_used, reason = "handled before end of function")]
        let sleep = self.sleep.clone()?;
        Some(match sleep {
            Sleep::Inflicted(turns_elapsed) => {
                self.sleep = Some(Sleep::Inflicted(turns_elapsed + 1));
                Effect::Sleep.duration_end_probability(turns_elapsed)
            }
            Sleep::Resting => {
                self.sleep = Some(Sleep::WakingFromRest);
                0.0
            }
            Sleep::WakingFromRest => 1.0,
        })
    }

    pub(crate) fn can_be_burned(&self, types: &[Type; 2]) -> bool {
        self.hp > 0 && !self.has_status_condition() && !types.contains(&Type::Fire)
    }

    pub(crate) fn can_be_poisoned(&self, types: &[Type; 2]) -> bool {
        self.hp > 0 && !self.has_status_condition() && !types.contains(&Type::Poison)
    }

    pub(crate) fn can_be_seeded(&self, types: &[Type; 2]) -> bool {
        !self.flags.has_any(CombatantFlags::SEEDED) && !types.contains(&Type::Grass)
    }

    pub(crate) fn change_atk_stage(&mut self, change: i16) {
        let stage = (i16::from(self.stages.atk) + change).clamp(-6, 6) as i8;
        if stage != self.stages.atk {
            self.stages.atk = stage;
            self.flags.remove(CombatantFlags::REST_ATK_PENALTY);
        }
    }

    pub(crate) fn change_spd_stage(&mut self, change: i8) {
        let stage = (self.stages.spd + change).clamp(-6, 6);
        if stage != self.stages.spd {
            self.stages.spd = stage;
            self.flags.remove(CombatantFlags::REST_SPD_PENALTY);
        }
    }

    pub(crate) fn disabled_slot(&self) -> Option<u8> {
        self.disabled.as_ref().map(|disabled| disabled.slot)
    }

    pub(crate) fn heal(&mut self, amount: u16, max_hp: u16) {
        self.hp = self.hp.saturating_add(amount).min(max_hp);
    }

    pub(crate) fn heal_with_move(&mut self, max_hp: u16, mv: &Move) {
        let missing_hp = max_hp - self.hp;
        if missing_hp == 0 || (missing_hp as u8 == u8::MAX && max_hp as u8 != u8::MAX) {
            return;
        }
        if mv == &REST {
            self.hp = max_hp;
            if self.flags.has_any(CombatantFlags::BURNED) {
                self.flags.insert(CombatantFlags::REST_ATK_PENALTY);
            }
            if self.flags.has_any(CombatantFlags::PARALYZED) {
                self.flags.insert(CombatantFlags::REST_SPD_PENALTY);
            }
            self.flags
                .remove(CombatantFlags::BURNED | CombatantFlags::PARALYZED | CombatantFlags::POISONED);
            self.frozen = false;
            self.sleep = Some(Sleep::Resting);
        } else {
            self.heal(max_hp / 2, max_hp);
        }
    }

    pub(crate) fn is_disabled(&self, slot: usize) -> bool {
        self.disabled.as_ref().is_some_and(|disabled| disabled.slot == slot as u8)
    }

    pub(crate) fn physical_attack(&self, base_atk: u16) -> u32 {
        let attack = StatStages::apply(base_atk, self.stages.atk);
        if self.attack_halved() { (attack / 2).max(1) } else { attack }
    }

    pub(crate) fn rage_hit_probability(&mut self, target_eva: i8) -> f64 {
        let accuracy = self.rage_accuracy.unwrap_or_else(|| RAGE.accuracy());
        let threshold = self.stages.hit_threshold(accuracy, target_eva);
        if self.rage_accuracy.is_some() {
            self.rage_accuracy = Some(threshold);
        }
        f64::from(threshold) / 256.0
    }

    pub(crate) fn rampage_hit_probability(&mut self, target_eva: i8) -> f64 {
        let rampage = self.rampage.as_mut().expect("A rampage move must have started");
        rampage.accuracy = self.stages.hit_threshold(rampage.accuracy, target_eva);
        f64::from(rampage.accuracy) / 256.0
    }

    pub(crate) fn residual_damage(max_hp: u16) -> u16 {
        (max_hp / 16).max(1)
    }

    pub(crate) fn seed(&mut self, types: &[Type; 2]) {
        if self.can_be_seeded(types) {
            self.flags.insert(CombatantFlags::SEEDED);
        }
    }

    pub(crate) fn speed(&self, base_spd: u16) -> u32 {
        let speed = StatStages::apply(base_spd, self.stages.spd);
        if self.flags.has_any(CombatantFlags::PARALYZED | CombatantFlags::REST_SPD_PENALTY) {
            (speed / 4).max(1)
        } else {
            speed
        }
    }

    pub(crate) fn start_charging(&mut self, slot: usize, mv: &Move) {
        self.charging_slot = Some(slot as u8);
        if mv == &DIG {
            self.flags.insert(CombatantFlags::UNDERGROUND);
        }
    }

    pub(crate) fn start_rage(&mut self) {
        if self.rage_accuracy.is_none() {
            self.rage_accuracy = Some(StatStages::apply(u16::from(RAGE.accuracy()), self.stages.acc).min(255) as u8);
        }
    }

    pub(crate) fn trapping_move(&self) -> Option<&'static Move> {
        self.trapping.as_ref().map(|trapping| trapping.mv)
    }

    #[expect(clippy::too_many_arguments, reason = "to be looked at")]
    pub(super) const fn from_key(
        hp: u16,
        flags: CombatantFlags,
        stages: StatStages,
        bide: Option<BideState>,
        charging_slot: Option<u8>,
        confusion_elapsed: Option<u8>,
        disabled: Option<DisableState>,
        frozen: bool,
        mist: bool,
        rage_accuracy: Option<u8>,
        rampage: Option<RampageState>,
        recharge_slot: Option<u8>,
        reflect: bool,
        sleep: Option<Sleep>,
        trapping: Option<TrappingState>,
    ) -> Self {
        Self {
            hp,
            flags,
            stages,
            bide,
            charging_slot,
            confusion_elapsed,
            disabled,
            frozen,
            mist,
            rage_accuracy,
            rampage,
            recharge_slot,
            reflect,
            sleep,
            trapping,
        }
    }

    pub(super) const fn start_bide(&mut self, bide: BideState) {
        self.bide = Some(bide);
    }

    pub(super) const fn start_trapping(&mut self, mv: &'static Move, damage: u16) {
        self.trapping = Some(TrappingState::new(mv, damage));
    }

    pub(super) fn build_rage(&mut self, hits: u16) {
        if self.hp > 0 && self.rage_accuracy.is_some() {
            self.change_atk_stage(hits.cast_signed());
        }
    }

    #[expect(clippy::question_mark_used, reason = "handled before end of function")]
    pub(super) fn continue_bide(&mut self, last_damage: u16) -> Option<u16> {
        let damage = self.bide.as_mut()?.advance(last_damage)?;
        self.bide = None;
        Some(damage)
    }

    pub(super) fn continue_trapping(&mut self) -> (&'static Move, u16, f64) {
        self.trapping
            .as_mut()
            .expect("Continuing a trapping move requires an active trap")
            .advance()
    }

    pub(super) fn finish_turn(&mut self) {
        if self.trapping.as_ref().is_some_and(TrappingState::is_finished) {
            self.trapping = None;
        }
    }

    pub(super) fn hurt_in_confusion(&mut self, damage: u16) -> u16 {
        let dmg_dealt = damage.min(self.hp);
        self.hp -= dmg_dealt;
        self.interrupt_move();
        self.flags.remove(CombatantFlags::UNDERGROUND);
        dmg_dealt
    }

    pub(super) fn new(hp: u16) -> Self {
        Self {
            hp,
            flags: CombatantFlags::default(),
            stages: StatStages::default(),
            bide: None,
            charging_slot: None,
            confusion_elapsed: None,
            disabled: None,
            frozen: false,
            mist: false,
            rage_accuracy: None,
            rampage: None,
            recharge_slot: None,
            reflect: false,
            sleep: None,
            trapping: None,
        }
    }
}
