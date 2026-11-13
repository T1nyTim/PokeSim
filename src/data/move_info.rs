use core::hash::{Hash, Hasher};

use crate::data::{Effect, STRUGGLE, Type};

#[derive(Eq, Hash, PartialEq)]
pub enum MoveKind {
    Absorb,
    Acid,
    AcidArmor,
    Agility,
    Amnesia,
    AuroraBeam,
    Barrage,
    Barrier,
    Bide,
    Bind,
    Bite,
    Blizzard,
    BodySlam,
    BoneClub,
    Bonemerang,
    Bubble,
    Bubblebeam,
    Clamp,
    CometPunch,
    ConfuseRay,
    Confusion,
    Constrict,
    Counter,
    DefenseCurl,
    Dig,
    Disable,
    DizzyPunch,
    DoubleEdge,
    DoubleKick,
    DoubleTeam,
    Doubleslap,
    DragonRage,
    DreamEater,
    DrillPeck,
    Earthquake,
    EggBomb,
    Ember,
    Explosion,
    FireBlast,
    FireSpin,
    Fissure,
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
    HyperBeam,
    HyperFang,
    Hypnosis,
    IceBeam,
    KarateChop,
    LeechLife,
    LeechSeed,
    Leer,
    Lick,
    LightScreen,
    LowKick,
    Meditate,
    MegaDrain,
    MegaKick,
    MegaPunch,
    Metronome,
    Mimic,
    Minimize,
    MirrorMove,
    NightShade,
    PayDay,
    Peck,
    PetalDance,
    PinMissile,
    PoisonGas,
    PoisonSting,
    Poisonpowder,
    Pound,
    Psybeam,
    Psychic,
    Psywave,
    QuickAttack,
    Rage,
    RazorLeaf,
    RazorWind,
    Recover,
    Reflect,
    Rest,
    Roar,
    RockSlide,
    RockThrow,
    SandAttack,
    Scratch,
    Screech,
    SeismicToss,
    Selfdestruct,
    Sing,
    SkullBash,
    SkyAttack,
    Slam,
    Slash,
    SleepPowder,
    Sludge,
    Smog,
    Smokescreen,
    Solarbeam,
    Sonicboom,
    SpikeCannon,
    Splash,
    Spore,
    Stomp,
    StringShot,
    Struggle,
    StunSpore,
    Submission,
    Substitute,
    SuperFang,
    Supersonic,
    Swift,
    SwordsDance,
    Tackle,
    TailWhip,
    TakeDown,
    Teleport,
    Thrash,
    Thunder,
    ThunderWave,
    Thunderbolt,
    Thundershock,
    Toxic,
    TriAttack,
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
            Self::AuroraBeam => "Aurora Beam",
            Self::Barrage => "Barrage",
            Self::Barrier => "Barrier",
            Self::Bide => "Bide",
            Self::Bind => "Bind",
            Self::Bite => "Bite",
            Self::Blizzard => "Blizzard",
            Self::BodySlam => "Body Slam",
            Self::BoneClub => "Bone Club",
            Self::Bonemerang => "Bonemerang",
            Self::Bubble => "Bubble",
            Self::Bubblebeam => "Bubblebeam",
            Self::Clamp => "Clamp",
            Self::CometPunch => "Comet Punch",
            Self::ConfuseRay => "Confuse Ray",
            Self::Confusion => "Confusion",
            Self::Constrict => "Constrict",
            Self::Counter => "Counter",
            Self::DefenseCurl => "Defense Curl",
            Self::Dig => "Dig",
            Self::Disable => "Disable",
            Self::DizzyPunch => "Dizzy Punch",
            Self::DoubleEdge => "Double-Edge",
            Self::DoubleKick => "Double Kick",
            Self::DoubleTeam => "Double Team",
            Self::Doubleslap => "Doubleslap",
            Self::DragonRage => "Dragon Rage",
            Self::DreamEater => "Dream Eater",
            Self::DrillPeck => "Drill Peck",
            Self::Earthquake => "Earthquake",
            Self::EggBomb => "Egg Bomb",
            Self::Ember => "Ember",
            Self::Explosion => "Explosion",
            Self::FireBlast => "Fire Blast",
            Self::FireSpin => "Fire Spin",
            Self::Fissure => "Fissure",
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
            Self::HyperBeam => "Hyper Beam",
            Self::HyperFang => "Hyper Fang",
            Self::Hypnosis => "Hypnosis",
            Self::IceBeam => "Ice Beam",
            Self::KarateChop => "Karate Chop",
            Self::LeechLife => "Leech Life",
            Self::LeechSeed => "Leech Seed",
            Self::Leer => "Leer",
            Self::Lick => "Lick",
            Self::LightScreen => "Light Screen",
            Self::LowKick => "Low Kick",
            Self::Meditate => "Meditate",
            Self::MegaDrain => "Mega Drain",
            Self::MegaKick => "Mega Kick",
            Self::MegaPunch => "Mega Punch",
            Self::Metronome => "Metronome",
            Self::Mimic => "Mimic",
            Self::Minimize => "Minimize",
            Self::MirrorMove => "Mirror Move",
            Self::NightShade => "Night Shade",
            Self::PayDay => "Pay Day",
            Self::Peck => "Peck",
            Self::PetalDance => "Petal Dance",
            Self::PinMissile => "Pin Missile",
            Self::PoisonGas => "Poison Gas",
            Self::PoisonSting => "Poison Sting",
            Self::Poisonpowder => "Poisonpowder",
            Self::Pound => "Pound",
            Self::Psybeam => "Psybeam",
            Self::Psychic => "Psychic",
            Self::Psywave => "Psywave",
            Self::QuickAttack => "Quick Attack",
            Self::Rage => "Rage",
            Self::RazorLeaf => "Razor Leaf",
            Self::RazorWind => "Razor Wind",
            Self::Recover => "Recover",
            Self::Reflect => "Reflect",
            Self::Rest => "Rest",
            Self::Roar => "Roar",
            Self::RockSlide => "Rock Slide",
            Self::RockThrow => "Rock Throw",
            Self::SandAttack => "Sand-Attack",
            Self::Scratch => "Scratch",
            Self::Screech => "Screech",
            Self::SeismicToss => "Seismic Toss",
            Self::Selfdestruct => "Selfdestruct",
            Self::Sing => "Sing",
            Self::SkullBash => "Skull Bash",
            Self::SkyAttack => "Sky Attack",
            Self::Slam => "Slam",
            Self::Slash => "Slash",
            Self::SleepPowder => "Sleep Powder",
            Self::Sludge => "Sludge",
            Self::Smog => "Smog",
            Self::Smokescreen => "Smokescreen",
            Self::Solarbeam => "Solarbeam",
            Self::Sonicboom => "Sonicboom",
            Self::SpikeCannon => "Spike Cannon",
            Self::Splash => "Splash",
            Self::Spore => "Spore",
            Self::Stomp => "Stomp",
            Self::StringShot => "String Shot",
            Self::Struggle => "Struggle",
            Self::StunSpore => "Stun Spore",
            Self::Submission => "Submission",
            Self::Substitute => "Substitute",
            Self::SuperFang => "Super Fang",
            Self::Supersonic => "Supersonic",
            Self::Swift => "Swift",
            Self::SwordsDance => "Swords Dance",
            Self::Tackle => "Tackle",
            Self::TailWhip => "Tail Whip",
            Self::TakeDown => "Take Down",
            Self::Teleport => "Teleport",
            Self::Thrash => "Thrash",
            Self::Thunder => "Thunder",
            Self::ThunderWave => "Thunder Wave",
            Self::Thunderbolt => "Thunderbolt",
            Self::Thundershock => "Thundershock",
            Self::Toxic => "Toxic",
            Self::TriAttack => "Tri Attack",
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

    pub(crate) const fn fixed_damage(&self, level: u8) -> Option<u16> {
        if matches!(self.kind, MoveKind::NightShade | MoveKind::SeismicToss) {
            Some(level as u16)
        } else if matches!(self.kind, MoveKind::Sonicboom) {
            Some(20)
        } else if matches!(self.kind, MoveKind::DragonRage) {
            Some(40)
        } else {
            None
        }
    }

    pub(crate) const fn name(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(crate) const fn priority(&self) -> u8 {
        if matches!(self.kind, MoveKind::QuickAttack) { 1 } else { 0 }
    }

    pub(crate) fn crit_probability(&self, base_spd: u8, focus_energy: bool) -> f64 {
        let mut threshold = u16::from(base_spd / 2);
        threshold = if focus_energy { threshold / 2 } else { threshold * 2 };
        if matches!(self.kind, MoveKind::KarateChop | MoveKind::RazorLeaf | MoveKind::Slash) {
            threshold = (threshold * 4).min(255);
        } else {
            threshold /= 2;
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
