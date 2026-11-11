pub(super) mod accumulator;
pub(super) mod battle;
pub(super) mod key;

use core::hash::Hash;

use crate::data::{
    Type,
    effects::Effect,
    move_data::Move,
    moves::{DIG, RAGE, REST},
};

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
    pub charging_slot: Option<u8>,
    confusion_elapsed: Option<u8>,
    disabled: Option<DisableState>,
    pub focus_energy: bool,
    pub frozen: bool,
    pub light_screen: bool,
    pub mist: bool,
    paralyzed: bool,
    poisoned: bool,
    pub rage_accuracy: Option<u8>,
    pub rampage: Option<RampageState>,
    pub recharge_slot: Option<u8>,
    pub reflect: bool,
    rest_atk_penalty: bool,
    rest_spd_penalty: bool,
    seeded: bool,
    sleep: Option<Sleep>,
    trapping: Option<TrappingState>,
    pub underground: bool,
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

    pub(crate) const fn has_status_condition(&self) -> bool {
        self.burned || self.frozen || self.paralyzed || self.poisoned || self.is_asleep()
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

    pub(crate) fn change_atk_stage(&mut self, change: i16) {
        let stage = (i16::from(self.stages.atk) + change).clamp(-6, 6) as i8;
        if stage != self.stages.atk {
            self.stages.atk = stage;
            self.rest_atk_penalty = false;
        }
    }

    pub(crate) fn change_spd_stage(&mut self, change: i8) {
        let stage = (self.stages.spd + change).clamp(-6, 6);
        if stage != self.stages.spd {
            self.stages.spd = stage;
            self.rest_spd_penalty = false;
        }
    }

    pub(crate) fn residual_damage(max_hp: u16) -> u16 {
        (max_hp / 16).max(1)
    }

    pub(crate) fn seed(&mut self, types: &[Type; 2]) {
        if self.can_be_seeded(types) {
            self.seeded = true;
        }
    }

    pub(super) const fn attack_halved(&self) -> bool {
        self.burned || self.rest_atk_penalty
    }

    pub(super) const fn can_be_disabled(&self) -> bool {
        self.hp > 0 && self.disabled.is_none()
    }

    pub(super) const fn can_be_put_to_sleep(&self) -> bool {
        self.hp > 0 && (self.recharge_slot.is_some() || !self.has_status_condition())
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
        if self.recharge_slot.take().is_some() {
            self.burned = false;
            self.frozen = false;
            self.paralyzed = false;
            self.poisoned = false;
        }
        self.sleep = Some(Sleep::Inflicted(0));
    }

    pub(super) const fn finish_charging(&mut self) {
        self.charging_slot = None;
        self.underground = false;
    }

    pub(super) const fn finish_rampage(&mut self) {
        self.rampage = None;
        self.confusion_elapsed = Some(0);
    }

    pub(super) const fn full_paralysis_probability(&self) -> f64 {
        if self.paralyzed { 63.0 / 256.0 } else { 0.0 }
    }

    pub(super) const fn interrupt_move(&mut self) {
        self.bide = None;
        self.charging_slot = None;
        self.rampage = None;
        self.trapping = None;
    }

    pub(super) const fn is_asleep(&self) -> bool {
        self.sleep.is_some()
    }

    pub(super) const fn start_rampage(&mut self, slot: usize, accuracy: u8) {
        self.rampage = Some(RampageState::new(slot as u8, 1, accuracy));
    }

    pub(super) const fn wake_up(&mut self) {
        self.sleep = None;
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

    pub(super) fn advance_rampage(&mut self) -> Option<f64> {
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

    pub(super) fn advance_sleep(&mut self) -> Option<f64> {
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

    pub(super) fn can_be_seeded(&self, types: &[Type; 2]) -> bool {
        !self.seeded && !types.contains(&Type::Grass)
    }

    pub(super) fn disabled_slot(&self) -> Option<u8> {
        self.disabled.as_ref().map(|disabled| disabled.slot)
    }

    pub(super) fn heal(&mut self, amount: u16, max_hp: u16) {
        self.hp = self.hp.saturating_add(amount).min(max_hp);
    }

    pub(super) fn heal_with_move(&mut self, max_hp: u16, mv: &Move) {
        let missing_hp = max_hp - self.hp;
        if missing_hp == 0 || (missing_hp as u8 == u8::MAX && max_hp as u8 != u8::MAX) {
            return;
        }
        if mv == &REST {
            self.hp = max_hp;
            self.rest_atk_penalty |= self.burned;
            self.rest_spd_penalty |= self.paralyzed;
            self.burned = false;
            self.frozen = false;
            self.paralyzed = false;
            self.poisoned = false;
            self.sleep = Some(Sleep::Resting);
        } else {
            self.heal(max_hp / 2, max_hp);
        }
    }

    pub(super) fn is_disabled(&self, slot: usize) -> bool {
        self.disabled.as_ref().is_some_and(|disabled| disabled.slot == slot as u8)
    }

    pub(super) fn physical_attack(&self, base_atk: u16) -> u32 {
        let attack = StatStages::apply(base_atk, self.stages.atk);
        if self.attack_halved() { (attack / 2).max(1) } else { attack }
    }

    pub(super) fn rage_hit_probability(&mut self, target_eva: i8) -> f64 {
        let accuracy = self.rage_accuracy.unwrap_or_else(|| RAGE.accuracy());
        let threshold = self.stages.hit_threshold(accuracy, target_eva);
        if self.rage_accuracy.is_some() {
            self.rage_accuracy = Some(threshold);
        }
        f64::from(threshold) / 256.0
    }

    pub(super) fn rampage_hit_probability(&mut self, target_eva: i8) -> f64 {
        let rampage = self.rampage.as_mut().expect("A rampage move must have started");
        rampage.accuracy = self.stages.hit_threshold(rampage.accuracy, target_eva);
        f64::from(rampage.accuracy) / 256.0
    }

    pub(super) fn speed(&self, base_spd: u16) -> u32 {
        let speed = StatStages::apply(base_spd, self.stages.spd);
        if self.paralyzed || self.rest_spd_penalty {
            (speed / 4).max(1)
        } else {
            speed
        }
    }

    pub(super) fn start_charging(&mut self, slot: usize, mv: &Move) {
        self.charging_slot = Some(slot as u8);
        self.underground |= mv == &DIG;
    }

    pub(super) fn start_rage(&mut self) {
        if self.rage_accuracy.is_none() {
            self.rage_accuracy = Some(StatStages::apply(u16::from(RAGE.accuracy()), self.stages.acc).min(255) as u8);
        }
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
        charging_slot: Option<u8>,
        confusion_elapsed: Option<u8>,
        disabled: Option<DisableState>,
        focus_energy: bool,
        frozen: bool,
        light_screen: bool,
        mist: bool,
        paralyzed: bool,
        poisoned: bool,
        rage_accuracy: Option<u8>,
        rampage: Option<RampageState>,
        recharge_slot: Option<u8>,
        reflect: bool,
        rest_atk_penalty: bool,
        rest_spd_penalty: bool,
        seeded: bool,
        sleep: Option<Sleep>,
        trapping: Option<TrappingState>,
        underground: bool,
    ) -> Self {
        Self {
            hp,
            stages,
            bide,
            burned,
            charging_slot,
            confusion_elapsed,
            disabled,
            focus_energy,
            frozen,
            light_screen,
            mist,
            paralyzed,
            poisoned,
            rage_accuracy,
            rampage,
            recharge_slot,
            reflect,
            rest_atk_penalty,
            rest_spd_penalty,
            seeded,
            sleep,
            trapping,
            underground,
        }
    }

    const fn start_bide(&mut self, bide: BideState) {
        self.bide = Some(bide);
    }

    const fn start_trapping(&mut self, mv: &'static Move, damage: u16) {
        self.trapping = Some(TrappingState::new(mv, damage));
    }

    fn build_rage(&mut self, hits: u16) {
        if self.hp > 0 && self.rage_accuracy.is_some() {
            self.change_atk_stage(hits.cast_signed());
        }
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
        self.underground = false;
        dmg_dealt
    }

    fn new(hp: u16) -> Self {
        Self {
            hp,
            stages: StatStages::default(),
            bide: None,
            burned: false,
            charging_slot: None,
            confusion_elapsed: None,
            disabled: None,
            focus_energy: false,
            frozen: false,
            light_screen: false,
            mist: false,
            paralyzed: false,
            poisoned: false,
            rage_accuracy: None,
            rampage: None,
            recharge_slot: None,
            reflect: false,
            rest_atk_penalty: false,
            rest_spd_penalty: false,
            seeded: false,
            sleep: None,
            trapping: None,
            underground: false,
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

#[derive(Clone, Eq, PartialEq)]
enum Sleep {
    Inflicted(u8),
    Resting,
    WakingFromRest,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct TrappingState {
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
