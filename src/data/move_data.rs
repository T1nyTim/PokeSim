use core::hash::{Hash, Hasher};

use crate::data::{Type, effects::Effect, moves::STRUGGLE};

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
    pub(crate) const fn accuracy(&self) -> u8 {
        (self.accuracy as u16 * 255 / 100) as u8
    }

    pub(crate) const fn name(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(crate) const fn priority(&self) -> u8 {
        if matches!(self.kind, MoveKind::QuickAttack) { 1 } else { 0 }
    }

    pub(crate) fn crit_probability(&self, base_spd: u8) -> f64 {
        let mut threshold = u16::from(base_spd / 2);
        if matches!(self.kind, MoveKind::KarateChop | MoveKind::RazorLeaf | MoveKind::Slash) {
            threshold = (threshold * 8).min(255);
        }
        f64::from(threshold) / 256.0
    }

    pub(crate) fn hit_probability(&self, probability: f64, target_types: &[Type; 2], is_ai: bool) -> f64 {
        if matches!(self.effect, Effect::Paralyze) && self.mv_type == Type::Electric && target_types.contains(&Type::Ground) {
            return 0.0;
        }
        self.effect.hit_probability(probability, is_ai)
    }

    pub(crate) fn recoil_damage(&self, damage_dealt: u16) -> u16 {
        let divisor = if self == &STRUGGLE { 2 } else { 4 };
        self.effect.recoil_damage(damage_dealt, divisor)
    }

    pub(super) const fn new(kind: MoveKind, effect: Effect, power: u8, mv_type: Type, accuracy: u8, pp: u8) -> Self {
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
