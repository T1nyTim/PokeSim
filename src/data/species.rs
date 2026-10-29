#![cfg_attr(rustfmt, rustfmt::skip)]
use crate::data::{BaseStats, Evolve, GrowthRate, Item, Species, SpeciesKind, Type, moves::*};

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
pub const DIGLETT: Species = Species::new(
    SpeciesKind::Diglett, BaseStats::new(10, 55, 25, 95, 45), [Type::Ground, Type::Ground], 81,
    [Some(&SCRATCH), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(26), &DUGTRIO)],
    &[(15, &GROWL), (19, &DIG), (24, &SAND_ATTACK), (31, &SLASH), (40, &EARTHQUAKE)]
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
pub const GOLBAT: Species = Species::new(
    SpeciesKind::Golbat, BaseStats::new(75, 80, 70, 90, 75), [Type::Poison, Type::Flying], 171,
    [Some(&LEECH_LIFE), Some(&SCREECH), Some(&BITE), None], GrowthRate::MediumFast, &[],
    &[(10, &SUPERSONIC), (15, &BITE), (21, &CONFUSE_RAY), (32, &WING_ATTACK), (32, &HAZE)]
);
pub const GRAVELER: Species = Species::new(
    SpeciesKind::Graveler, BaseStats::new(55, 95, 115, 35, 45), [Type::Rock, Type::Ground], 134,
    [Some(&TACKLE), Some(&DEFENSE_CURL), None, None], GrowthRate::MediumSlow, &[],
    &[(11, &DEFENSE_CURL), (16, &ROCK_THROW), (21, &SELFDESTRUCT), (29, &HARDEN), (36, &EARTHQUAKE), (43, &EXPLOSION)]
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
pub const KAKUNA: Species = Species::new(
    SpeciesKind::Kakuna, BaseStats::new(45, 25, 50, 35, 25), [Type::Bug, Type::Poison], 71,
    [Some(&HARDEN), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(10), &BEEDRILL)],
    &[]
);
pub const METAPOD: Species = Species::new(
    SpeciesKind::Metapod, BaseStats::new(50, 20, 55, 30, 25), [Type::Bug, Type::Bug], 72,
    [Some(&HARDEN), None, None, None], GrowthRate::MediumFast, &[(Evolve::Level(10), &BUTTERFREE)],
    &[]
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
pub const ONIX: Species = Species::new(
    SpeciesKind::Onix, BaseStats::new(35, 45, 160, 70, 30), [Type::Rock, Type::Ground], 108,
    [Some(&TACKLE), Some(&SCREECH), None, None], GrowthRate::MediumFast, &[],
    &[(15, &BIND), (19, &ROCK_THROW), (25, &RAGE), (33, &SLAM), (43, &HARDEN)]
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
pub const SPEAROW: Species = Species::new(
    SpeciesKind::Spearow, BaseStats::new(40, 60, 30, 70, 31), [Type::Normal, Type::Flying], 58,
    [Some(&PECK), Some(&GROWL), None, None], GrowthRate::MediumFast, &[(Evolve::Level(20), &FEAROW)],
    &[(9, &LEER), (15, &FURY_ATTACK), (22, &MIRROR_MOVE), (29, &DRILL_PECK), (36, &AGILITY)]
);
pub const VENUSAUR: Species = Species::new(
    SpeciesKind::Venusaur, BaseStats::new(80, 82, 83, 80, 100), [Type::Grass, Type::Poison], 208,
    [Some(&TACKLE), Some(&GROWL), Some(&LEECH_SEED), Some(&VINE_WHIP)], GrowthRate::MediumSlow, &[],
    &[(7, &LEECH_SEED), (13, &VINE_WHIP), (22, &POISONPOWDER), (30, &RAZOR_LEAF), (43, &GROWTH), (55, &SLEEP_POWDER), (65, &SOLARBEAM)]
);
pub const WEEDLE: Species = Species::new(
    SpeciesKind::Weedle, BaseStats::new(40, 35, 30, 50, 20), [Type::Bug, Type::Poison], 52,
    [Some(&POISON_STING), Some(&STRING_SHOT), None, None], GrowthRate::MediumFast, &[(Evolve::Level(7), &KAKUNA)],
    &[]
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
