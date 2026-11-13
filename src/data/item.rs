use crate::data::{Effect, Move};

#[derive(Eq, PartialEq)]
pub enum Item {
    Calcium,
    Carbos,
    FireStone,
    GuardSpec,
    HpUp,
    HyperPotion,
    Iron,
    LeafStone,
    MoonStone,
    Potion,
    PpUp,
    Protein,
    RareCandy,
    SuperPotion,
    ThunderStone,
    Tm(&'static Move),
    WaterStone,
    XDefend,
    XSpeed,
}

impl Item {
    pub(crate) const fn battle_effect(&self) -> Option<&'static Effect> {
        match self {
            Self::Calcium
            | Self::Carbos
            | Self::FireStone
            | Self::HpUp
            | Self::HyperPotion
            | Self::Iron
            | Self::LeafStone
            | Self::MoonStone
            | Self::Potion
            | Self::PpUp
            | Self::Protein
            | Self::RareCandy
            | Self::SuperPotion
            | Self::ThunderStone
            | Self::Tm(_)
            | Self::WaterStone => None,
            Self::GuardSpec => Some(&Effect::Mist),
            Self::XDefend => Some(&Effect::DefenseUp1),
            Self::XSpeed => Some(&Effect::SpeedUp1),
        }
    }

    pub(crate) const fn name(&self) -> &'static str {
        match self {
            Self::Calcium => "Calcium",
            Self::Carbos => "Carbos",
            Self::FireStone => "Fire Stone",
            Self::GuardSpec => "Guard Spec.",
            Self::HpUp => "HP Up",
            Self::HyperPotion => "Hyper Potion",
            Self::Iron => "Iron",
            Self::LeafStone => "Leaf Stone",
            Self::MoonStone => "Moon Stone",
            Self::Potion => "Potion",
            Self::PpUp => "PP Up",
            Self::Protein => "Protein",
            Self::RareCandy => "Rare Candy",
            Self::SuperPotion => "Super Potion",
            Self::ThunderStone => "Thunder Stone",
            Self::Tm(mv) => mv.name(),
            Self::WaterStone => "Water Stone",
            Self::XDefend => "X Defend",
            Self::XSpeed => "X Speed",
        }
    }

    pub(crate) const fn battle_heal_amount(&self) -> Option<u16> {
        match self {
            Self::HyperPotion => Some(200),
            Self::Potion => Some(20),
            Self::SuperPotion => Some(50),
            Self::Calcium
            | Self::Carbos
            | Self::FireStone
            | Self::GuardSpec
            | Self::HpUp
            | Self::Iron
            | Self::LeafStone
            | Self::MoonStone
            | Self::PpUp
            | Self::Protein
            | Self::RareCandy
            | Self::ThunderStone
            | Self::Tm(_)
            | Self::WaterStone
            | Self::XDefend
            | Self::XSpeed => None,
        }
    }
}
