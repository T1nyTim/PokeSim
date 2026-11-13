use crate::data::{Item, Move, Type};

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
    pub(crate) fn experience_at(&self, level: u8) -> u32 {
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
    Dodrio,
    Doduo,
    Drowzee,
    Dugtrio,
    Ekans,
    Electrode,
    Exeggcute,
    Exeggutor,
    Farfetchd,
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
            Self::Dodrio => "Dodrio",
            Self::Doduo => "Doduo",
            Self::Drowzee => "Drowzee",
            Self::Dugtrio => "Dugtrio",
            Self::Ekans => "Ekans",
            Self::Electrode => "Electrode",
            Self::Exeggcute => "Exeggcute",
            Self::Exeggutor => "Exeggutor",
            Self::Farfetchd => "Farfetch'd",
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
    pub(crate) const fn name(&self) -> &'static str {
        self.kind.as_str()
    }

    pub(crate) fn can_learn_tm(&self, mv: &Move) -> bool {
        self.tm_moves.contains(&mv)
    }

    pub(crate) fn evolution_at(&self, level: u8) -> Option<&'static Self> {
        self.evolution
            .iter()
            .find_map(|(condition, species)| condition.is_met(level).then_some(*species))
    }

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
}
