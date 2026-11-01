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
    AttackTwice,
    Bide,
    BurnSide1,
    Charge,
    Confusion,
    ConfusionSide,
    DefenseDown1,
    DefenseDown2,
    DefenseDownSide,
    DefenseUp1,
    Disable,
    DrainHp,
    EvasionUp,
    Explode,
    Flinch1,
    FocusEnergy,
    Haze,
    Heal,
    LeechSeed,
    LightScreen,
    Metronome,
    MirrorMove,
    #[expect(clippy::upper_case_acronyms, reason = "is an acronym")]
    OHKO,
    Paralyze,
    ParalyzeSide2,
    Poison,
    PoisonSide1,
    Rage,
    Recoil,
    Sleep,
    SpecialUp1,
    SpeedDown,
    SpeedUp,
    SuperFang,
    Swift,
    SwitchAndTeleport,
    ThrashPetalDance,
    Trapping,
    Twineedle,
    TwoToFiveAttacks,
}

impl Effect {
    pub(crate) const fn inflicts_status_condition(&self) -> bool {
        matches!(
            self,
            Self::BurnSide1 | Self::Paralyze | Self::ParalyzeSide2 | Self::Poison | Self::PoisonSide1 | Self::Sleep
        )
    }

    pub(crate) const fn preserves_last_damage(&self) -> bool {
        matches!(self, Self::Confusion | Self::FocusEnergy | Self::LeechSeed | Self::Poison)
    }

    pub(crate) const fn resolution(&self) -> EffectResolution {
        match *self {
            Self::Bide => EffectResolution::Bide,
            Self::MirrorMove => EffectResolution::CopyMove,
            Self::NoEffect
            | Self::BurnSide1
            | Self::Charge
            | Self::ConfusionSide
            | Self::DefenseDownSide
            | Self::DrainHp
            | Self::Flinch1
            | Self::ParalyzeSide2
            | Self::PoisonSide1
            | Self::Rage
            | Self::Recoil
            | Self::Swift
            | Self::Trapping
            | Self::Twineedle
            | Self::TwoToFiveAttacks => EffectResolution::Damage,
            Self::Disable => EffectResolution::Disable,
            Self::SuperFang => EffectResolution::FixedDamage,
            Self::Sleep => EffectResolution::Sleep,
            Self::AccuracyDown
            | Self::AttackDown
            | Self::Confusion
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseUp1
            | Self::FocusEnergy
            | Self::LeechSeed
            | Self::Paralyze
            | Self::Poison
            | Self::SpecialUp1
            | Self::SpeedDown
            | Self::SpeedUp => EffectResolution::Status,
            Self::SwitchAndTeleport => EffectResolution::SwitchOrEscape,
            Self::AttackTwice
            | Self::EvasionUp
            | Self::Explode
            | Self::Haze
            | Self::Heal
            | Self::LightScreen
            | Self::Metronome
            | Self::OHKO
            | Self::ThrashPetalDance => todo!(),
        }
    }

    pub(crate) const fn targets_user(&self) -> bool {
        matches!(self, Self::Bide | Self::DefenseUp1 | Self::FocusEnergy | Self::SpecialUp1 | Self::SpeedUp)
    }

    pub(crate) fn apply_secondary(&self, target: &mut CombatantState) {
        match *self {
            Self::PoisonSide1 => target.poison(),
            Self::DefenseDownSide => self.apply_status(&mut target.stages),
            Self::NoEffect
            | Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackTwice
            | Self::Bide
            | Self::BurnSide1
            | Self::Charge
            | Self::Confusion
            | Self::ConfusionSide
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseUp1
            | Self::Disable
            | Self::DrainHp
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::FocusEnergy
            | Self::Haze
            | Self::Heal
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::MirrorMove
            | Self::OHKO
            | Self::Paralyze
            | Self::ParalyzeSide2
            | Self::Poison
            | Self::Rage
            | Self::Recoil
            | Self::Sleep
            | Self::SpecialUp1
            | Self::SpeedDown
            | Self::SpeedUp
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
            Self::AttackDown => stages.atk = (stages.atk - 1).max(-6),
            Self::DefenseDown1 | Self::DefenseDownSide => stages.def = (stages.def - 1).max(-6),
            Self::DefenseDown2 => stages.def = (stages.def - 2).max(-6),
            Self::DefenseUp1 => stages.def = (stages.def + 1).min(6),
            Self::SpeedDown => stages.spd = (stages.spd - 1).max(-6),
            Self::NoEffect
            | Self::AttackTwice
            | Self::Bide
            | Self::BurnSide1
            | Self::Charge
            | Self::Confusion
            | Self::ConfusionSide
            | Self::Disable
            | Self::DrainHp
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::FocusEnergy
            | Self::Haze
            | Self::Heal
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::MirrorMove
            | Self::OHKO
            | Self::Paralyze
            | Self::ParalyzeSide2
            | Self::Poison
            | Self::PoisonSide1
            | Self::Rage
            | Self::Recoil
            | Self::Sleep
            | Self::SpecialUp1
            | Self::SpeedUp
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

    pub(crate) fn disable_outcomes(&self, occupied_slots: [bool; 4], pp: Option<&[u8; 4]>) -> Vec<(DisableState, f64)> {
        if !matches!(self, Self::Disable) {
            unreachable!("Only Disable disables a move")
        }
        let slots: Vec<_> = occupied_slots
            .iter()
            .enumerate()
            .filter_map(|(slot, &occupied)| (occupied && pp.is_none_or(|remaining| remaining[slot] > 0)).then_some(slot))
            .collect();
        let mut outcomes = Vec::new();
        if !slots.is_empty() {
            let probability = 1.0f64 / slots.len() as f64;
            for slot in slots {
                outcomes.push((DisableState::new(slot), probability));
            }
        }
        outcomes
    }

    pub(crate) fn drained_hp(&self, dmg_dealt: u16) -> u16 {
        if dmg_dealt > 0 && matches!(self, Self::DrainHp) {
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
            Self::NoEffect
            | Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackTwice
            | Self::Bide
            | Self::BurnSide1
            | Self::Charge
            | Self::ConfusionSide
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseDownSide
            | Self::DefenseUp1
            | Self::DrainHp
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::FocusEnergy
            | Self::Haze
            | Self::Heal
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::MirrorMove
            | Self::OHKO
            | Self::Paralyze
            | Self::ParalyzeSide2
            | Self::Poison
            | Self::PoisonSide1
            | Self::Rage
            | Self::Recoil
            | Self::SpecialUp1
            | Self::SpeedDown
            | Self::SpeedUp
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
            Self::Bide | Self::DefenseUp1 => 1.0,
            Self::NoEffect
            | Self::Confusion
            | Self::DefenseDownSide
            | Self::Disable
            | Self::DrainHp
            | Self::LeechSeed
            | Self::PoisonSide1
            | Self::Recoil
            | Self::Sleep
            | Self::Trapping => probability,
            Self::AccuracyDown | Self::AttackDown | Self::DefenseDown1 | Self::DefenseDown2 | Self::SpeedDown => {
                if is_ai {
                    probability * 0.75
                } else {
                    probability
                }
            }
            Self::AttackTwice
            | Self::BurnSide1
            | Self::Charge
            | Self::ConfusionSide
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::FocusEnergy
            | Self::Haze
            | Self::Heal
            | Self::LightScreen
            | Self::Metronome
            | Self::MirrorMove
            | Self::OHKO
            | Self::Paralyze
            | Self::ParalyzeSide2
            | Self::Poison
            | Self::Rage
            | Self::SpecialUp1
            | Self::SpeedUp
            | Self::SuperFang
            | Self::Swift
            | Self::SwitchAndTeleport
            | Self::ThrashPetalDance
            | Self::Twineedle
            | Self::TwoToFiveAttacks => todo!(),
        }
    }

    pub(crate) fn secondary_probability(&self, target: &CombatantState, target_types: &[Type; 2], is_ai: bool) -> f64 {
        if target.hp == 0 {
            return 0.0;
        }
        match *self {
            Self::PoisonSide1 if target.can_be_poisoned(target_types) => self.poison_probability(),
            Self::DefenseDownSide if target.stages.def > -6 => {
                let probability = 85.0 / 256.0;
                if is_ai { probability * 0.75 } else { probability }
            }
            Self::NoEffect
            | Self::AccuracyDown
            | Self::AttackDown
            | Self::AttackTwice
            | Self::Bide
            | Self::BurnSide1
            | Self::Charge
            | Self::Confusion
            | Self::ConfusionSide
            | Self::DefenseDown1
            | Self::DefenseDown2
            | Self::DefenseDownSide
            | Self::DefenseUp1
            | Self::Disable
            | Self::DrainHp
            | Self::EvasionUp
            | Self::Explode
            | Self::Flinch1
            | Self::FocusEnergy
            | Self::Haze
            | Self::Heal
            | Self::LeechSeed
            | Self::LightScreen
            | Self::Metronome
            | Self::MirrorMove
            | Self::OHKO
            | Self::Paralyze
            | Self::ParalyzeSide2
            | Self::Poison
            | Self::PoisonSide1
            | Self::Rage
            | Self::Recoil
            | Self::Sleep
            | Self::SpecialUp1
            | Self::SpeedDown
            | Self::SpeedUp
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
        if matches!(self, Self::PoisonSide1) { 52.0f64 / 256.0f64 } else { 0.0f64 }
    }
}

pub enum EffectResolution {
    Bide,
    CopyMove,
    Damage,
    Disable,
    FixedDamage,
    Sleep,
    Status,
    SwitchOrEscape,
}
