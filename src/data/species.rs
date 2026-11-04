#![cfg_attr(rustfmt, rustfmt::skip)]
use crate::data::{BaseStats, Evolve, GrowthRate, Item, Species, SpeciesKind, Type, moves::*};

pub const ABRA: Species = Species::new(
    SpeciesKind::Abra, BaseStats::new(25, 20, 15, 90, 105), [Type::Psychic, Type::Psychic], 73,
    [Some(&TELEPORT), None, None, None], GrowthRate::MediumSlow, &[(Evolve::Level(16), &KADABRA)],
    &[]
);
pub const ARBOK: Species = Species::new(
    SpeciesKind::Arbok, BaseStats::new(60, 85, 69, 80, 65), [Type::Poison, Type::Poison], 147,
    [Some(&WRAP), Some(&LEER), Some(&POISON_STING), None], GrowthRate::MediumFast, &[],
    &[(10, &POISON_STING), (17, &BITE), (27, &GLARE), (36, &SCREECH), (47, &ACID)]
);
pub const BEEDRILL: Species = Species::new(
    SpeciesKind::Beedrill, BaseStats::new(65, 80, 40, 75, 45), [Type::Bug, Type::Poison], 159,
    [Some(&FURY_ATTACK), None, None, None], GrowthRate::MediumFast, &[],
    &[(12, &FURY_ATTACK), (16, &FOCUS_ENERGY), (20, &TWINEEDLE), (25, &RAGE), (30, &PIN_MISSILE), (35, &AGILITY)]
);
pub const BELLSPROUT: Species = Species::new(
    SpeciesKind::Bellsprout, BaseStats::new(50, 75, 35, 40, 70), [Type::Grass, Type::Poison], 84,
    [Some(&VINE_WHIP), Some(&GROWTH), None, None], GrowthRate::MediumSlow, &[(Evolve::Level(21), &WEEPINBELL)],
    &[(13, &WRAP), (15, &POISONPOWDER), (18, &SLEEP_POWDER), (21, &STUN_SPORE), (26, &ACID), (33, &RAZOR_LEAF), (42, &SLAM)]
);
pub const BLASTOISE: Species = Species::new(
    SpeciesKind::Blastoise, BaseStats::new(79, 83, 100, 78, 85), [Type::Water, Type::Water], 210,
    [Some(&TACKLE), Some(&TAIL_WHIP), Some(&BUBBLE), Some(&WATER_GUN)], GrowthRate::MediumSlow, &[],
    &[(8, &BUBBLE), (15, &WATER_GUN), (24, &BITE), (31, &WITHDRAW), (42, &SKULL_BASH), (52, &HYDRO_PUMP)]
);
pub const BULBASAUR: Species = Species::new(
    SpeciesKind::Bulbasaur, BaseStats::new(45, 49, 49, 45, 65), [Type::Grass, Type::Poison], 64,
    [Some(&TACKLE), Some(&GROWL), None, None], GrowthRate::MediumSlow, &[(Evolve::Level(16), &IVYSAUR)],
    &[(7, &LEECH_SEED), (13, &VINE_WHIP), (20, &POISONPOWDER), (27, &RAZOR_LEAF), (34, &GROWTH), (41, &SLEEP_POWDER), (48, &SOLARBEAM)]
);
pub const BUTTERFREE: Species = Species::new(
    SpeciesKind::Butterfree, BaseStats::new(60, 45, 50, 70, 80), [Type::Bug, Type::Flying], 160,
    [Some(&CONFUSION), None, None, None], GrowthRate::MediumFast, &[],
    &[(12, &CONFUSION), (15, &POISONPOWDER), (16, &STUN_SPORE), (17, &SLEEP_POWDER), (21, &SUPERSONIC), (26, &WHIRLWIND), (32, &PSYBEAM)]
);
pub const CATERPIE: Species = Species::new(
    SpeciesKind::Caterpie, BaseStats::new(45, 30, 35, 45, 20), [Type::Bug, Type::Bug], 53,
    [Some(&TACKLE), Some(&STRING_SHOT), None, None], GrowthRate::MediumFast, &[(Evolve::Level(7), &METAPOD)],
    &[],
);
pub const CHARIZARD: Species = Species::new(
    SpeciesKind::Charizard, BaseStats::new(78, 84, 78, 100, 85), [Type::Fire, Type::Flying], 209,
    [Some(&SCRATCH), Some(&GROWL), Some(&EMBER), Some(&LEER)], GrowthRate::MediumSlow, &[],
    &[(9, &EMBER), (15, &LEER), (24, &RAGE), (36, &SLASH), (46, &FLAMETHROWER), (55, &FIRE_SPIN)]
);
pub const CHARMANDER: Species = Species::new(
    SpeciesKind::Charmander, BaseStats::new(39, 52, 43, 65, 50), [Type::Fire, Type::Fire], 65,
    [Some(&SCRATCH), Some(&GROWL), None, None], GrowthRate::MediumSlow, &[(Evolve::Level(16), &CHARMELEON)],
    &[(9, &EMBER), (15, &LEER), (22, &RAGE), (30, &SLASH), (38, &FLAMETHROWER), (46, &FIRE_SPIN)]
);
pub const CHARMELEON: Species = Species::new(
    SpeciesKind::Charmeleon, BaseStats::new(58, 64, 58, 80, 65), [Type::Fire, Type::Fire], 142,
    [Some(&SCRATCH), Some(&GROWL), Some(&EMBER), None], GrowthRate::MediumSlow, &[(Evolve::Level(36), &CHARIZARD)],
    &[(9, &EMBER), (15, &LEER), (24, &RAGE), (33, &SLASH), (42, &FLAMETHROWER), (56, &FIRE_SPIN)]
);
pub const CLEFABLE: Species = Species::new(
    SpeciesKind::Clefable, BaseStats::new(95, 70, 73, 60, 85), [Type::Normal, Type::Normal], 129,
    [Some(&SING), Some(&DOUBLESLAP), Some(&MINIMIZE), Some(&METRONOME)], GrowthRate::Fast, &[],
    &[]
);
pub const CLEFAIRY: Species = Species::new(
    SpeciesKind::Clefairy, BaseStats::new(70, 45, 48, 35, 60), [Type::Normal, Type::Normal], 68,
    [Some(&POUND), Some(&GROWL), None, None], GrowthRate::Fast, &[(Evolve::Item(Item::MoonStone), &CLEFABLE)],
    &[(13, &SING), (18, &DOUBLESLAP), (24, &MINIMIZE), (31, &METRONOME), (39, &DEFENSE_CURL), (48, &LIGHT_SCREEN)]
);
pub const CLOYSTER: Species = Species::new(
    SpeciesKind::Cloyster, BaseStats::new(50, 95, 180, 70, 85), [Type::Water, Type::Ice], 203,
    [Some(&WITHDRAW), Some(&SUPERSONIC), Some(&CLAMP), Some(&AURORA_BEAM)], GrowthRate::Slow, &[],
    &[(50, &SPIKE_CANNON)]
);
pub const DIGLETT: Species = Species::new(
    SpeciesKind::Diglett, BaseStats::new(10, 55, 25, 95, 45), [Type::Ground, Type::Ground], 81,
    [Some(&SCRATCH), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(26), &DUGTRIO)],
    &[(15, &GROWL), (19, &DIG), (24, &SAND_ATTACK), (31, &SLASH), (40, &EARTHQUAKE)]
);
pub const DROWZEE: Species = Species::new(
    SpeciesKind::Drowzee, BaseStats::new(60, 48, 45, 42, 90), [Type::Psychic, Type::Psychic], 102,
    [Some(&POUND), Some(&HYPNOSIS), None, None], GrowthRate::MediumFast, &[(Evolve::Level(26), &HYPNO)],
    &[(12, &DISABLE), (17, &CONFUSION), (24, &HEADBUTT), (29, &POISON_GAS), (32, &PSYCHIC), (37, &MEDITATE)]
);
pub const DUGTRIO: Species = Species::new(
    SpeciesKind::Dugtrio, BaseStats::new(35, 80, 50, 120, 70), [Type::Ground, Type::Ground], 153,
    [Some(&SCRATCH), Some(&GROWL), Some(&DIG), None], GrowthRate::MediumFast, &[],
    &[(15, &GROWL), (19, &DIG), (24, &SAND_ATTACK), (31, &SLASH), (40, &EARTHQUAKE)]
);
pub const EKANS: Species = Species::new(
    SpeciesKind::Ekans, BaseStats::new(35, 60, 44, 55, 40), [Type::Poison, Type::Poison], 62,
    [Some(&WRAP), Some(&LEER), None, None], GrowthRate::MediumFast, &[(Evolve::Level(22), &ARBOK)],
    &[(10, &POISON_STING), (17, &BITE), (24, &GLARE), (31, &SCREECH), (38, &ACID)]
);
pub const ELECTRODE: Species = Species::new(
    SpeciesKind::Electrode, BaseStats::new(60, 50, 70, 140, 80), [Type::Electric, Type::Electric], 150,
    [Some(&TACKLE), Some(&SCREECH), Some(&SONICBOOM), None], GrowthRate::MediumFast, &[],
    &[(17, &SONICBOOM), (22, &SELFDESTRUCT), (29, &LIGHT_SCREEN), (40, &SWIFT), (50, &EXPLOSION)]
);
pub const FEAROW: Species = Species::new(
    SpeciesKind::Fearow, BaseStats::new(65, 90, 65, 100, 61), [Type::Normal, Type::Flying], 162,
    [Some(&PECK), Some(&GROWL), Some(&LEER), None], GrowthRate::MediumFast, &[],
    &[(9, &LEER), (15, &FURY_ATTACK), (25, &MIRROR_MOVE), (34, &DRILL_PECK), (43, &AGILITY)]
);
pub const GEODUDE: Species = Species::new(
    SpeciesKind::Geodude, BaseStats::new(40, 80, 100, 20, 30), [Type::Rock, Type::Ground], 86,
    [Some(&TACKLE), None, None, None], GrowthRate::MediumSlow, &[(Evolve::Level(25), &GRAVELER)],
    &[(11, &DEFENSE_CURL), (16, &ROCK_THROW), (21, &SELFDESTRUCT), (26, &HARDEN), (31, &EARTHQUAKE), (36, &EXPLOSION)]
);
pub const GLOOM: Species = Species::new(
    SpeciesKind::Gloom, BaseStats::new(60, 65, 70, 40, 85), [Type::Grass, Type::Poison], 132,
    [Some(&ABSORB), Some(&POISONPOWDER), Some(&STUN_SPORE), None], GrowthRate::MediumSlow, &[(Evolve::Item(Item::LeafStone), &VILEPLUME)],
    &[(15, &POISONPOWDER), (17, &STUN_SPORE), (19, &SLEEP_POWDER), (28, &ACID), (38, &PETAL_DANCE), (52, &SOLARBEAM)]
);
pub const GOLBAT: Species = Species::new(
    SpeciesKind::Golbat, BaseStats::new(75, 80, 70, 90, 75), [Type::Poison, Type::Flying], 171,
    [Some(&LEECH_LIFE), Some(&SCREECH), Some(&BITE), None], GrowthRate::MediumFast, &[],
    &[(10, &SUPERSONIC), (15, &BITE), (21, &CONFUSE_RAY), (32, &WING_ATTACK), (32, &HAZE)]
);
pub const GOLDEEN: Species = Species::new(
    SpeciesKind::Goldeen, BaseStats::new(45, 67, 60, 63, 50), [Type::Water, Type::Water], 111,
    [Some(&PECK), Some(&TAIL_WHIP), None, None], GrowthRate::MediumFast, &[(Evolve::Level(33), &SEAKING)],
    &[(19, &SUPERSONIC), (24, &HORN_ATTACK), (30, &FURY_ATTACK), (37, &WATERFALL), (45, &HORN_DRILL), (54, &AGILITY)]
);
pub const GRAVELER: Species = Species::new(
    SpeciesKind::Graveler, BaseStats::new(55, 95, 115, 35, 45), [Type::Rock, Type::Ground], 134,
    [Some(&TACKLE), Some(&DEFENSE_CURL), None, None], GrowthRate::MediumSlow, &[],
    &[(11, &DEFENSE_CURL), (16, &ROCK_THROW), (21, &SELFDESTRUCT), (29, &HARDEN), (36, &EARTHQUAKE), (43, &EXPLOSION)]
);
pub const GRIMER: Species = Species::new(
    SpeciesKind::Grimer, BaseStats::new(80, 80, 50, 25, 40), [Type::Poison, Type::Poison], 90,
    [Some(&POUND), Some(&DISABLE), None, None], GrowthRate::MediumFast, &[(Evolve::Level(38), &MUK)],
    &[(30, &POISON_GAS), (33, &MINIMIZE), (37, &SLUDGE), (42, &HARDEN), (48, &SCREECH), (55, &ACID_ARMOR)]
);
pub const HORSEA: Species = Species::new(
    SpeciesKind::Horsea, BaseStats::new(30, 40, 70, 60, 70), [Type::Water, Type::Water], 83,
    [Some(&BUBBLE), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(32), &SEADRA)],
    &[(19, &SMOKESCREEN), (24, &LEER), (30, &WATER_GUN), (37, &AGILITY), (45, &HYDRO_PUMP)]
);
pub const HYPNO: Species = Species::new(
    SpeciesKind::Hypno, BaseStats::new(85, 73, 70, 67, 115), [Type::Psychic, Type::Psychic], 165,
    [Some(&POUND), Some(&HYPNOSIS), Some(&DISABLE), Some(&CONFUSION)], GrowthRate::MediumFast, &[],
    &[(12, &DISABLE), (17, &CONFUSION), (24, &HEADBUTT), (33, &POISON_GAS), (37, &PSYCHIC), (43, &MEDITATE)]
);
pub const IVYSAUR: Species = Species::new(
    SpeciesKind::Ivysaur, BaseStats::new(60, 62, 63, 60, 80), [Type::Grass, Type::Poison], 141,
    [Some(&TACKLE), Some(&GROWL), Some(&LEECH_SEED), None], GrowthRate::MediumSlow, &[(Evolve::Level(32), &VENUSAUR)],
    &[(7, &LEECH_SEED), (13, &VINE_WHIP), (22, &POISONPOWDER), (30, &RAZOR_LEAF), (38, &GROWTH), (46, &SLEEP_POWDER), (54, &SOLARBEAM)]
);
pub const JIGGLYPUFF: Species = Species::new(
    SpeciesKind::Jigglypuff, BaseStats::new(115, 45, 20, 20, 25), [Type::Normal, Type::Normal], 76,
    [Some(&SING), None, None, None], GrowthRate::Fast, &[(Evolve::Item(Item::MoonStone), &WIGGLYTUFF)],
    &[(9, &POUND), (14, &DISABLE), (19, &DEFENSE_CURL), (24, &DOUBLESLAP), (29, &REST), (34, &BODY_SLAM), (39, &DOUBLE_EDGE)]
);
pub const KADABRA: Species = Species::new(
    SpeciesKind::Kadabra, BaseStats::new(40, 35, 30, 105, 120), [Type::Psychic, Type::Psychic], 145,
    [Some(&TELEPORT), Some(&CONFUSION), Some(&DISABLE), None], GrowthRate::MediumSlow, &[],
    &[(16, &CONFUSION), (20, &DISABLE), (27, &PSYBEAM), (31, &RECOVER), (38, &PSYCHIC), (42, &REFLECT)]
);
pub const KAKUNA: Species = Species::new(
    SpeciesKind::Kakuna, BaseStats::new(45, 25, 50, 35, 25), [Type::Bug, Type::Poison], 71,
    [Some(&HARDEN), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(10), &BEEDRILL)],
    &[]
);
pub const KOFFING: Species = Species::new(
    SpeciesKind::Koffing, BaseStats::new(40, 65, 95, 35, 60), [Type::Poison, Type::Poison], 114,
    [Some(&TACKLE), Some(&SMOG), None, None], GrowthRate::MediumFast, &[(Evolve::Level(35), &WEEZING)],
    &[(32, &SLUDGE), (37, &SMOKESCREEN), (40, &SELFDESTRUCT), (45, &HAZE), (48, &EXPLOSION)]
);
pub const MACHOKE: Species = Species::new(
    SpeciesKind::Machoke, BaseStats::new(80, 100, 70, 45, 50), [Type::Fighting, Type::Fighting], 146,
    [Some(&KARATE_CHOP), Some(&LOW_KICK), Some(&LEER), None], GrowthRate::MediumSlow, &[],
    &[(20, &LOW_KICK), (25, &LEER), (36, &FOCUS_ENERGY), (44, &SEISMIC_TOSS), (52, &SUBMISSION)]
);
pub const MACHOP: Species = Species::new(
    SpeciesKind::Machop, BaseStats::new(70, 80, 50, 35, 35), [Type::Fighting, Type::Fighting], 88,
    [Some(&KARATE_CHOP), None, None, None], GrowthRate::MediumSlow, &[(Evolve::Level(28), &MACHOKE)],
    &[(20, &LOW_KICK), (25, &LEER), (32, &FOCUS_ENERGY), (39, &SEISMIC_TOSS), (46, &SUBMISSION)]
);
pub const MAGNEMITE: Species = Species::new(
    SpeciesKind::Magnemite, BaseStats::new(25, 35, 70, 45, 95), [Type::Electric, Type::Electric], 89,
    [Some(&TACKLE), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(30), &MAGNETON)],
    &[(21, &SONICBOOM), (25, &THUNDERSHOCK), (29, &SUPERSONIC), (35, &THUNDER_WAVE), (41, &SWIFT), (47, &SCREECH)]
);
pub const MAGNETON: Species = Species::new(
    SpeciesKind::Magneton, BaseStats::new(50, 60, 95, 70, 120), [Type::Electric, Type::Electric], 161,
    [Some(&TACKLE), Some(&SONICBOOM), Some(&THUNDERSHOCK), None], GrowthRate::MediumFast, &[],
    &[(21, &SONICBOOM), (25, &THUNDERSHOCK), (29, &SUPERSONIC), (38, &THUNDER_WAVE), (46, &SWIFT), (54, &SCREECH)]
);
pub const MANKEY: Species = Species::new(
    SpeciesKind::Mankey, BaseStats::new(40, 80, 35, 70, 35), [Type::Fighting, Type::Fighting], 74,
    [Some(&SCRATCH), Some(&LEER), None, None], GrowthRate::MediumFast, &[(Evolve::Level(28), &PRIMEAPE)],
    &[(15, &KARATE_CHOP), (21, &FURY_SWIPES), (27, &FOCUS_ENERGY), (33, &SEISMIC_TOSS), (39, &THRASH)]
);
pub const METAPOD: Species = Species::new(
    SpeciesKind::Metapod, BaseStats::new(50, 20, 55, 30, 25), [Type::Bug, Type::Bug], 72,
    [Some(&HARDEN), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(10), &BUTTERFREE)],
    &[]
);
pub const MUK: Species = Species::new(
    SpeciesKind::Muk, BaseStats::new(105, 105, 75, 50, 65), [Type::Poison, Type::Poison], 157,
    [Some(&POUND), Some(&DISABLE), Some(&POISON_GAS), None], GrowthRate::MediumFast, &[],
    &[(30, &POISON_GAS), (33, &MINIMIZE), (37, &SLUDGE), (45, &HARDEN), (53, &SCREECH), (60, &ACID_ARMOR)]
);
pub const NIDOKING: Species = Species::new(
    SpeciesKind::Nidoking, BaseStats::new(81, 92, 77, 85, 75), [Type::Poison, Type::Ground], 195,
    [Some(&TACKLE), Some(&HORN_ATTACK), Some(&POISON_STING), Some(&THRASH)], GrowthRate::MediumSlow, &[],
    &[(8, &HORN_ATTACK), (14, &POISON_STING), (23, &THRASH)]
);
pub const NIDOQUEEN: Species = Species::new(
    SpeciesKind::Nidoqueen, BaseStats::new(90, 82, 87, 76, 75), [Type::Poison, Type::Ground], 194,
    [Some(&TACKLE), Some(&SCRATCH), Some(&TAIL_WHIP), Some(&BODY_SLAM)], GrowthRate::MediumSlow, &[],
    &[(8, &SCRATCH), (14, &POISON_STING), (23, &BODY_SLAM)]
);
pub const NIDORAN_F: Species = Species::new(
    SpeciesKind::NidoranF, BaseStats::new(55, 47, 52, 41, 40), [Type::Poison, Type::Poison], 59,
    [Some(&GROWL), Some(&TACKLE), None, None], GrowthRate::MediumSlow, &[(Evolve::Level(16), &NIDORINA)],
    &[(8, &SCRATCH), (14, &POISON_STING), (21, &TAIL_WHIP), (29, &BITE), (36, &FURY_SWIPES), (43, &DOUBLE_KICK)]
);
pub const NIDORAN_M: Species = Species::new(
    SpeciesKind::NidoranM, BaseStats::new(46, 57, 40, 50, 40), [Type::Poison, Type::Poison], 60,
    [Some(&LEER), Some(&TACKLE), None, None], GrowthRate::MediumSlow, &[(Evolve::Level(16), &NIDORINO)],
    &[(8, &HORN_ATTACK), (14, &POISON_STING), (21, &FOCUS_ENERGY), (29, &FURY_ATTACK), (36, &HORN_DRILL), (43, &DOUBLE_KICK)]
);
pub const NIDORINA: Species = Species::new(
    SpeciesKind::Nidorina, BaseStats::new(70, 62, 67, 56, 55), [Type::Poison, Type::Poison], 117,
    [Some(&GROWL), Some(&TACKLE), Some(&SCRATCH), None], GrowthRate::MediumSlow, &[(Evolve::Item(Item::MoonStone), &NIDOQUEEN)],
    &[(8, &SCRATCH), (14, &POISON_STING), (23, &TAIL_WHIP), (32, &BITE), (41, &FURY_SWIPES), (50, &DOUBLE_KICK)]
);
pub const NIDORINO: Species = Species::new(
    SpeciesKind::Nidorino, BaseStats::new(61, 72, 57, 65, 55), [Type::Poison, Type::Poison], 118,
    [Some(&LEER), Some(&TACKLE), Some(&HORN_ATTACK), None], GrowthRate::MediumSlow, &[(Evolve::Item(Item::MoonStone), &NIDOKING)],
    &[(8, &HORN_ATTACK), (14, &POISON_STING), (23, &FOCUS_ENERGY), (32, &FURY_ATTACK), (41, &HORN_DRILL), (50, &DOUBLE_KICK)]
);
pub const ODDISH: Species = Species::new(
    SpeciesKind::Oddish, BaseStats::new(45, 50, 55, 30, 75), [Type::Grass, Type::Poison], 78,
    [Some(&ABSORB), None, None, None], GrowthRate::MediumSlow, &[(Evolve::Level(21), &GLOOM)],
    &[(15, &POISONPOWDER), (17, &STUN_SPORE), (19, &SLEEP_POWDER), (24, &ACID), (33, &PETAL_DANCE), (46, &SOLARBEAM)]
);
pub const ONIX: Species = Species::new(
    SpeciesKind::Onix, BaseStats::new(35, 45, 160, 70, 30), [Type::Rock, Type::Ground], 108,
    [Some(&TACKLE), Some(&SCREECH), None, None], GrowthRate::MediumFast, &[],
    &[(15, &BIND), (19, &ROCK_THROW), (25, &RAGE), (33, &SLAM), (43, &HARDEN)]
);
pub const PARAS: Species = Species::new(
    SpeciesKind::Paras, BaseStats::new(35, 70, 55, 25, 55), [Type::Bug, Type::Grass], 70,
    [Some(&SCRATCH), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(24), &PARASECT)],
    &[(13, &STUN_SPORE), (20, &LEECH_LIFE), (27, &SPORE), (34, &SLASH), (41, &GROWTH)]
);
pub const PARASECT: Species = Species::new(
    SpeciesKind::Parasect, BaseStats::new(60, 95, 80, 30, 80), [Type::Bug, Type::Grass], 128,
    [Some(&SCRATCH), Some(&STUN_SPORE), Some(&LEECH_LIFE), None], GrowthRate::MediumFast, &[],
    &[(13, &STUN_SPORE), (20, &LEECH_LIFE), (30, &SPORE), (39, &SLASH), (48, &GROWTH)]
);
pub const PIDGEOT: Species = Species::new(
    SpeciesKind::Pidgeot, BaseStats::new(83, 80, 75, 91, 70), [Type::Normal, Type::Flying], 172,
    [Some(&GUST), Some(&SAND_ATTACK), Some(&QUICK_ATTACK), None], GrowthRate::MediumSlow, &[],
    &[(5, &SAND_ATTACK), (12, &QUICK_ATTACK), (21, &WHIRLWIND), (31, &WING_ATTACK), (44, &AGILITY), (54, &MIRROR_MOVE)]
);
pub const PIDGEOTTO: Species = Species::new(
    SpeciesKind::Pidgeotto, BaseStats::new(63, 60, 55, 71, 50), [Type::Normal, Type::Flying], 113,
    [Some(&GUST), Some(&SAND_ATTACK), None, None], GrowthRate::MediumSlow, &[(Evolve::Level(36), &PIDGEOT)],
    &[(5, &SAND_ATTACK), (12, &QUICK_ATTACK), (21, &WHIRLWIND), (31, &WING_ATTACK), (40, &AGILITY), (49, &MIRROR_MOVE)]
);
pub const PIDGEY: Species = Species::new(
    SpeciesKind::Pidgey, BaseStats::new(40, 45, 40, 56, 35), [Type::Normal, Type::Flying], 55,
    [Some(&GUST), None, None, None], GrowthRate::MediumSlow, &[(Evolve::Level(18), &PIDGEOTTO)],
    &[(5, &SAND_ATTACK), (12, &QUICK_ATTACK), (19, &WHIRLWIND), (28, &WING_ATTACK), (36, &AGILITY), (44, &MIRROR_MOVE)]
);
pub const PIKACHU: Species = Species::new(
    SpeciesKind::Pikachu, BaseStats::new(35, 55, 30, 90, 50), [Type::Electric, Type::Electric], 82,
    [Some(&THUNDERSHOCK), Some(&GROWL), None, None], GrowthRate::MediumFast, &[(Evolve::Item(Item::ThunderStone), &RAICHU)],
    &[(9, &THUNDER_WAVE), (16, &QUICK_ATTACK), (26, &SWIFT), (33, &AGILITY), (43, &THUNDER)]
);
pub const PRIMEAPE: Species = Species::new(
    SpeciesKind::Primeape, BaseStats::new(65, 105, 60, 95, 60), [Type::Fighting, Type::Fighting], 149,
    [Some(&SCRATCH), Some(&LEER), Some(&KARATE_CHOP), Some(&FURY_SWIPES)], GrowthRate::MediumFast, &[],
    &[(15, &KARATE_CHOP), (21, &FURY_SWIPES), (27, &FOCUS_ENERGY), (37, &SEISMIC_TOSS), (46, &THRASH)]
);
pub const RAICHU: Species = Species::new(
    SpeciesKind::Raichu, BaseStats::new(60, 90, 55, 100, 90), [Type::Electric, Type::Electric], 122,
    [Some(&THUNDERSHOCK), Some(&GROWL), Some(&THUNDER_WAVE), None], GrowthRate::MediumFast, &[],
    &[]
);
pub const RATICATE: Species = Species::new(
    SpeciesKind::Raticate, BaseStats::new(55, 81, 60, 97, 50), [Type::Normal, Type::Normal], 116,
    [Some(&TACKLE), Some(&TAIL_WHIP), Some(&QUICK_ATTACK), None], GrowthRate::MediumFast, &[],
    &[(7, &QUICK_ATTACK), (14, &HYPER_FANG), (27, &FOCUS_ENERGY), (41, &SUPER_FANG)]
);
pub const RATTATA: Species = Species::new(
    SpeciesKind::Rattata, BaseStats::new(30, 56, 35, 72, 25), [Type::Normal, Type::Normal], 57,
    [Some(&TACKLE), Some(&TAIL_WHIP), None, None], GrowthRate::MediumFast, &[(Evolve::Level(20), &RATICATE)],
    &[(7, &QUICK_ATTACK), (14, &HYPER_FANG), (27, &FOCUS_ENERGY), (41, &SUPER_FANG)]
);
pub const SANDSHREW: Species = Species::new(
    SpeciesKind::Sandshrew, BaseStats::new(50, 75, 85, 40, 30), [Type::Ground, Type::Ground], 93,
    [Some(&SCRATCH), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(22), &SANDSLASH)],
    &[(10, &SAND_ATTACK), (17, &SLASH), (24, &POISON_STING), (31, &SWIFT), (38, &FURY_SWIPES)]
);
pub const SANDSLASH: Species = Species::new(
    SpeciesKind::Sandslash, BaseStats::new(75, 100, 110, 65, 55), [Type::Ground, Type::Ground], 163,
    [Some(&SCRATCH), Some(&SAND_ATTACK), None, None], GrowthRate::MediumFast, &[],
    &[(10, &SAND_ATTACK), (17, &SLASH), (27, &POISON_STING), (36, &SWIFT), (47, &FURY_SWIPES)]
);
pub const SEADRA: Species = Species::new(
    SpeciesKind::Seadra, BaseStats::new(55, 65, 95, 85, 95), [Type::Water, Type::Water], 155,
    [Some(&BUBBLE), Some(&SMOKESCREEN), None, None], GrowthRate::MediumFast, &[],
    &[(19, &SMOKESCREEN), (24, &LEER), (30, &WATER_GUN), (41, &AGILITY), (52, &HYDRO_PUMP)]
);
pub const SEAKING: Species = Species::new(
    SpeciesKind::Seaking, BaseStats::new(80, 92, 65, 68, 80), [Type::Water, Type::Water], 170,
    [Some(&PECK), Some(&TAIL_WHIP), Some(&SUPERSONIC), None], GrowthRate::MediumFast, &[],
    &[(19, &SUPERSONIC), (24, &HORN_ATTACK), (30, &FURY_ATTACK), (39, &WATERFALL), (48, &HORN_DRILL), (54, &AGILITY)]
);
pub const SHELLDER: Species = Species::new(
    SpeciesKind::Shellder, BaseStats::new(30, 65, 100, 40, 45), [Type::Water, Type::Water], 97,
    [Some(&TACKLE), Some(&WITHDRAW), None, None], GrowthRate::Slow, &[(Evolve::Item(Item::WaterStone), &CLOYSTER)],
    &[(18, &SUPERSONIC), (23, &CLAMP), (30, &AURORA_BEAM), (39, &LEER), (50, &ICE_BEAM)]
);
pub const SLOWBRO: Species = Species::new(
    SpeciesKind::Slowbro, BaseStats::new(95, 75, 110, 30, 80), [Type::Water, Type::Psychic], 164,
    [Some(&CONFUSION), Some(&DISABLE), Some(&HEADBUTT), None], GrowthRate::MediumFast, &[],
    &[(18, &DISABLE), (22, &HEADBUTT), (27, &GROWL), (33, &WATER_GUN), (37, &WITHDRAW), (44, &AMNESIA), (55, &PSYCHIC)]
);
pub const SLOWPOKE: Species = Species::new(
    SpeciesKind::Slowpoke, BaseStats::new(90, 65, 65, 15, 40), [Type::Water, Type::Psychic], 99,
    [Some(&CONFUSION), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(37), &SLOWBRO)],
    &[(18, &DISABLE), (22, &HEADBUTT), (27, &GROWL), (33, &WATER_GUN), (40, &AMNESIA), (48, &PSYCHIC)]
);
pub const SPEAROW: Species = Species::new(
    SpeciesKind::Spearow, BaseStats::new(40, 60, 30, 70, 31), [Type::Normal, Type::Flying], 58,
    [Some(&PECK), Some(&GROWL), None, None], GrowthRate::MediumFast, &[(Evolve::Level(20), &FEAROW)],
    &[(9, &LEER), (15, &FURY_ATTACK), (22, &MIRROR_MOVE), (29, &DRILL_PECK), (36, &AGILITY)]
);
pub const SQUIRTLE: Species = Species::new(
    SpeciesKind::Squirtle, BaseStats::new(44, 48, 65, 43, 50), [Type::Water, Type::Water], 66,
    [Some(&TACKLE), Some(&TAIL_WHIP), None, None], GrowthRate::MediumSlow, &[(Evolve::Level(16), &WARTORTLE)],
    &[(8, &BUBBLE), (15, &WATER_GUN), (22, &BITE), (28, &WITHDRAW), (35, &SKULL_BASH), (42, &HYDRO_PUMP)]
);
pub const STARMIE: Species = Species::new(
    SpeciesKind::Starmie, BaseStats::new(60, 75, 85, 115, 100), [Type::Water, Type::Psychic], 207,
    [Some(&TACKLE), Some(&WATER_GUN), Some(&HARDEN), None], GrowthRate::Slow, &[],
    &[]
);
pub const STARYU: Species = Species::new(
    SpeciesKind::Staryu, BaseStats::new(30, 45, 55, 85, 70), [Type::Water, Type::Water], 106,
    [Some(&TACKLE), None, None, None], GrowthRate::Slow, &[(Evolve::Item(Item::WaterStone), &STARMIE)],
    &[(17, &WATER_GUN), (22, &HARDEN), (27, &RECOVER), (32, &SWIFT), (37, &MINIMIZE), (42, &LIGHT_SCREEN), (52, &HYDRO_PUMP)]
);
pub const VENUSAUR: Species = Species::new(
    SpeciesKind::Venusaur, BaseStats::new(80, 82, 83, 80, 100), [Type::Grass, Type::Poison], 208,
    [Some(&TACKLE), Some(&GROWL), Some(&LEECH_SEED), Some(&VINE_WHIP)], GrowthRate::MediumSlow, &[],
    &[(7, &LEECH_SEED), (13, &VINE_WHIP), (22, &POISONPOWDER), (30, &RAZOR_LEAF), (43, &GROWTH), (55, &SLEEP_POWDER), (65, &SOLARBEAM)]
);
pub const VICTREEBEL: Species = Species::new(
    SpeciesKind::Victreebel, BaseStats::new(80, 105, 65, 70, 100), [Type::Grass, Type::Poison], 191,
    [Some(&SLEEP_POWDER), Some(&STUN_SPORE), Some(&ACID), Some(&RAZOR_LEAF)], GrowthRate::MediumSlow, &[],
    &[(13, &WRAP), (15, &POISONPOWDER), (18, &SLEEP_POWDER)]
);
pub const VILEPLUME: Species = Species::new(
    SpeciesKind::Vileplume, BaseStats::new(75, 80, 85, 50, 100), [Type::Grass, Type::Poison], 184,
    [Some(&STUN_SPORE), Some(&SLEEP_POWDER), Some(&ACID), Some(&PETAL_DANCE)], GrowthRate::MediumSlow, &[],
    &[(15, &POISONPOWDER), (17, &STUN_SPORE), (19, &SLEEP_POWDER)]
);
pub const VOLTORB: Species = Species::new(
    SpeciesKind::Voltorb, BaseStats::new(40, 30, 50, 100, 55), [Type::Electric, Type::Electric], 103,
    [Some(&TACKLE), Some(&SCREECH), None, None], GrowthRate::MediumFast, &[(Evolve::Level(30), &ELECTRODE)],
    &[(17, &SONICBOOM), (22, &SELFDESTRUCT), (29, &LIGHT_SCREEN), (36, &SWIFT), (43, &EXPLOSION)]
);
pub const WARTORTLE: Species = Species::new(
    SpeciesKind::Wartortle, BaseStats::new(59, 63, 80, 58, 65), [Type::Water, Type::Water], 143,
    [Some(&TACKLE), Some(&TAIL_WHIP), Some(&BUBBLE), None], GrowthRate::MediumSlow, &[(Evolve::Level(36), &BLASTOISE)],
    &[(8, &BUBBLE), (15, &WATER_GUN), (24, &BITE), (31, &WITHDRAW), (39, &SKULL_BASH), (47, &HYDRO_PUMP)]
);
pub const WEEDLE: Species = Species::new(
    SpeciesKind::Weedle, BaseStats::new(40, 35, 30, 50, 20), [Type::Bug, Type::Poison], 52,
    [Some(&POISON_STING), Some(&STRING_SHOT), None, None], GrowthRate::MediumFast, &[(Evolve::Level(7), &KAKUNA)],
    &[]
);
pub const WEEPINBELL: Species = Species::new(
    SpeciesKind::Weepinbell, BaseStats::new(65, 90, 50, 55, 85), [Type::Grass, Type::Poison], 151,
    [Some(&VINE_WHIP), Some(&GROWTH), Some(&WRAP), None], GrowthRate::MediumSlow, &[(Evolve::Item(Item::LeafStone), &VICTREEBEL)],
    &[(13, &WRAP), (15, &POISONPOWDER), (18, &SLEEP_POWDER), (23, &STUN_SPORE), (29, &ACID), (38, &RAZOR_LEAF), (49, &SLAM)]
);
pub const WEEZING: Species = Species::new(
    SpeciesKind::Weezing, BaseStats::new(65, 90, 120, 60, 85), [Type::Poison, Type::Poison], 173,
    [Some(&TACKLE), Some(&SMOG), Some(&SLUDGE), None], GrowthRate::MediumFast, &[],
    &[(32, &SLUDGE), (39, &SMOKESCREEN), (43, &SELFDESTRUCT), (49, &HAZE), (53, &EXPLOSION)]
);
pub const WIGGLYTUFF: Species = Species::new(
    SpeciesKind::Wigglytuff, BaseStats::new(140, 70, 45, 45, 50), [Type::Normal, Type::Normal], 109,
    [Some(&SING), Some(&DISABLE), Some(&DEFENSE_CURL), Some(&DOUBLESLAP)], GrowthRate::Fast, &[],
    &[]
);
pub const ZUBAT: Species = Species::new(
    SpeciesKind::Zubat, BaseStats::new(40, 45, 35, 55, 40), [Type::Poison, Type::Flying], 54,
    [Some(&LEECH_LIFE), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(22), &GOLBAT)],
    &[(10, &SUPERSONIC), (15, &BITE), (21, &CONFUSE_RAY), (28, &WING_ATTACK), (36, &HAZE)]
);
