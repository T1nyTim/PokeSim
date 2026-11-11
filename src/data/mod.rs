pub mod effects;
pub mod move_data;
pub mod moves;
pub mod species;

use core::hash::Hash;

use crate::data::{effects::Effect, move_data::Move};

pub enum Evolve {
    Level(u8),
    #[expect(dead_code, reason = "Will be used when other items are implemented")]
    Item(Item),
}

impl Evolve {
    const fn is_met(&self, level: u8) -> bool {
        match *self {
            Self::Item(_) => false,
            Self::Level(req) => level >= req,
        }
    }
}

pub enum GrowthRate {
    Fast,
    MediumFast,
    MediumSlow,
    Slow,
}

impl GrowthRate {
    pub(super) fn experience_at(&self, level: u8) -> u32 {
        let lv = i32::from(level);
        let total = match *self {
            Self::Fast => lv.pow(3) * 4i32 / 5i32,
            Self::MediumFast => lv.pow(3),
            Self::MediumSlow => lv.pow(3) * 6i32 / 5i32 - 15i32 * lv.pow(2) + lv * 100i32 - 140i32,
            Self::Slow => lv.pow(3) * 5i32 / 6i32,
        };
        total.max(0).cast_unsigned()
    }
}

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
    pub(super) const fn battle_effect(&self) -> Option<&'static Effect> {
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
            Self::XSpeed => Some(&Effect::SpeedUp),
        }
    }

    pub(super) const fn name(&self) -> &'static str {
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

    pub(super) const fn battle_heal_amount(&self) -> Option<u16> {
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

pub enum SpeciesKind {
    Abra,
    Arbok,
    Arcanine,
    Beedrill,
    Bellsprout,
    Blastoise,
    Bulbasaur,
    Butterfree,
    Caterpie,
    Charizard,
    Charmander,
    Charmeleon,
    Clefable,
    Clefairy,
    Cloyster,
    Cubone,
    Diglett,
    Drowzee,
    Dugtrio,
    Ekans,
    Electrode,
    Exeggcute,
    Exeggutor,
    Fearow,
    Gastly,
    Geodude,
    Gloom,
    Golbat,
    Goldeen,
    Graveler,
    Grimer,
    Growlithe,
    Gyarados,
    Haunter,
    Horsea,
    Hypno,
    Ivysaur,
    Jigglypuff,
    Kadabra,
    Kakuna,
    Kangaskhan,
    Koffing,
    Machoke,
    Machop,
    Magikarp,
    Magnemite,
    Magneton,
    Mankey,
    Marowak,
    Meowth,
    Metapod,
    Muk,
    Nidoking,
    Nidoqueen,
    NidoranF,
    NidoranM,
    Nidorina,
    Nidorino,
    Ninetales,
    Onix,
    Oddish,
    Paras,
    Parasect,
    Persian,
    Pidgeot,
    Pidgeotto,
    Pidgey,
    Pikachu,
    Ponyta,
    Poliwag,
    Poliwhirl,
    Poliwrath,
    Primeape,
    Raichu,
    Rapidash,
    Raticate,
    Rattata,
    Rhydon,
    Rhyhorn,
    Sandshrew,
    Sandslash,
    Seadra,
    Seaking,
    Shellder,
    Slowbro,
    Slowpoke,
    Snorlax,
    Spearow,
    Squirtle,
    Starmie,
    Staryu,
    Tangela,
    Tentacool,
    Tentacruel,
    Venomoth,
    Venonat,
    Venusaur,
    Victreebel,
    Vileplume,
    Voltorb,
    Vulpix,
    Wartortle,
    Weedle,
    Weepinbell,
    Weezing,
    Wigglytuff,
    Zubat,
}

impl SpeciesKind {
    const fn as_str(&self) -> &'static str {
        match *self {
            Self::Abra => "Abra",
            Self::Arbok => "Arbok",
            Self::Arcanine => "Arcanine",
            Self::Beedrill => "Beedrill",
            Self::Bellsprout => "Bellsprout",
            Self::Blastoise => "Blastoise",
            Self::Bulbasaur => "Bulbasaur",
            Self::Butterfree => "Butterfree",
            Self::Caterpie => "Caterpie",
            Self::Charizard => "Charizard",
            Self::Charmander => "Charmander",
            Self::Charmeleon => "Charmeleon",
            Self::Clefable => "Clefable",
            Self::Clefairy => "Clefairy",
            Self::Cloyster => "Cloyster",
            Self::Cubone => "Cubone",
            Self::Diglett => "Diglett",
            Self::Drowzee => "Drowzee",
            Self::Dugtrio => "Dugtrio",
            Self::Ekans => "Ekans",
            Self::Electrode => "Electrode",
            Self::Exeggcute => "Exeggcute",
            Self::Exeggutor => "Exeggutor",
            Self::Fearow => "Fearow",
            Self::Gastly => "Gastly",
            Self::Geodude => "Geodude",
            Self::Gloom => "Gloom",
            Self::Golbat => "Golbat",
            Self::Goldeen => "Goldeen",
            Self::Graveler => "Graveler",
            Self::Grimer => "Grimer",
            Self::Growlithe => "Growlithe",
            Self::Gyarados => "Gyarados",
            Self::Haunter => "Haunter",
            Self::Horsea => "Horsea",
            Self::Hypno => "Hypno",
            Self::Ivysaur => "Ivysaur",
            Self::Jigglypuff => "Jigglypuff",
            Self::Kadabra => "Kadabra",
            Self::Kakuna => "Kakuna",
            Self::Kangaskhan => "Kangaskhan",
            Self::Koffing => "Koffing",
            Self::Machoke => "Machoke",
            Self::Machop => "Machop",
            Self::Magikarp => "Magikarp",
            Self::Magnemite => "Magnemite",
            Self::Magneton => "Magneton",
            Self::Mankey => "Mankey",
            Self::Marowak => "Marowak",
            Self::Meowth => "Meowth",
            Self::Metapod => "Metapod",
            Self::Muk => "Muk",
            Self::Nidoking => "Nidoking",
            Self::Nidoqueen => "Nidoqueen",
            Self::NidoranF => "Nidoran\u{2640}",
            Self::NidoranM => "Nidoran\u{2642}",
            Self::Nidorina => "Nidorina",
            Self::Nidorino => "Nidorino",
            Self::Ninetales => "Ninetales",
            Self::Oddish => "Oddish",
            Self::Onix => "Onix",
            Self::Paras => "Paras",
            Self::Parasect => "Parasect",
            Self::Persian => "Persian",
            Self::Pidgeot => "Pidgeot",
            Self::Pidgeotto => "Pidgeotto",
            Self::Pidgey => "Pidgey",
            Self::Pikachu => "Pikachu",
            Self::Poliwag => "Poliwag",
            Self::Poliwhirl => "Poliwhirl",
            Self::Poliwrath => "Poliwrath",
            Self::Ponyta => "Ponyta",
            Self::Primeape => "Primeape",
            Self::Raichu => "Raichu",
            Self::Rapidash => "Rapidash",
            Self::Raticate => "Raticate",
            Self::Rattata => "Rattata",
            Self::Rhydon => "Rhydon",
            Self::Rhyhorn => "Rhyhorn",
            Self::Sandshrew => "Sandshrew",
            Self::Sandslash => "Sandslash",
            Self::Seadra => "Seadra",
            Self::Seaking => "Seaking",
            Self::Shellder => "Shellder",
            Self::Slowbro => "Slowbro",
            Self::Slowpoke => "Slowpoke",
            Self::Snorlax => "Snorlax",
            Self::Spearow => "Spearow",
            Self::Squirtle => "Squirtle",
            Self::Starmie => "Starmie",
            Self::Staryu => "Staryu",
            Self::Tangela => "Tangela",
            Self::Tentacool => "Tentacool",
            Self::Tentacruel => "Tentacruel",
            Self::Venomoth => "Venomoth",
            Self::Venonat => "Venonat",
            Self::Venusaur => "Venusaur",
            Self::Victreebel => "Victreebel",
            Self::Vileplume => "Vileplume",
            Self::Voltorb => "Voltorb",
            Self::Vulpix => "Vulpix",
            Self::Wartortle => "Wartortle",
            Self::Weedle => "Weedle",
            Self::Weepinbell => "Weepinbell",
            Self::Weezing => "Weezing",
            Self::Wigglytuff => "Wigglytuff",
            Self::Zubat => "Zubat",
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum Type {
    Bug,
    Dragon,
    Electric,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,
    Normal,
    Poison,
    Psychic,
    Rock,
    Water,
}

impl Type {
    pub(super) const fn effectiveness(&self, target: &Self) -> u32 {
        self.ordered_effectiveness(target).1
    }

    pub(super) const fn is_special(&self) -> bool {
        matches!(
            self,
            Self::Dragon | Self::Electric | Self::Fire | Self::Grass | Self::Ice | Self::Psychic | Self::Water
        )
    }

    pub(super) fn ai_effectiveness(&self, targets: &[Self; 2]) -> u32 {
        self.ordered_effectiveness(&targets[0]).min(self.ordered_effectiveness(&targets[1])).1
    }

    #[expect(clippy::match_same_arms, reason = "Order is important here")]
    const fn ordered_effectiveness(&self, target: &Self) -> (u8, u32) {
        match (self, target) {
            (&Self::Bug, &Self::Fire) => (0, 5),
            (&Self::Bug, &Self::Grass) => (1, 20),
            (&Self::Bug, &Self::Fighting | &Self::Flying) => (2, 5),
            (&Self::Bug, &Self::Psychic) => (3, 20),
            (&Self::Bug, &Self::Ghost) => (4, 5),
            (&Self::Bug, &Self::Poison) => (5, 20),
            (&Self::Dragon, &Self::Dragon) => (0, 20),
            (&Self::Electric, &Self::Water) => (0, 20),
            (&Self::Electric, &Self::Electric | &Self::Grass) => (1, 5),
            (&Self::Electric, &Self::Ground) => (2, 0),
            (&Self::Electric, &Self::Flying) => (3, 20),
            (&Self::Electric, &Self::Dragon) => (4, 5),
            (&Self::Fighting, &Self::Normal) => (0, 20),
            (&Self::Fighting, &Self::Poison | &Self::Flying | &Self::Psychic | &Self::Bug) => (1, 5),
            (&Self::Fighting, &Self::Rock | &Self::Ice) => (2, 20),
            (&Self::Fighting, &Self::Ghost) => (3, 0),
            (&Self::Fire, &Self::Grass | &Self::Ice) => (0, 20),
            (&Self::Fire, &Self::Fire | &Self::Water) => (1, 5),
            (&Self::Fire, &Self::Bug) => (2, 20),
            (&Self::Fire, &Self::Rock) => (3, 5),
            (&Self::Fire, &Self::Dragon) => (4, 5),
            (&Self::Flying, &Self::Electric) => (0, 5),
            (&Self::Flying, &Self::Fighting | &Self::Bug | &Self::Grass) => (1, 20),
            (&Self::Flying, &Self::Rock) => (2, 5),
            (&Self::Ghost, &Self::Ghost) => (0, 20),
            (&Self::Ghost, &Self::Normal | &Self::Psychic) => (1, 0),
            (&Self::Grass, &Self::Water) => (0, 20),
            (&Self::Grass, &Self::Grass | &Self::Fire) => (1, 5),
            (&Self::Grass, &Self::Ground) => (2, 20),
            (&Self::Grass, &Self::Bug | &Self::Poison) => (3, 5),
            (&Self::Grass, &Self::Rock) => (4, 20),
            (&Self::Grass, &Self::Flying) => (5, 5),
            (&Self::Grass, &Self::Dragon) => (6, 5),
            (&Self::Ground, &Self::Flying) => (0, 0),
            (&Self::Ground, &Self::Fire | &Self::Electric) => (1, 20),
            (&Self::Ground, &Self::Grass | &Self::Bug) => (2, 5),
            (&Self::Ground, &Self::Rock | &Self::Poison) => (3, 20),
            (&Self::Ice, &Self::Ice | &Self::Water) => (0, 5),
            (&Self::Ice, &Self::Grass | &Self::Ground | &Self::Flying) => (1, 20),
            (&Self::Ice, &Self::Dragon) => (2, 20),
            (&Self::Normal, &Self::Rock) => (0, 5),
            (&Self::Normal, &Self::Ghost) => (1, 0),
            (&Self::Poison, &Self::Grass) => (0, 20),
            (&Self::Poison, &Self::Poison | &Self::Ground) => (1, 5),
            (&Self::Poison, &Self::Bug) => (2, 20),
            (&Self::Poison, &Self::Rock | &Self::Ghost) => (3, 5),
            (&Self::Psychic, &Self::Psychic) => (0, 5),
            (&Self::Psychic, &Self::Fighting | &Self::Poison) => (1, 20),
            (&Self::Rock, &Self::Fire) => (0, 20),
            (&Self::Rock, &Self::Fighting | &Self::Ground) => (1, 5),
            (&Self::Rock, &Self::Flying | &Self::Bug | &Self::Ice) => (2, 20),
            (&Self::Water, &Self::Fire | &Self::Rock) => (0, 20),
            (&Self::Water, &Self::Water | &Self::Grass) => (1, 5),
            (&Self::Water, &Self::Ground) => (2, 20),
            (&Self::Water, &Self::Dragon) => (3, 5),
            _ => (u8::MAX, 10),
        }
    }
}

pub struct BaseStats {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spd: u8,
    pub spc: u8,
}

impl BaseStats {
    pub(super) const fn new(hp: u8, atk: u8, def: u8, spd: u8, spc: u8) -> Self {
        Self { hp, atk, def, spd, spc }
    }
}

pub struct Species {
    kind: SpeciesKind,
    pub base_stats: BaseStats,
    pub types: [Type; 2],
    pub base_exp: u8,
    pub learnset: [Option<&'static Move>; 4],
    pub growth: GrowthRate,
    tm_moves: &'static [&'static Move],
    evolution: &'static [(Evolve, &'static Self)],
    pub evo_moves: &'static [(u8, &'static Move)],
}

impl Species {
    #[expect(clippy::too_many_arguments, reason = "to be looked at")]
    pub(super) const fn new(
        kind: SpeciesKind,
        base_stats: BaseStats,
        types: [Type; 2],
        base_exp: u8,
        learnset: [Option<&'static Move>; 4],
        growth: GrowthRate,
        tm_moves: &'static [&'static Move],
        evolution: &'static [(Evolve, &'static Self)],
        evo_moves: &'static [(u8, &'static Move)],
    ) -> Self {
        Self {
            kind,
            base_stats,
            types,
            base_exp,
            learnset,
            growth,
            tm_moves,
            evolution,
            evo_moves,
        }
    }

    pub(super) const fn name(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(super) fn can_learn_tm(&self, mv: &Move) -> bool {
        self.tm_moves.contains(&mv)
    }

    pub(super) fn evolution_at(&self, level: u8) -> Option<&'static Self> {
        self.evolution
            .iter()
            .find_map(|(condition, species)| condition.is_met(level).then_some(*species))
    }
}
