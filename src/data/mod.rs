pub mod effects;
pub mod moves;
pub mod species;

use core::hash::{Hash, Hasher};

use crate::data::{effects::Effect, moves::STRUGGLE};

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
}

impl GrowthRate {
    pub(super) fn experience_at(&self, level: u8) -> u32 {
        let lv = i32::from(level);
        let total = match *self {
            Self::Fast => lv.pow(3) * 4i32 / 5i32,
            Self::MediumFast => lv.pow(3),
            Self::MediumSlow => lv.pow(3) * 6i32 / 5i32 - 15i32 * lv.pow(2) + lv * 100i32 - 140i32,
        };
        total.max(0).cast_unsigned()
    }
}

#[derive(Eq, PartialEq)]
pub enum Item {
    HpUp,
    MoonStone,
}

impl Item {
    pub(super) const fn name(&self) -> &'static str {
        match self {
            Self::HpUp => "HP Up",
            Self::MoonStone => "Moon Stone",
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum MoveKind {
    Acid,
    Agility,
    Bide,
    Bind,
    Bite,
    BodySlam,
    ConfuseRay,
    Confusion,
    DefenseCurl,
    Dig,
    Disable,
    DoubleEdge,
    DoubleKick,
    Doubleslap,
    DrillPeck,
    Earthquake,
    Ember,
    Explosion,
    FireSpin,
    Flamethrower,
    FocusEnergy,
    FuryAttack,
    FurySwipes,
    Glare,
    Growl,
    Growth,
    Gust,
    Harden,
    Haze,
    HornAttack,
    HornDrill,
    HyperFang,
    LeechLife,
    LeechSeed,
    Leer,
    LightScreen,
    Metronome,
    Minimize,
    MirrorMove,
    Peck,
    PinMissile,
    PoisonSting,
    Poisonpowder,
    Pound,
    Psybeam,
    QuickAttack,
    Rage,
    RazorLeaf,
    Rest,
    RockThrow,
    SandAttack,
    Scratch,
    Screech,
    Selfdestruct,
    Sing,
    Slam,
    Slash,
    SleepPowder,
    Solarbeam,
    StringShot,
    Struggle,
    StunSpore,
    SuperFang,
    Supersonic,
    Swift,
    Tackle,
    TailWhip,
    Thrash,
    Twineedle,
    VineWhip,
    Whirlwind,
    WingAttack,
    Wrap,
}

impl MoveKind {
    const fn as_str(&self) -> &'static str {
        match *self {
            Self::Acid => "Acid",
            Self::Agility => "Agility",
            Self::Bide => "Bide",
            Self::Bind => "Bind",
            Self::Bite => "Bite",
            Self::BodySlam => "Body Slam",
            Self::ConfuseRay => "Confuse Ray",
            Self::Confusion => "Confusion",
            Self::DefenseCurl => "Defense Curl",
            Self::Dig => "Dig",
            Self::Disable => "Disable",
            Self::DoubleEdge => "Double-Edge",
            Self::DoubleKick => "Double Kick",
            Self::Doubleslap => "Doubleslap",
            Self::DrillPeck => "Drill Peck",
            Self::Earthquake => "Earthquake",
            Self::Ember => "Ember",
            Self::Explosion => "Explosion",
            Self::FireSpin => "Fire Spin",
            Self::Flamethrower => "Flamethrower",
            Self::FocusEnergy => "Focus Energy",
            Self::FuryAttack => "Fury Attack",
            Self::FurySwipes => "Fury Swipes",
            Self::Glare => "Glare",
            Self::Growl => "Growl",
            Self::Growth => "Growth",
            Self::Gust => "Gust",
            Self::Harden => "Harden",
            Self::Haze => "Haze",
            Self::HornAttack => "Horn Attack",
            Self::HornDrill => "Horn Drill",
            Self::HyperFang => "Hyper Fang",
            Self::LeechLife => "Leech Life",
            Self::LeechSeed => "Leech Seed",
            Self::Leer => "Leer",
            Self::LightScreen => "Light Screen",
            Self::Metronome => "Metronome",
            Self::Minimize => "Minimize",
            Self::MirrorMove => "Mirror Move",
            Self::Peck => "Peck",
            Self::PinMissile => "Pin Missile",
            Self::PoisonSting => "Poison Sting",
            Self::Poisonpowder => "Poisonpowder",
            Self::Pound => "Pound",
            Self::Psybeam => "Psybeam",
            Self::QuickAttack => "Quick Attack",
            Self::Rage => "Rage",
            Self::RazorLeaf => "Razor Leaf",
            Self::Rest => "Rest",
            Self::RockThrow => "Rock Throw",
            Self::SandAttack => "Sand-Attack",
            Self::Scratch => "Scratch",
            Self::Screech => "Screech",
            Self::Selfdestruct => "Selfdestruct",
            Self::Sing => "Sing",
            Self::Slam => "Slam",
            Self::Slash => "Slash",
            Self::SleepPowder => "Sleep Powder",
            Self::Solarbeam => "Solarbeam",
            Self::StringShot => "String Shot",
            Self::Struggle => "Struggle",
            Self::StunSpore => "Stun Spore",
            Self::SuperFang => "Super Fang",
            Self::Supersonic => "Supersonic",
            Self::Swift => "Swift",
            Self::Tackle => "Tackle",
            Self::TailWhip => "Tail Whip",
            Self::Thrash => "Thrash",
            Self::Twineedle => "Twineedle",
            Self::VineWhip => "Vine Whip",
            Self::Whirlwind => "Whirlwind",
            Self::WingAttack => "Wing Attack",
            Self::Wrap => "Wrap",
        }
    }
}

pub enum SpeciesKind {
    Arbok,
    Beedrill,
    Bulbasaur,
    Butterfree,
    Caterpie,
    Charizard,
    Charmander,
    Charmeleon,
    Clefable,
    Clefairy,
    Diglett,
    Dugtrio,
    Ekans,
    Fearow,
    Geodude,
    Golbat,
    Graveler,
    Ivysaur,
    Jigglypuff,
    Kakuna,
    Metapod,
    Nidoking,
    Nidoqueen,
    NidoranF,
    NidoranM,
    Nidorina,
    Nidorino,
    Onix,
    Pidgeot,
    Pidgeotto,
    Pidgey,
    Raticate,
    Rattata,
    Sandshrew,
    Sandslash,
    Spearow,
    Venusaur,
    Weedle,
    Wigglytuff,
    Zubat,
}

impl SpeciesKind {
    const fn as_str(&self) -> &'static str {
        match *self {
            Self::Arbok => "Arbok",
            Self::Beedrill => "Beedrill",
            Self::Bulbasaur => "Bulbasaur",
            Self::Butterfree => "Butterfree",
            Self::Caterpie => "Caterpie",
            Self::Charizard => "Charizard",
            Self::Charmander => "Charmander",
            Self::Charmeleon => "Charmeleon",
            Self::Clefable => "Clefable",
            Self::Clefairy => "Clefairy",
            Self::Diglett => "Diglett",
            Self::Dugtrio => "Dugtrio",
            Self::Ekans => "Ekans",
            Self::Fearow => "Fearow",
            Self::Geodude => "Geodude",
            Self::Golbat => "Golbat",
            Self::Graveler => "Graveler",
            Self::Ivysaur => "Ivysaur",
            Self::Jigglypuff => "Jigglypuff",
            Self::Kakuna => "Kakuna",
            Self::Metapod => "Metapod",
            Self::Nidoking => "Nidoking",
            Self::Nidoqueen => "Nidoqueen",
            Self::NidoranF => "Nidoran\u{2640}",
            Self::NidoranM => "Nidoran\u{2642}",
            Self::Nidorina => "Nidorina",
            Self::Nidorino => "Nidorino",
            Self::Onix => "Onix",
            Self::Pidgeot => "Pidgeot",
            Self::Pidgeotto => "Pidgeotto",
            Self::Pidgey => "Pidgey",
            Self::Raticate => "Raticate",
            Self::Rattata => "Rattata",
            Self::Sandshrew => "Sandshrew",
            Self::Sandslash => "Sandslash",
            Self::Spearow => "Spearow",
            Self::Venusaur => "Venusaur",
            Self::Weedle => "Weedle",
            Self::Wigglytuff => "Wigglytuff",
            Self::Zubat => "Zubat",
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum Type {
    Bug,
    // Dragon,
    // Electric,
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
    // Water,
}

impl Type {
    pub(super) const fn effectiveness(&self, target: &Self) -> u32 {
        match (self, target) {
            (&Self::Fire | &Self::Flying | &Self::Poison | &Self::Rock, &Self::Bug)
            | (&Self::Flying | &Self::Psychic, &Self::Fighting)
            | (&Self::Ground | &Self::Rock, &Self::Fire)
            | (&Self::Ice | &Self::Rock, &Self::Flying)
            | (&Self::Ghost, &Self::Ghost)
            | (&Self::Bug | &Self::Fire | &Self::Flying | &Self::Ice | &Self::Poison, &Self::Grass)
            | (&Self::Grass | &Self::Ice, &Self::Ground)
            | (&Self::Fighting | &Self::Fire | &Self::Rock, &Self::Ice)
            | (&Self::Fighting, &Self::Normal)
            | (&Self::Bug | &Self::Ground | &Self::Psychic, &Self::Poison)
            | (&Self::Bug, &Self::Psychic)
            | (&Self::Fighting | &Self::Grass | &Self::Ground, &Self::Rock) => 20,
            (&Self::Fighting | &Self::Grass | &Self::Ground, &Self::Bug)
            | (&Self::Bug | &Self::Fire | &Self::Grass, &Self::Fire)
            | (&Self::Bug | &Self::Rock, &Self::Fighting)
            | (&Self::Bug | &Self::Fighting | &Self::Grass, &Self::Flying)
            | (&Self::Bug | &Self::Poison, &Self::Ghost)
            | (&Self::Grass | &Self::Ground, &Self::Grass)
            | (&Self::Poison | &Self::Rock, &Self::Ground)
            | (&Self::Ice, &Self::Ice)
            | (&Self::Fighting | &Self::Grass | &Self::Poison, &Self::Poison)
            | (&Self::Fighting | &Self::Psychic, &Self::Psychic)
            | (&Self::Fire | &Self::Flying | &Self::Normal | &Self::Poison, &Self::Rock) => 5,
            (&Self::Ground, &Self::Flying)
            | (&Self::Fighting | &Self::Normal, &Self::Ghost)
            | (&Self::Ghost, &Self::Normal)
            | (&Self::Ghost, &Self::Psychic) => 0,
            _ => 10,
        }
    }

    pub(super) const fn is_special(&self) -> bool {
        matches!(self, Self::Fire | Self::Grass | Self::Psychic) // Dragon, Electric, Ice, Water
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

#[derive(Eq, PartialEq)]
pub struct Move {
    kind: MoveKind,
    pub effect: Effect,
    pub power: u8,
    pub mv_type: Type,
    accuracy: u8,
    pub pp: u8,
}

#[expect(clippy::missing_trait_methods, reason = "hash_slice method not required")]
impl Hash for Move {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.kind.hash(state);
    }
}

impl Move {
    const fn new(kind: MoveKind, effect: Effect, power: u8, mv_type: Type, accuracy: u8, pp: u8) -> Self {
        Self {
            kind,
            effect,
            power,
            mv_type,
            accuracy,
            pp,
        }
    }

    pub(super) const fn accuracy(&self) -> u8 {
        (self.accuracy as u16 * 255 / 100) as u8
    }

    pub(super) const fn name(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(super) fn recoil_damage(&self, damage_dealt: u16) -> u16 {
        let divisor = if self == &STRUGGLE { 2 } else { 4 };
        self.effect.recoil_damage(damage_dealt, divisor)
    }
}

pub struct Species {
    kind: SpeciesKind,
    pub base_stats: BaseStats,
    pub types: [Type; 2],
    pub base_exp: u8,
    pub learnset: [Option<&'static Move>; 4],
    pub growth: GrowthRate,
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
            evolution,
            evo_moves,
        }
    }

    pub(super) const fn name(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(super) fn evolution_at(&self, level: u8) -> Option<&'static Self> {
        self.evolution
            .iter()
            .find_map(|(condition, species)| condition.is_met(level).then_some(*species))
    }
}
