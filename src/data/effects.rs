use core::array;

use crate::{
    battle::state::{BideState, CombatantState, DisableState, StatStages},
    data::Type,
};

#[derive(Eq, Hash, PartialEq)]
pub enum Effect {
    #[expect(clippy::enum_variant_names, reason = "Matching Poke Decomp")]
    NoEffect,
    AccuracyDown,
    AttackDown,
    AttackDownSide,
    AttackTwice,
    AttackUp1,
    AttackUp2,
    Bide,
    BurnSide1,
    BurnSide2,
    Charge,
    Confusion,
    ConfusionSide,
    DefenseDown1,
    DefenseDown2,
    DefenseDownSide,
    DefenseUp1,
    DefenseUp2,
    Disable,
    DrainHp,
    DreamEater,
    EvasionUp,
    Explode,
    Flinch1,
    Flinch2,
    FocusEnergy,
    FreezeSide,
    Haze,
    Heal,
    HyperBeam,
    LeechSeed,
    LightScreen,
    Metronome,
    Mimic,
    MirrorMove,
    #[expect(clippy::upper_case_acronyms, reason = "is an acronym")]
    OHKO,
    Paralyze,
    ParalyzeSide1,
    ParalyzeSide2,
    PayDay,
    Poison,
    PoisonSide1,
    PoisonSide2,
    Rage,
    Recoil,
    Reflect,
    Sleep,
    SpecialDamage,
    SpecialDownSide,
    SpecialUp1,
    SpecialUp2,
    SpeedDown,
    SpeedDownSide,
    SpeedUp,
    Substitute,
    SuperFang,
    Swift,
    SwitchAndTeleport,
    ThrashPetalDance,
    Trapping,
    Twineedle,
    TwoToFiveAttacks,
}

impl Effect {
    pub(crate) const fn damage_per_turn(&self, damage: f64) -> f64 {
        if matches!(self, Self::Charge) { damage * 0.5 } else { damage }
    }

    pub(crate) const fn encouraged_by_ai(&self) -> bool {
        matches!(
            self,
            Self::AccuracyDown
                | Self::AttackUp1
                | Self::AttackUp2
                | Self::AttackDown
                // | Self::Conversion
                | Self::DefenseDown1
                | Self::DefenseDown2
                | Self::DefenseUp1
                | Self::DefenseUp2
                | Self::EvasionUp
                | Self::Haze
                | Self::Heal
                | Self::LightScreen
                | Self::PayDay
                | Self::Reflect
                | Self::SpecialUp1
                | Self::SpecialUp2
                | Self::SpeedDown
                | Self::SpeedUp
                | Self::Swift // | Self::Transform
        )
    }

    pub(crate) const fn hit_count_outcomes(&self) -> &'static [(u16, f64)] {
        if matches!(self, Self::TwoToFiveAttacks) {
            &[(2, 0.375), (3, 0.375), (4, 0.125), (5, 0.125)]
        } else {
            &[(1, 1.0)]
        }
    }

    pub(crate) const fn inflicts_status_condition(&self) -> bool {
        matches!(
            self,
            Self::BurnSide1
                | Self::BurnSide2
                | Self::FreezeSide
                | Self::Paralyze
                | Self::ParalyzeSide1
                | Self::ParalyzeSide2
                | Self::Poison
                | Self::PoisonSide1
                | Self::PoisonSide2
                | Self::Sleep
        )
    }

    pub(crate) const fn preserves_last_damage(&self) -> bool {
        matches!(
            self,
            Self::Confusion | Self::FocusEnergy | Self::LeechSeed | Self::Paralyze | Self::Poison
        )
    }

    pub(crate) const fn resolution(&self) -> EffectResolution {
        match *self {
            Self::Bide => EffectResolution::Bide,
            Self::Charge => EffectResolution::Charge,
            Self::Mimic | Self::MirrorMove => EffectResolution::CopyMove,
            Self::AttackDownSide
            | Self::BurnSide1
            | Self::BurnSide2
            | Self::ConfusionSide
            | Self::DefenseDownSide
            | Self::DrainHp
            | Self::Explode
            | Self::Flinch1
            | Self::Flinch2
            | Self::NoEffect
            | Self::ParalyzeSide1
            | Self::ParalyzeSide2
            | Self::PayDay
            | Self::PoisonSide1
            | Self::PoisonSide2
            | Self::Rage
            | Self::Recoil
            | Self::SpecialDownSide
            | Self::SpeedDownSide
            | Self::Swift
            | Self::Trapping
            | Self::Twineedle
            | Self::TwoToFiveAttacks => EffectResolution::Damage,
            Self::Disable => EffectResolution::Disable,
            Self::SpecialDamage | Self::SuperFang => EffectResolution::FixedDamage,
            Self::Sleep => EffectResolution::Sleep,
            Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackUp1
            | Self::AttackUp2
            | Self::Confusion
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseUp1
            | Self::DefenseUp2
            | Self::FocusEnergy
            | Self::LeechSeed
            | Self::Paralyze
            | Self::Poison
            | Self::SpecialUp1
            | Self::SpecialUp2
            | Self::SpeedDown
            | Self::SpeedUp => EffectResolution::Status,
            Self::SwitchAndTeleport => EffectResolution::SwitchOrEscape,
            Self::AttackTwice
            | Self::DreamEater
            | Self::EvasionUp
            | Self::FreezeSide
            | Self::Haze
            | Self::Heal
            | Self::HyperBeam
            | Self::LightScreen
            | Self::Metronome
            | Self::OHKO
            | Self::Reflect
            | Self::Substitute
            | Self::ThrashPetalDance => todo!(),
        }
    }

    pub(crate) const fn targets_user(&self) -> bool {
        matches!(
            self,
            Self::AttackUp1
                | Self::AttackUp2
                | Self::Bide
                | Self::DefenseUp1
                | Self::DefenseUp2
                | Self::EvasionUp
                | Self::FocusEnergy
                | Self::Heal
                | Self::LightScreen
                | Self::Reflect
                | Self::SpecialUp1
                | Self::SpecialUp2
                | Self::SpeedUp
                | Self::Substitute
        )
    }

    pub(crate) fn apply_primary(&self, target: &mut CombatantState, target_types: &[Type; 2]) {
        match self {
            Self::Confusion => target.confuse(),
            Self::FocusEnergy => target.focus_energy = true,
            Self::LeechSeed => target.seed(target_types),
            Self::Paralyze => {
                if target.can_be_paralyzed() {
                    target.paralyze();
                }
            }
            Self::Poison => {
                if target.can_be_poisoned(target_types) {
                    target.poison();
                }
            }
            Self::AttackDownSide
            | Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackTwice
            | Self::AttackUp1
            | Self::AttackUp2
            | Self::Bide
            | Self::BurnSide1
            | Self::BurnSide2
            | Self::Charge
            | Self::ConfusionSide
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseDownSide
            | Self::DefenseUp1
            | Self::DefenseUp2
            | Self::Disable
            | Self::DrainHp
            | Self::DreamEater
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::Flinch2
            | Self::FreezeSide
            | Self::Haze
            | Self::Heal
            | Self::HyperBeam
            | Self::LightScreen
            | Self::Metronome
            | Self::MirrorMove
            | Self::Mimic
            | Self::NoEffect
            | Self::OHKO
            | Self::ParalyzeSide1
            | Self::ParalyzeSide2
            | Self::PayDay
            | Self::PoisonSide1
            | Self::PoisonSide2
            | Self::Rage
            | Self::Recoil
            | Self::Reflect
            | Self::Sleep
            | Self::SpecialDamage
            | Self::SpecialDownSide
            | Self::SpecialUp1
            | Self::SpecialUp2
            | Self::SpeedDown
            | Self::SpeedDownSide
            | Self::SpeedUp
            | Self::Substitute
            | Self::SuperFang
            | Self::Swift
            | Self::SwitchAndTeleport
            | Self::ThrashPetalDance
            | Self::Trapping
            | Self::Twineedle
            | Self::TwoToFiveAttacks => self.apply_status(&mut target.stages),
        }
    }

    pub(crate) fn apply_secondary(&self, target: &mut CombatantState) {
        match *self {
            Self::AttackDownSide | Self::DefenseDownSide | Self::SpecialDownSide | Self::SpeedDownSide => self.apply_status(&mut target.stages),
            Self::BurnSide1 | Self::BurnSide2 => target.burn(),
            Self::ConfusionSide => target.confuse(),
            Self::ParalyzeSide1 | Self::ParalyzeSide2 => target.paralyze(),
            Self::PoisonSide1 | Self::PoisonSide2 => target.poison(),
            Self::NoEffect
            | Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackTwice
            | Self::AttackUp1
            | Self::AttackUp2
            | Self::Bide
            | Self::Charge
            | Self::Confusion
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseUp1
            | Self::DefenseUp2
            | Self::Disable
            | Self::DrainHp
            | Self::DreamEater
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::Flinch2
            | Self::FocusEnergy
            | Self::FreezeSide
            | Self::Haze
            | Self::Heal
            | Self::HyperBeam
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::Mimic
            | Self::MirrorMove
            | Self::OHKO
            | Self::Paralyze
            | Self::PayDay
            | Self::Poison
            | Self::Rage
            | Self::Recoil
            | Self::Reflect
            | Self::Sleep
            | Self::SpecialDamage
            | Self::SpecialUp1
            | Self::SpecialUp2
            | Self::SpeedDown
            | Self::SpeedUp
            | Self::Substitute
            | Self::SuperFang
            | Self::Swift
            | Self::SwitchAndTeleport
            | Self::ThrashPetalDance
            | Self::Trapping
            | Self::Twineedle
            | Self::TwoToFiveAttacks => unreachable!("Only supported secondary effects can be applied"),
        }
    }

    pub(crate) fn apply_status(&self, stages: &mut StatStages) {
        match *self {
            Self::AccuracyDown => stages.acc = (stages.acc - 1).max(-6),
            Self::AttackDown | Self::AttackDownSide => stages.atk = (stages.atk - 1).max(-6),
            Self::AttackUp1 => stages.atk = (stages.atk + 1).min(6),
            Self::AttackUp2 => stages.atk = (stages.atk + 2).min(6),
            Self::DefenseDown1 | Self::DefenseDownSide => stages.def = (stages.def - 1).max(-6),
            Self::DefenseDown2 => stages.def = (stages.def - 2).max(-6),
            Self::DefenseUp1 => stages.def = (stages.def + 1).min(6),
            Self::DefenseUp2 => stages.def = (stages.def + 2).min(6),
            Self::SpecialDownSide => stages.spc = (stages.spc - 1).max(-6),
            Self::SpecialUp1 => stages.spc = (stages.spc + 1).min(6),
            Self::SpecialUp2 => stages.spc = (stages.spc + 2).min(6),
            Self::SpeedDown | Self::SpeedDownSide => stages.spd = (stages.spd - 1).max(-6),
            Self::SpeedUp => stages.spd = (stages.spd + 1).min(6),
            Self::AttackTwice
            | Self::Bide
            | Self::BurnSide1
            | Self::BurnSide2
            | Self::Charge
            | Self::Confusion
            | Self::ConfusionSide
            | Self::Disable
            | Self::DrainHp
            | Self::DreamEater
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::Flinch2
            | Self::FocusEnergy
            | Self::FreezeSide
            | Self::Haze
            | Self::Heal
            | Self::HyperBeam
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::Mimic
            | Self::MirrorMove
            | Self::NoEffect
            | Self::OHKO
            | Self::Paralyze
            | Self::ParalyzeSide1
            | Self::ParalyzeSide2
            | Self::PayDay
            | Self::Poison
            | Self::PoisonSide1
            | Self::PoisonSide2
            | Self::Rage
            | Self::Recoil
            | Self::Reflect
            | Self::Sleep
            | Self::SpecialDamage
            | Self::Substitute
            | Self::SuperFang
            | Self::Swift
            | Self::SwitchAndTeleport
            | Self::ThrashPetalDance
            | Self::Trapping
            | Self::Twineedle
            | Self::TwoToFiveAttacks => unreachable!(),
        }
    }

    pub(crate) fn bide_outcomes(&self) -> [(BideState, f64); 2] {
        if matches!(self, Self::Bide) {
            [(BideState::new(2), 0.5), (BideState::new(3), 0.5)]
        } else {
            unreachable!("Only Bide starts a Bide state")
        }
    }

    pub(crate) fn disable_outcomes(
        &self,
        occupied_slots: [bool; 4],
        pp: Option<&[u8; 4]>,
    ) -> impl DoubleEndedIterator<Item = (DisableState, f64)> + use<> {
        if !matches!(self, Self::Disable) {
            unreachable!("Only Disable disables a move")
        }
        let eligible: [bool; 4] = array::from_fn(|slot| occupied_slots[slot] && pp.is_none_or(|remaining| remaining[slot] > 0));
        let count = eligible.iter().filter(|&&available| available).count();
        let probability = 1.0f64 / count.max(1) as f64;
        eligible
            .into_iter()
            .enumerate()
            .filter(|&(_, available)| available)
            .map(move |(slot, _)| (DisableState::new(slot), probability))
    }

    pub(crate) fn drained_hp(&self, dmg_dealt: u16) -> u16 {
        if dmg_dealt > 0 && matches!(self, Self::DrainHp | Self::DreamEater) {
            (dmg_dealt / 2).max(1)
        } else {
            0
        }
    }

    pub(crate) fn duration_end_probability(&self, turns_elapsed: u8) -> f64 {
        let max_turns = match *self {
            Self::Confusion => {
                if turns_elapsed == 0 {
                    return 0.0;
                }
                5
            }
            Self::Disable => 8,
            Self::Sleep => 7,
            Self::Trapping => {
                return match turns_elapsed {
                    0 => 3.0 / 8.0,
                    1 => 3.0 / 5.0,
                    2 => 1.0 / 2.0,
                    3 => 1.0,
                    _ => unreachable!("A trapping move must end after its 4th follow-up hit"),
                };
            }
            Self::AttackDownSide
            | Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackTwice
            | Self::AttackUp1
            | Self::AttackUp2
            | Self::Bide
            | Self::BurnSide1
            | Self::BurnSide2
            | Self::Charge
            | Self::ConfusionSide
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseDownSide
            | Self::DefenseUp1
            | Self::DefenseUp2
            | Self::DrainHp
            | Self::DreamEater
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::Flinch2
            | Self::FocusEnergy
            | Self::FreezeSide
            | Self::Haze
            | Self::Heal
            | Self::HyperBeam
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::Mimic
            | Self::MirrorMove
            | Self::OHKO
            | Self::NoEffect
            | Self::Paralyze
            | Self::ParalyzeSide1
            | Self::ParalyzeSide2
            | Self::PayDay
            | Self::Poison
            | Self::PoisonSide1
            | Self::PoisonSide2
            | Self::Rage
            | Self::Recoil
            | Self::Reflect
            | Self::SpecialDamage
            | Self::SpecialDownSide
            | Self::SpecialUp1
            | Self::SpecialUp2
            | Self::SpeedDown
            | Self::SpeedDownSide
            | Self::SpeedUp
            | Self::Substitute
            | Self::SuperFang
            | Self::Swift
            | Self::SwitchAndTeleport
            | Self::ThrashPetalDance
            | Self::Twineedle
            | Self::TwoToFiveAttacks => unreachable!("Only Confusion, Disable, Sleep and trapping moves defer their durations"),
        };
        1.0 / f64::from(max_turns - turns_elapsed)
    }

    pub(crate) fn hit_probability(&self, probability: f64, is_ai: bool) -> f64 {
        match *self {
            Self::AccuracyDown | Self::AttackDown | Self::DefenseDown1 | Self::DefenseDown2 | Self::SpeedDown => {
                if is_ai {
                    probability * 0.75
                } else {
                    probability
                }
            }
            Self::AttackDownSide
            | Self::BurnSide1
            | Self::BurnSide2
            | Self::Charge
            | Self::Confusion
            | Self::ConfusionSide
            | Self::DefenseDownSide
            | Self::Disable
            | Self::DrainHp
            | Self::DreamEater
            | Self::Explode
            | Self::Flinch1
            | Self::Flinch2
            | Self::FreezeSide
            | Self::HyperBeam
            | Self::LeechSeed
            | Self::Mimic
            | Self::NoEffect
            | Self::Paralyze
            | Self::ParalyzeSide1
            | Self::ParalyzeSide2
            | Self::PayDay
            | Self::Poison
            | Self::PoisonSide1
            | Self::PoisonSide2
            | Self::Recoil
            | Self::Sleep
            | Self::SpecialDamage
            | Self::SpecialDownSide
            | Self::SpeedDownSide
            | Self::SuperFang
            | Self::Trapping
            | Self::TwoToFiveAttacks => probability,
            Self::AttackTwice
            | Self::Haze
            | Self::Metronome
            | Self::MirrorMove
            | Self::OHKO
            | Self::Rage
            | Self::SpeedUp
            | Self::ThrashPetalDance
            | Self::Twineedle => todo!(),
            Self::AttackUp1
            | Self::AttackUp2
            | Self::Bide
            | Self::DefenseUp1
            | Self::DefenseUp2
            | Self::EvasionUp
            | Self::FocusEnergy
            | Self::Heal
            | Self::LightScreen
            | Self::Reflect
            | Self::SpecialUp1
            | Self::SpecialUp2
            | Self::Substitute
            | Self::Swift
            | Self::SwitchAndTeleport => 1.0,
        }
    }

    pub(crate) fn secondary_probability(&self, target: &CombatantState, target_types: &[Type; 2], mv_type: &Type, is_ai: bool) -> f64 {
        if target.hp == 0 {
            return 0.0;
        }
        match *self {
            Self::AttackDownSide if target.stages.atk > -6 => {
                let probability = 85.0 / 256.0;
                if is_ai { probability * 0.75 } else { probability }
            }
            Self::BurnSide1 if target.can_be_burned(target_types) => 26.0 / 256.0,
            Self::BurnSide2 if target.can_be_burned(target_types) => 77.0 / 256.0,
            Self::ConfusionSide if target.can_be_confused() => 25.0 / 256.0,
            Self::DefenseDownSide if target.stages.def > -6 => {
                let probability = 85.0 / 256.0;
                if is_ai { probability * 0.75 } else { probability }
            }
            Self::Flinch1 => 26.0 / 256.0,
            Self::Flinch2 => 77.0 / 256.0,
            Self::ParalyzeSide1 | Self::ParalyzeSide2 if target.can_be_paralyzed() => {
                if target_types.contains(mv_type) || target_types.iter().any(|target_type| mv_type.effectiveness(target_type) == 0) {
                    0.0
                } else if matches!(self, Self::ParalyzeSide1) {
                    26.0 / 256.0
                } else {
                    77.0 / 256.0
                }
            }
            Self::PoisonSide1 | Self::PoisonSide2 if target.can_be_poisoned(target_types) => self.poison_probability(),
            Self::SpecialDownSide if target.stages.spc > -6 => {
                let probability = 85.0 / 256.0;
                if is_ai { probability * 0.75 } else { probability }
            }
            Self::SpeedDownSide if target.stages.spd > -6 => {
                let probability = 85.0 / 256.0;
                if is_ai { probability * 0.75 } else { probability }
            }
            Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackDownSide
            | Self::AttackTwice
            | Self::AttackUp1
            | Self::AttackUp2
            | Self::Bide
            | Self::BurnSide1
            | Self::BurnSide2
            | Self::Charge
            | Self::Confusion
            | Self::ConfusionSide
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseDownSide
            | Self::DefenseUp1
            | Self::DefenseUp2
            | Self::Disable
            | Self::DrainHp
            | Self::DreamEater
            | Self::EvasionUp
            | Self::Explode
            | Self::FocusEnergy
            | Self::FreezeSide
            | Self::Haze
            | Self::Heal
            | Self::HyperBeam
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::Mimic
            | Self::MirrorMove
            | Self::NoEffect
            | Self::OHKO
            | Self::Paralyze
            | Self::ParalyzeSide1
            | Self::ParalyzeSide2
            | Self::PayDay
            | Self::Poison
            | Self::PoisonSide1
            | Self::PoisonSide2
            | Self::Rage
            | Self::Recoil
            | Self::Reflect
            | Self::Sleep
            | Self::SpecialDamage
            | Self::SpecialDownSide
            | Self::SpecialUp1
            | Self::SpecialUp2
            | Self::SpeedDown
            | Self::SpeedDownSide
            | Self::SpeedUp
            | Self::Substitute
            | Self::SuperFang
            | Self::Swift
            | Self::SwitchAndTeleport
            | Self::ThrashPetalDance
            | Self::Trapping
            | Self::Twineedle
            | Self::TwoToFiveAttacks => 0.0,
        }
    }

    pub(super) fn recoil_damage(&self, damage_dealt: u16, divisor: u16) -> u16 {
        if damage_dealt > 0 && matches!(self, Self::Recoil) {
            (damage_dealt / divisor).max(1)
        } else {
            0
        }
    }

    const fn poison_probability(&self) -> f64 {
        if matches!(self, Self::PoisonSide1) {
            52.0f64 / 256.0f64
        } else if matches!(self, Self::PoisonSide2) {
            103.0f64 / 256.0f64
        } else {
            0.0f64
        }
    }
}

pub enum EffectResolution {
    Bide,
    Charge,
    CopyMove,
    Damage,
    Disable,
    FixedDamage,
    Sleep,
    Status,
    SwitchOrEscape,
}
