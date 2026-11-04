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
    HpUp,
    LeafStone,
    MoonStone,
    RareCandy,
    ThunderStone,
    WaterStone,
    XDefend,
}

impl Item {
    pub(super) const fn battle_effect(&self) -> Option<&'static Effect> {
        match self {
            Self::HpUp | Self::LeafStone | Self::MoonStone | Self::RareCandy | Self::ThunderStone | Self::WaterStone => None,
            Self::XDefend => Some(&Effect::DefenseUp1),
        }
    }

    pub(super) const fn name(&self) -> &'static str {
        match self {
            Self::HpUp => "HP Up",
            Self::LeafStone => "Leaf Stone",
            Self::MoonStone => "Moon Stone",
            Self::RareCandy => "Rare Candy",
            Self::ThunderStone => "Thunder Stone",
            Self::WaterStone => "Water Stone",
            Self::XDefend => "X Defend",
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub enum MoveKind {
    Absorb,
    Acid,
    AcidArmor,
    Agility,
    Amnesia,
    AuroraBeam,
    Bide,
    Bind,
    Bite,
    BodySlam,
    Bubble,
    Bubblebeam,
    Clamp,
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
    Headbutt,
    HornAttack,
    HornDrill,
    HydroPump,
    HyperFang,
    Hypnosis,
    IceBeam,
    KarateChop,
    LeechLife,
    LeechSeed,
    Leer,
    LightScreen,
    LowKick,
    Meditate,
    Metronome,
    Minimize,
    MirrorMove,
    Peck,
    PetalDance,
    PinMissile,
    PoisonGas,
    PoisonSting,
    Poisonpowder,
    Pound,
    Psybeam,
    Psychic,
    QuickAttack,
    Rage,
    RazorLeaf,
    Recover,
    Reflect,
    Rest,
    RockThrow,
    SandAttack,
    Scratch,
    Screech,
    SeismicToss,
    Selfdestruct,
    Sing,
    SkullBash,
    Slam,
    Slash,
    SleepPowder,
    Sludge,
    Smog,
    Smokescreen,
    Solarbeam,
    Sonicboom,
    SpikeCannon,
    Spore,
    StringShot,
    Struggle,
    StunSpore,
    Submission,
    SuperFang,
    Supersonic,
    Swift,
    Tackle,
    TailWhip,
    Teleport,
    Thrash,
    Thunder,
    ThunderWave,
    Thundershock,
    Twineedle,
    VineWhip,
    WaterGun,
    Waterfall,
    Whirlwind,
    WingAttack,
    Withdraw,
    Wrap,
}

impl MoveKind {
    const fn as_str(&self) -> &'static str {
        match *self {
            Self::Absorb => "Absorb",
            Self::Acid => "Acid",
            Self::AcidArmor => "Acid Armor",
            Self::Agility => "Agility",
            Self::Amnesia => "Amnesia",
            Self::AuroraBeam => "AuroraBeam",
            Self::Bide => "Bide",
            Self::Bind => "Bind",
            Self::Bite => "Bite",
            Self::BodySlam => "Body Slam",
            Self::Bubble => "Bubble",
            Self::Bubblebeam => "Bubblebeam",
            Self::Clamp => "Clamp",
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
            Self::Headbutt => "Headbutt",
            Self::HornAttack => "Horn Attack",
            Self::HornDrill => "Horn Drill",
            Self::HydroPump => "Hydro Pump",
            Self::HyperFang => "Hyper Fang",
            Self::Hypnosis => "Hypnosis",
            Self::IceBeam => "Ice Beam",
            Self::KarateChop => "Karate Chop",
            Self::LeechLife => "Leech Life",
            Self::LeechSeed => "Leech Seed",
            Self::Leer => "Leer",
            Self::LightScreen => "Light Screen",
            Self::LowKick => "Low Kick",
            Self::Meditate => "Meditate",
            Self::Metronome => "Metronome",
            Self::Minimize => "Minimize",
            Self::MirrorMove => "Mirror Move",
            Self::Peck => "Peck",
            Self::PetalDance => "Petal Dance",
            Self::PinMissile => "Pin Missile",
            Self::PoisonGas => "Poison Gas",
            Self::PoisonSting => "Poison Sting",
            Self::Poisonpowder => "Poisonpowder",
            Self::Pound => "Pound",
            Self::Psybeam => "Psybeam",
            Self::Psychic => "Psychic",
            Self::QuickAttack => "Quick Attack",
            Self::Rage => "Rage",
            Self::RazorLeaf => "Razor Leaf",
            Self::Recover => "Recover",
            Self::Reflect => "Reflect",
            Self::Rest => "Rest",
            Self::RockThrow => "Rock Throw",
            Self::SandAttack => "Sand-Attack",
            Self::Scratch => "Scratch",
            Self::Screech => "Screech",
            Self::SeismicToss => "Seismic Toss",
            Self::Selfdestruct => "Selfdestruct",
            Self::Sing => "Sing",
            Self::SkullBash => "Skull Bash",
            Self::Slam => "Slam",
            Self::Slash => "Slash",
            Self::SleepPowder => "Sleep Powder",
            Self::Sludge => "Sludge",
            Self::Smog => "Smog",
            Self::Smokescreen => "Smokescreen",
            Self::Solarbeam => "Solarbeam",
            Self::Sonicboom => "Sonicboom",
            Self::SpikeCannon => "Spike Cannon",
            Self::Spore => "Spore",
            Self::StringShot => "String Shot",
            Self::Struggle => "Struggle",
            Self::StunSpore => "Stun Spore",
            Self::Submission => "Submission",
            Self::SuperFang => "Super Fang",
            Self::Supersonic => "Supersonic",
            Self::Swift => "Swift",
            Self::Tackle => "Tackle",
            Self::TailWhip => "Tail Whip",
            Self::Teleport => "Teleport",
            Self::Thrash => "Thrash",
            Self::Thunder => "Thunder",
            Self::ThunderWave => "Thunder Wave",
            Self::Thundershock => "Thundershock",
            Self::Twineedle => "Twineedle",
            Self::VineWhip => "Vine Whip",
            Self::WaterGun => "Water Gun",
            Self::Waterfall => "Waterfall",
            Self::Whirlwind => "Whirlwind",
            Self::WingAttack => "Wing Attack",
            Self::Withdraw => "Withdraw",
            Self::Wrap => "Wrap",
        }
    }
}

pub enum SpeciesKind {
    Abra,
    Arbok,
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
    Diglett,
    Drowzee,
    Dugtrio,
    Ekans,
    Electrode,
    Fearow,
    Geodude,
    Gloom,
    Golbat,
    Goldeen,
    Graveler,
    Grimer,
    Horsea,
    Hypno,
    Ivysaur,
    Jigglypuff,
    Kadabra,
    Kakuna,
    Koffing,
    Machoke,
    Machop,
    Magnemite,
    Magneton,
    Mankey,
    Metapod,
    Muk,
    Nidoking,
    Nidoqueen,
    NidoranF,
    NidoranM,
    Nidorina,
    Nidorino,
    Onix,
    Oddish,
    Paras,
    Parasect,
    Pidgeot,
    Pidgeotto,
    Pidgey,
    Pikachu,
    Primeape,
    Raichu,
    Raticate,
    Rattata,
    Sandshrew,
    Sandslash,
    Seadra,
    Seaking,
    Shellder,
    Slowbro,
    Slowpoke,
    Spearow,
    Squirtle,
    Starmie,
    Staryu,
    Venusaur,
    Victreebel,
    Vileplume,
    Voltorb,
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
            Self::Diglett => "Diglett",
            Self::Drowzee => "Drowzee",
            Self::Dugtrio => "Dugtrio",
            Self::Ekans => "Ekans",
            Self::Electrode => "Electrode",
            Self::Fearow => "Fearow",
            Self::Geodude => "Geodude",
            Self::Gloom => "Gloom",
            Self::Golbat => "Golbat",
            Self::Goldeen => "Goldeen",
            Self::Graveler => "Graveler",
            Self::Grimer => "Grimer",
            Self::Horsea => "Horsea",
            Self::Hypno => "Hypno",
            Self::Ivysaur => "Ivysaur",
            Self::Jigglypuff => "Jigglypuff",
            Self::Kadabra => "Kadabra",
            Self::Kakuna => "Kakuna",
            Self::Koffing => "Koffing",
            Self::Machoke => "Machoke",
            Self::Machop => "Machop",
            Self::Magnemite => "Magnemite",
            Self::Magneton => "Magneton",
            Self::Mankey => "Mankey",
            Self::Metapod => "Metapod",
            Self::Muk => "Muk",
            Self::Nidoking => "Nidoking",
            Self::Nidoqueen => "Nidoqueen",
            Self::NidoranF => "Nidoran\u{2640}",
            Self::NidoranM => "Nidoran\u{2642}",
            Self::Nidorina => "Nidorina",
            Self::Nidorino => "Nidorino",
            Self::Oddish => "Oddish",
            Self::Onix => "Onix",
            Self::Paras => "Paras",
            Self::Parasect => "Parasect",
            Self::Pidgeot => "Pidgeot",
            Self::Pidgeotto => "Pidgeotto",
            Self::Pidgey => "Pidgey",
            Self::Pikachu => "Pikachu",
            Self::Primeape => "Primeape",
            Self::Raichu => "Raichu",
            Self::Raticate => "Raticate",
            Self::Rattata => "Rattata",
            Self::Sandshrew => "Sandshrew",
            Self::Sandslash => "Sandslash",
            Self::Seadra => "Seadra",
            Self::Seaking => "Seaking",
            Self::Shellder => "Shellder",
            Self::Slowbro => "Slowbro",
            Self::Slowpoke => "Slowpoke",
            Self::Spearow => "Spearow",
            Self::Squirtle => "Squirtle",
            Self::Starmie => "Starmie",
            Self::Staryu => "Staryu",
            Self::Venusaur => "Venusaur",
            Self::Victreebel => "Victreebel",
            Self::Vileplume => "Vileplume",
            Self::Voltorb => "Voltorb",
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
    // Dragon,
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
        matches!(self, Self::Electric | Self::Fire | Self::Grass | Self::Ice | Self::Psychic | Self::Water) // Dragon
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
            (&Self::Electric, &Self::Water) => (0, 20),
            (&Self::Electric, &Self::Electric | &Self::Grass) => (1, 5),
            (&Self::Electric, &Self::Ground) => (2, 0),
            (&Self::Electric, &Self::Flying) => (3, 20),
            (&Self::Fighting, &Self::Normal) => (0, 20),
            (&Self::Fighting, &Self::Poison | &Self::Flying | &Self::Psychic | &Self::Bug) => (1, 5),
            (&Self::Fighting, &Self::Rock | &Self::Ice) => (2, 20),
            (&Self::Fighting, &Self::Ghost) => (3, 0),
            (&Self::Fire, &Self::Grass | &Self::Ice) => (0, 20),
            (&Self::Fire, &Self::Fire | &Self::Water) => (1, 5),
            (&Self::Fire, &Self::Bug) => (2, 20),
            (&Self::Fire, &Self::Rock) => (3, 5),
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
            (&Self::Ground, &Self::Flying) => (0, 0),
            (&Self::Ground, &Self::Fire | &Self::Electric) => (1, 20),
            (&Self::Ground, &Self::Grass | &Self::Bug) => (2, 5),
            (&Self::Ground, &Self::Rock | &Self::Poison) => (3, 20),
            (&Self::Ice, &Self::Ice | &Self::Water) => (0, 5),
            (&Self::Ice, &Self::Grass | &Self::Ground | &Self::Flying) => (1, 20),
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
    pub(super) const fn accuracy(&self) -> u8 {
        (self.accuracy as u16 * 255 / 100) as u8
    }

    pub(super) const fn name(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(super) const fn priority(&self) -> u8 {
        if matches!(self.kind, MoveKind::QuickAttack) { 1 } else { 0 }
    }

    pub(super) fn crit_probability(&self, base_spd: u8) -> f64 {
        let mut threshold = u16::from(base_spd / 2);
        if matches!(self.kind, MoveKind::KarateChop | MoveKind::RazorLeaf | MoveKind::Slash) {
            threshold = (threshold * 8).min(255);
        }
        f64::from(threshold) / 256.0
    }

    pub(super) fn hit_probability(&self, probability: f64, target_types: &[Type; 2], is_ai: bool) -> f64 {
        if matches!(self.effect, Effect::Paralyze) && self.mv_type == Type::Electric && target_types.contains(&Type::Ground) {
            return 0.0;
        }
        self.effect.hit_probability(probability, is_ai)
    }

    pub(super) fn recoil_damage(&self, damage_dealt: u16) -> u16 {
        let divisor = if self == &STRUGGLE { 2 } else { 4 };
        self.effect.recoil_damage(damage_dealt, divisor)
    }

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
