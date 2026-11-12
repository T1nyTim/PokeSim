mod battle;
mod data;
mod journey;
mod player;
mod pokemon;
mod win_rates;

use std::time::Instant;

use crate::{
    battle::Battle,
    data::{
        Item,
        moves::{
            BIDE, BODY_SLAM, BUBBLEBEAM, DIG, DOUBLE_EDGE, DOUBLE_TEAM, DRAGON_RAGE, DREAM_EATER, EGG_BOMB, HORN_DRILL, HYPER_BEAM, ICE_BEAM,
            MEGA_DRAIN, MEGA_KICK, MEGA_PUNCH, PSYCHIC, RAZOR_WIND, REFLECT, REST, ROCK_SLIDE, SEISMIC_TOSS, SUBMISSION, SUBSTITUTE, SWIFT,
            TAKE_DOWN, TELEPORT, THUNDER_WAVE, THUNDERBOLT, TRI_ATTACK, WATER_GUN, WHIRLWIND,
        },
        species::BULBASAUR,
    },
    journey::{
        BEAUTY, BIKER, BIRD_KEEPER, BROCK, BUG_CATCHER, CHANNELER, COOLTRAINER_F, ENGINEER, ERIKA, FISHER, GAMBLER, GENTLEMAN, GIOVANNI, HIKER,
        JR_TRAINER_F, JR_TRAINER_M, JourneyStep, LASS, LT_SURGE, MISTY, MatchupGroup, POKEMANIAC, PlayerPolicy, RIVAL1, RIVAL2, ROCKER, ROCKET,
        SAILOR, SUPER_NERD, SWIMMER, YOUNGSTER,
    },
    player::Player,
    pokemon::{Dvs, Pokemon, data::*},
    win_rates::WinRates,
};

fn main() {
    let mut player = Player::new(Pokemon::from_species(&BULBASAUR, Dvs::AVERAGE, 5), PlayerPolicy::MostDmg);
    #[rustfmt::skip]
    let journey = vec![
        JourneyStep::Matchup(MatchupGroup::new_boss(vec![&CHARMANDER_TRAINER_L5], RIVAL1).with_encounter_depth(0, 28)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&RATTATA_WILD_L3)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&PIDGEY_WILD_L3)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&RATTATA_WILD_L2)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&RATTATA_WILD_L3)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&RATTATA_WILD_L3)),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(vec![&PIDGEY_TRAINER_L9, &CHARMANDER_TRAINER_L8], RIVAL1).with_encounter_depth(0, 53).with_encounter_depth(1, 33)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&PIDGEY_WILD_L3)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&PIDGEY_WILD_L2)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&WEEDLE_WILD_L4)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANF_TRAINER_L6, &NIDORANM_TRAINER_L6], LASS).with_encounter_depth(0, 24)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L6, &CATERPIE_TRAINER_L6], BUG_CATCHER)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L7, &KAKUNA_TRAINER_L7, &WEEDLE_TRAINER_L7], BUG_CATCHER).with_encounter_depth(1, 25)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L8, &METAPOD_TRAINER_L8], BUG_CATCHER).with_encounter_depth(1, 27)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&KAKUNA_WILD_L5)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L9], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&WEEDLE_WILD_L3)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&DIGLETT_TRAINER_L11, &SANDSHREW_TRAINER_L11], JR_TRAINER_M).with_encounter_depth(1, 47)),
        JourneyStep::Matchup(MatchupGroup::new_boss(vec![&GEODUDE_TRAINER_L12, &ONIX_BROCK_L14], BROCK)),
        JourneyStep::Item(Item::Tm(&BIDE)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L9, &PIDGEY_TRAINER_L9], LASS).with_encounter_depth(0, 55).with_encounter_depth(1, 54)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L10, &WEEDLE_TRAINER_L10, &CATERPIE_TRAINER_L10], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L11, &EKANS_TRAINER_L11], YOUNGSTER).with_encounter_depth(1, 28)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L9, &KAKUNA_TRAINER_L9, &CATERPIE_TRAINER_L9, &METAPOD_TRAINER_L9], BUG_CATCHER)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SPEAROW_TRAINER_L14], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L10, &NIDORANM_TRAINER_L10], LASS)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L11, &METAPOD_TRAINER_L11], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&JIGGLYPUFF_TRAINER_L14], LASS).with_encounter_depth(0, 25)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L8)),
        JourneyStep::Item(Item::Tm(&WATER_GUN)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L7)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L11, &KAKUNA_TRAINER_L11], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L9)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CLEFAIRY_TRAINER_L14], LASS).with_encounter_depth(0, 34)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L8)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L9)),
        JourneyStep::Item(Item::HpUp),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L9)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&SANDSHREW_TRAINER_L11, &RATTATA_TRAINER_L11, &ZUBAT_TRAINER_L11], ROCKET).with_encounter_depth(0, 64)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L7)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L8)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MAGNEMITE_TRAINER_L11, &VOLTORB_TRAINER_L11], SUPER_NERD)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L6)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L10, &METAPOD_TRAINER_L10, &CATERPIE_TRAINER_L10], BUG_CATCHER)),
        JourneyStep::Item(Item::RareCandy),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L10)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L11, &BELLSPROUT_TRAINER_L11], LASS)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L8)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L10)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ZUBAT_TRAINER_L12, &EKANS_TRAINER_L12], ROCKET).with_encounter_depth(1, 31)),
        JourneyStep::Item(Item::Tm(&MEGA_PUNCH)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L7)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L10, &RATTATA_TRAINER_L10, &ZUBAT_TRAINER_L10], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L7)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L10, &GEODUDE_TRAINER_L10, &ONIX_TRAINER_L10], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L10)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L8)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATICATE_TRAINER_L16], ROCKET)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L10)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L11)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&PARAS_WILD_L10)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L13, &ZUBAT_TRAINER_L13], ROCKET)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L9)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GRIMER_TRAINER_L12, &VOLTORB_TRAINER_L12, &KOFFING_TRAINER_L12], SUPER_NERD)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L9)),
        JourneyStep::Item(Item::Tm(&WHIRLWIND)),
        JourneyStep::Item(Item::RareCandy),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&HORSEA_TRAINER_L16, &SHELLDER_TRAINER_L16], SWIMMER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GOLDEEN_TRAINER_L19], JR_TRAINER_F)),
        JourneyStep::Matchup(MatchupGroup::new_boss(vec![&STARYU_TRAINER_L18, &STARMIE_MISTY_L21], MISTY).with_item(&Item::XDefend, 0.25, 1)),
        JourneyStep::Item(Item::Tm(&BUBBLEBEAM)),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(vec![&PIDGEOTTO_TRAINER_L18, &ABRA_TRAINER_L15, &RATTATA_TRAINER_L15, &CHARMANDER_TRAINER_L17], RIVAL1)
            .with_encounter_depth(0, 42)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L14, &WEEDLE_TRAINER_L14], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L14, &NIDORANF_TRAINER_L14], LASS).with_encounter_depth(0, 46)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L14, &EKANS_TRAINER_L14, &ZUBAT_TRAINER_L14], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L16, &NIDORANF_TRAINER_L16], LASS).with_encounter_depth(0, 48)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MANKEY_TRAINER_L18], JR_TRAINER_M)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&EKANS_TRAINER_L15, &ZUBAT_TRAINER_L15], ROCKET).with_encounter_depth(0, 27)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L14, &EKANS_TRAINER_L14], JR_TRAINER_M)),
        JourneyStep::Item(Item::Tm(&THUNDER_WAVE)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L15, &GEODUDE_TRAINER_L15], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L15, &SPEAROW_TRAINER_L15], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ONIX_TRAINER_L17], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SLOWPOKE_TRAINER_L17], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANM_TRAINER_L15, &NIDORANF_TRAINER_L15], LASS)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L13, &GEODUDE_TRAINER_L13, &MACHOP_TRAINER_L13, &GEODUDE_TRAINER_L13], HIKER)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L14, &EKANS_TRAINER_L14], JR_TRAINER_M).with_encounter_depth(1, 25)),
        JourneyStep::Item(Item::Tm(&SEISMIC_TOSS)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&EKANS_TRAINER_L14, &SANDSHREW_TRAINER_L14], YOUNGSTER)
            .with_encounter_depth(0, 25)
            .with_encounter_depth(1, 73)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L13, &PIDGEY_TRAINER_L13, &ODDISH_TRAINER_L13], LASS).with_encounter_depth(1, 47)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L17, &DROWZEE_TRAINER_L17], ROCKET)),
        JourneyStep::Item(Item::Tm(&DIG)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L16, &CATERPIE_TRAINER_L16, &WEEDLE_TRAINER_L16], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SQUIRTLE_TRAINER_L20], JR_TRAINER_M)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L16, &PIKACHU_TRAINER_L16], JR_TRAINER_F)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&BUTTERFREE_TRAINER_L20], BUG_CATCHER)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L16, &PIDGEY_TRAINER_L16, &PIDGEY_TRAINER_L16], JR_TRAINER_F)
            .with_encounter_depth(0, 47)
            .with_encounter_depth(1, 47)
            .with_encounter_depth(2, 47)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SPEAROW_TRAINER_L16, &RATICATE_TRAINER_L16], JR_TRAINER_M)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L18, &NIDORANF_TRAINER_L18], LASS).with_encounter_depth(0, 51)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANM_TRAINER_L21], YOUNGSTER)),
        JourneyStep::Item(Item::Tm(&BODY_SLAM)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANM_TRAINER_L19, &NIDORANF_TRAINER_L19], GENTLEMAN)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GROWLITHE_TRAINER_L18, &GROWLITHE_TRAINER_L18], GENTLEMAN)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SHELLDER_TRAINER_L21], SAILOR)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&HORSEA_TRAINER_L17, &SHELLDER_TRAINER_L17, &TENTACOOL_TRAINER_L17], SAILOR)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&HORSEA_TRAINER_L17, &HORSEA_TRAINER_L17, &HORSEA_TRAINER_L17], SAILOR)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&TENTACOOL_TRAINER_L18, &STARYU_TRAINER_L18], SAILOR)),
        JourneyStep::Item(Item::Tm(&REST)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&TENTACOOL_TRAINER_L17, &STARYU_TRAINER_L17, &SHELLDER_TRAINER_L17], FISHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L20], SAILOR)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L17, &TENTACOOL_TRAINER_L17], SAILOR)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L18, &SHELLDER_TRAINER_L18], SAILOR)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GOLDEEN_TRAINER_L17, &TENTACOOL_TRAINER_L17, &GOLDEEN_TRAINER_L17], FISHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIKACHU_TRAINER_L23], GENTLEMAN)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GROWLITHE_TRAINER_L17, &PONYTA_TRAINER_L17], GENTLEMAN)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L18, &PIKACHU_TRAINER_L18], LASS)),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(vec![&PIDGEOTTO_TRAINER_L19, &RATICATE_TRAINER_L16, &KADABRA_TRAINER_L18, &CHARMELEON_TRAINER_L20], RIVAL2)
            .with_item_hp_threshold(&Item::Potion, 0.125, 1, 5)
            .with_encounter_depth(0, 49)
        ),
        JourneyStep::Item(Item::RareCandy),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L18)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L19)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L17)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L20)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L16)),
        JourneyStep::Item(Item::HpUp),
        JourneyStep::Item(Item::Tm(&DREAM_EATER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L15)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L18)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L19)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&DIGLETT_WILD_L17)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&EKANS_TRAINER_L21], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SANDSHREW_TRAINER_L19, &ZUBAT_TRAINER_L19], YOUNGSTER).with_encounter_depth(0, 49)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANM_TRAINER_L18, &NIDORINO_TRAINER_L18], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&BELLSPROUT_TRAINER_L18, &ODDISH_TRAINER_L18], GAMBLER).with_encounter_depth(0, 29)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GROWLITHE_TRAINER_L18, &VULPIX_TRAINER_L18], GAMBLER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MAGNEMITE_TRAINER_L21], ENGINEER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MAGNEMITE_TRAINER_L18, &MAGNEMITE_TRAINER_L18, &MAGNEMITE_TRAINER_L18], ENGINEER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&VOLTORB_TRAINER_L18, &MAGNEMITE_TRAINER_L18], GAMBLER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L17, &RATTATA_TRAINER_L17, &RATICATE_TRAINER_L17], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&POLIWAG_TRAINER_L18, &HORSEA_TRAINER_L18], GAMBLER).with_encounter_depth(0, 75)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIKACHU_TRAINER_L21, &PIKACHU_TRAINER_L21], SAILOR)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&VOLTORB_TRAINER_L20, &MAGNEMITE_TRAINER_L20, &VOLTORB_TRAINER_L20], ROCKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIKACHU_TRAINER_L23], GENTLEMAN)),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(vec![&VOLTORB_TRAINER_L21, &PIKACHU_TRAINER_L18, &RAICHU_SURGE_L24], LT_SURGE).with_item(&Item::XSpeed, 0.25, 1)
        ),
        JourneyStep::Item(Item::Tm(&THUNDERBOLT)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L18, &BELLSPROUT_TRAINER_L18, &ODDISH_TRAINER_L18, &BELLSPROUT_TRAINER_L18], JR_TRAINER_F)
            .with_encounter_depth(1, 29)
            .with_encounter_depth(3, 29)
        ),
        JourneyStep::Item(Item::Tm(&TELEPORT)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L20, &ONIX_TRAINER_L20], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GROWLITHE_TRAINER_L21, &CHARMANDER_TRAINER_L21], JR_TRAINER_M)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&BEEDRILL_TRAINER_L19, &BEEDRILL_TRAINER_L19], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L21, &ONIX_TRAINER_L21], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L20, &WEEDLE_TRAINER_L20, &VENONAT_TRAINER_L20], BUG_CATCHER)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L19, &DIGLETT_TRAINER_L19, &EKANS_TRAINER_L19, &SANDSHREW_TRAINER_L19], JR_TRAINER_M)
            .with_encounter_depth(3, 50)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L20, &MACHOP_TRAINER_L20, &GEODUDE_TRAINER_L20], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MEOWTH_TRAINER_L23], JR_TRAINER_F)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIKACHU_TRAINER_L20, &CLEFAIRY_TRAINER_L20], JR_TRAINER_F).with_encounter_depth(1, 29)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L16)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CUBONE_TRAINER_L23, &SLOWPOKE_TRAINER_L23], POKEMANIAC)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L17)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L17)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L16)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SLOWPOKE_TRAINER_L25], POKEMANIAC)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L22, &BULBASAUR_TRAINER_L22], JR_TRAINER_F).with_encounter_depth(0, 25)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L17)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CHARMANDER_TRAINER_L22, &CUBONE_TRAINER_L22], POKEMANIAC)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L17)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&MACHOP_WILD_L15)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L25], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L20, &ONIX_TRAINER_L20], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L16)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L19, &MACHOP_TRAINER_L20, &GEODUDE_TRAINER_L19, &GEODUDE_TRAINER_L19], HIKER)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&MACHOP_WILD_L15)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ONIX_TRAINER_L20, &ONIX_TRAINER_L20, &GEODUDE_TRAINER_L20], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L16)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L21, &GEODUDE_TRAINER_L21], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L18)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L18)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&JIGGLYPUFF_TRAINER_L21, &PIDGEY_TRAINER_L21, &MEOWTH_TRAINER_L21], JR_TRAINER_F)
            .with_encounter_depth(0, 25)
            .with_encounter_depth(1, 61)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L21, &GEODUDE_TRAINER_L21, &GRAVELER_TRAINER_L21], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L16)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SLOWPOKE_TRAINER_L20, &SLOWPOKE_TRAINER_L20, &SLOWPOKE_TRAINER_L20], POKEMANIAC)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L16)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&BELLSPROUT_TRAINER_L22, &CLEFAIRY_TRAINER_L22], JR_TRAINER_F)
            .with_encounter_depth(0, 75)
            .with_encounter_depth(1, 31)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L19, &RATTATA_TRAINER_L19, &RATTATA_TRAINER_L19, &BELLSPROUT_TRAINER_L19], JR_TRAINER_F)
            .with_encounter_depth(0, 60)
            .with_encounter_depth(3, 29)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L17)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&MEOWTH_TRAINER_L20, &ODDISH_TRAINER_L20, &PIDGEY_TRAINER_L20], JR_TRAINER_F)
            .with_encounter_depth(1, 25)
            .with_encounter_depth(2, 61)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L15)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L21, &PIDGEOTTO_TRAINER_L21], JR_TRAINER_F)
            .with_encounter_depth(0, 62)
            .with_encounter_depth(1, 61)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L21, &ONIX_TRAINER_L21], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ONIX_TRAINER_L19, &GRAVELER_TRAINER_L19], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CUBONE_TRAINER_L20, &SLOWPOKE_TRAINER_L20], POKEMANIAC)),
        JourneyStep::Item(Item::Tm(&SWIFT)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&CLEFAIRY_TRAINER_L22, &CLEFAIRY_TRAINER_L22], LASS).with_encounter_depth(0, 31).with_encounter_depth(1, 31)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GROWLITHE_TRAINER_L24, &VULPIX_TRAINER_L24], GAMBLER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GRIMER_TRAINER_L22, &MUK_TRAINER_L22, &GRIMER_TRAINER_L22], SUPER_NERD)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANF_TRAINER_L23, &NIDORINA_TRAINER_L23], LASS)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&KOFFING_TRAINER_L26], SUPER_NERD)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MEOWTH_TRAINER_L24, &MEOWTH_TRAINER_L24, &MEOWTH_TRAINER_L24], LASS)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(
                vec![&PIDGEY_TRAINER_L19, &RATTATA_TRAINER_L19, &NIDORANM_TRAINER_L19, &MEOWTH_TRAINER_L19, &PIKACHU_TRAINER_L19],
                LASS,
            )
            .with_encounter_depth(0, 58)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&POLIWAG_TRAINER_L22, &POLIWAG_TRAINER_L22, &POLIWHIRL_TRAINER_L22], GAMBLER)
            .with_encounter_depth(0, 53)
            .with_encounter_depth(1, 53)
            .with_encounter_depth(2, 53)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&VOLTORB_TRAINER_L20, &KOFFING_TRAINER_L20, &VOLTORB_TRAINER_L20, &MAGNEMITE_TRAINER_L20], SUPER_NERD)
        ),
        JourneyStep::Item(Item::PpUp),
        JourneyStep::Item(Item::Tm(&DOUBLE_TEAM)),
        JourneyStep::Item(Item::Tm(&REFLECT)),
        JourneyStep::Item(Item::Tm(&RAZOR_WIND)),
        JourneyStep::Item(Item::Tm(&HORN_DRILL)),
        JourneyStep::Item(Item::Tm(&EGG_BOMB)),
        JourneyStep::Item(Item::Tm(&MEGA_PUNCH)),
        JourneyStep::Item(Item::Tm(&MEGA_KICK)),
        JourneyStep::Item(Item::Tm(&TAKE_DOWN)),
        JourneyStep::Item(Item::Tm(&SUBMISSION)),
        // JourneyStep::Item(Item::Tm(&COUNTER)), Needs some thought
        JourneyStep::Item(Item::HpUp),
        JourneyStep::Item(Item::Protein),
        JourneyStep::Item(Item::Iron),
        JourneyStep::Item(Item::Carbos),
        JourneyStep::Item(Item::Calcium),
        JourneyStep::Item(Item::Tm(&ICE_BEAM)),
        JourneyStep::Item(Item::Tm(&ROCK_SLIDE)),
        JourneyStep::Item(Item::Tm(&TRI_ATTACK)),
        JourneyStep::Item(Item::Tm(&PSYCHIC)),
        JourneyStep::Item(Item::Tm(&DRAGON_RAGE)),
        JourneyStep::Item(Item::Tm(&HYPER_BEAM)),
        JourneyStep::Item(Item::Tm(&SUBSTITUTE)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATICATE_TRAINER_L20, &ZUBAT_TRAINER_L20], ROCKET)),
        JourneyStep::Item(Item::PpUp),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATICATE_TRAINER_L21, &RATICATE_TRAINER_L21], ROCKET)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&DROWZEE_TRAINER_L21, &MACHOP_TRAINER_L21], ROCKET).with_encounter_depth(0, 31)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(
                vec![&ZUBAT_TRAINER_L17, &KOFFING_TRAINER_L17, &GRIMER_TRAINER_L17, &ZUBAT_TRAINER_L17, &RATICATE_TRAINER_L17],
                ROCKET,
            )
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L21, &MACHOP_TRAINER_L21], ROCKET)),
        JourneyStep::Item(Item::Tm(&DOUBLE_EDGE)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L20, &RATICATE_TRAINER_L20, &DROWZEE_TRAINER_L20], ROCKET).with_encounter_depth(2, 25)
        ),
        JourneyStep::Item(Item::HpUp),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&KOFFING_TRAINER_L21, &ZUBAT_TRAINER_L21], ROCKET)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L19, &RATICATE_TRAINER_L19, &RATICATE_TRAINER_L19, &RATTATA_TRAINER_L19], ROCKET)
        ),
        JourneyStep::Item(Item::RareCandy),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GRIMER_TRAINER_L20, &KOFFING_TRAINER_L20, &KOFFING_TRAINER_L20], ROCKET)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&SANDSHREW_TRAINER_L23, &EKANS_TRAINER_L23, &SANDSLASH_TRAINER_L23], ROCKET)
            .with_encounter_depth(0, 51)
            .with_encounter_depth(2, 47)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&EKANS_TRAINER_L23, &SANDSHREW_TRAINER_L23, &ARBOK_TRAINER_L23], ROCKET).with_encounter_depth(1, 51)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(vec![&ONIX_TRAINER_L25, &RHYHORN_TRAINER_L24, &KANGASKHAN_TRAINER_L29], GIOVANNI)
            .with_item(&Item::GuardSpec, 0.25, 1)
            .with_encounter_depth(0, 23)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GRIMER_TRAINER_L22, &KOFFING_TRAINER_L22], ROCKET)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&BELLSPROUT_TRAINER_L23, &WEEPINBELL_TRAINER_L23], LASS)
            .with_encounter_depth(0, 47)
            .with_encounter_depth(1, 58)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L21, &BELLSPROUT_TRAINER_L21, &ODDISH_TRAINER_L21, &BELLSPROUT_TRAINER_L21], BEAUTY)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&BULBASAUR_TRAINER_L24, &IVYSAUR_TRAINER_L24], JR_TRAINER_F)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&BELLSPROUT_TRAINER_L24, &BELLSPROUT_TRAINER_L24], BEAUTY)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&EXEGGCUTE_TRAINER_L26], BEAUTY).with_encounter_depth(0, 37)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&WEEPINBELL_TRAINER_L24, &GLOOM_TRAINER_L24, &IVYSAUR_TRAINER_L24], COOLTRAINER_F)
            .with_item_hp_threshold(&Item::HyperPotion, 1.0, 1, 10)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L23, &GLOOM_TRAINER_L23], LASS).with_encounter_depth(0, 25).with_encounter_depth(1, 25)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(vec![&VICTREEBEL_TRAINER_L29, &TANGELA_TRAINER_L24, &VILEPLUME_ERIKA_L29], ERIKA)
            .with_item_hp_threshold(&Item::SuperPotion, 0.5, 1, 10)
        ),
        JourneyStep::Item(Item::Tm(&MEGA_DRAIN)),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(
                vec![&PIDGEOTTO_TRAINER_L25, &EXEGGCUTE_TRAINER_L23, &GYARADOS_TRAINER_L22, &KADABRA_TRAINER_L20, &CHARMELEON_TRAINER_L25],
                RIVAL2,
            )
            .with_item_hp_threshold(&Item::Potion, 0.125, 1, 5)
            .with_encounter_depth(0, 49)
            .with_encounter_depth(1, 28)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L20).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L23], CHANNELER).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L21).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L22], CHANNELER).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L24], CHANNELER).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L22).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L23, &GASTLY_TRAINER_L23], CHANNELER).with_encounter_depth(0, 22).with_encounter_depth(1, 22)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L22], CHANNELER).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L20).with_encounter_depth(0, 22)),
        JourneyStep::Item(Item::HpUp),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L21).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L24], CHANNELER).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L20).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L22], CHANNELER).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L21).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&HAUNTER_TRAINER_L23], CHANNELER).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L24], CHANNELER).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L22).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&HAUNTER_TRAINER_L22], CHANNELER).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L23).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L22, &GASTLY_TRAINER_L22, &GASTLY_TRAINER_L22], CHANNELER)
            .with_encounter_depth(0, 22)
            .with_encounter_depth(1, 22)
            .with_encounter_depth(2, 22)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L21).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L24], CHANNELER).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L22).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GASTLY_TRAINER_L24], CHANNELER).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L23).with_encounter_depth(0, 21)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&MAROWAK_SPECIAL_L30)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ZUBAT_TRAINER_L25, &ZUBAT_TRAINER_L25, &GOLBAT_TRAINER_L25], ROCKET)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GASTLY_WILD_L21).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&KOFFING_TRAINER_L26, &DROWZEE_TRAINER_L26], ROCKET).with_encounter_depth(1, 24)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&ZUBAT_TRAINER_L23, &RATTATA_TRAINER_L23, &RATICATE_TRAINER_L23, &ZUBAT_TRAINER_L23], ROCKET)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&GOLDEEN_TRAINER_L22, &POLIWAG_TRAINER_L22, &GOLDEEN_TRAINER_L22], FISHER).with_encounter_depth(1, 40)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&TENTACOOL_TRAINER_L24, &GOLDEEN_TRAINER_L24], FISHER)),
        JourneyStep::Item(Item::RareCandy),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GOLDEEN_TRAINER_L27], FISHER)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&POLIWAG_TRAINER_L21, &SHELLDER_TRAINER_L21, &GOLDEEN_TRAINER_L21, &HORSEA_TRAINER_L21], FISHER)
            .with_encounter_depth(0, 37)
            .with_encounter_depth(3, 37)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&SNORLAX_SPECIAL_L30)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&VOLTORB_TRAINER_L29, &ELECTRODE_TRAINER_L29], ROCKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MAGIKARP_TRAINER_L24, &MAGIKARP_TRAINER_L24], FISHER)),
        JourneyStep::Item(Item::Iron),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANM_TRAINER_L29, &NIDORINO_TRAINER_L29], JR_TRAINER_M)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&GOLDEEN_TRAINER_L28, &POLIWAG_TRAINER_L28, &HORSEA_TRAINER_L28], JR_TRAINER_F)
            .with_encounter_depth(1, 33)
            .with_encounter_depth(2, 33)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L29, &PIDGEOTTO_TRAINER_L29], BIRD_KEEPER)
            .with_encounter_depth(0, 32)
            .with_encounter_depth(1, 32)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(
                vec![&PIDGEY_TRAINER_L24, &MEOWTH_TRAINER_L24, &RATTATA_TRAINER_L24, &PIKACHU_TRAINER_L24, &MEOWTH_TRAINER_L24],
                JR_TRAINER_F
            )
            .with_encounter_depth(0, 31)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L27, &PIKACHU_TRAINER_L27, &RATTATA_TRAINER_L27], BEAUTY)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CLEFAIRY_TRAINER_L29, &MEOWTH_TRAINER_L29], BEAUTY).with_encounter_depth(0, 29)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&POLIWAG_TRAINER_L30, &POLIWAG_TRAINER_L30], JR_TRAINER_F)
            .with_encounter_depth(0, 28)
            .with_encounter_depth(1, 28)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L27, &MEOWTH_TRAINER_L27, &PIDGEY_TRAINER_L27, &PIDGEOTTO_TRAINER_L27], JR_TRAINER_F)
            .with_encounter_depth(0, 27)
            .with_encounter_depth(2, 27)
            .with_encounter_depth(3, 27)
        ),
        JourneyStep::Item(Item::Calcium),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L26, &PIDGEOTTO_TRAINER_L26, &SPEAROW_TRAINER_L26, &FEAROW_TRAINER_L26], BIRD_KEEPER)
            .with_encounter_depth(0, 27)
            .with_encounter_depth(1, 27)
            .with_encounter_depth(3, 17)
        ),
        JourneyStep::Item(Item::PpUp),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(
                vec![&SPEAROW_TRAINER_L25, &PIDGEY_TRAINER_L25, &PIDGEY_TRAINER_L25, &SPEAROW_TRAINER_L25, &SPEAROW_TRAINER_L25],
                BIRD_KEEPER
            )
            .with_encounter_depth(0, 19)
            .with_encounter_depth(1, 26)
            .with_encounter_depth(1, 26)
            .with_encounter_depth(0, 15)
            .with_encounter_depth(0, 15)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&KOFFING_TRAINER_L28, &KOFFING_TRAINER_L28, &KOFFING_TRAINER_L28], BIKER)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L26, &SPEAROW_TRAINER_L26, &PIDGEY_TRAINER_L26, &FEAROW_TRAINER_L26], BIRD_KEEPER)
            .with_encounter_depth(0, 24)
            .with_encounter_depth(2, 24)
            .with_encounter_depth(3, 18)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L28, &DODUO_TRAINER_L28, &PIDGEOTTO_TRAINER_L28], BIRD_KEEPER)
            .with_encounter_depth(0, 24)
            .with_encounter_depth(2, 24)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&PIDGEOTTO_TRAINER_L29, &FEAROW_TRAINER_L29], BIRD_KEEPER)
            .with_encounter_depth(0, 23)
            .with_encounter_depth(1, 17)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&SPEAROW_TRAINER_L28, &DODUO_TRAINER_L28, &FEAROW_TRAINER_L28], BIRD_KEEPER)
            .with_encounter_depth(0, 19)
            .with_encounter_depth(2, 18)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&KOFFING_TRAINER_L29, &MUK_TRAINER_L29], BIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&FARFETCHD_TRAINER_L33], BIRD_KEEPER)),
        // JourneyStep::Matchup(
        //     MatchupGroup::new_trainer(vec![&KOFFING_TRAINER_L26, &KOFFING_TRAINER_L26, &GRIMER_TRAINER_L26, &KOFFING_TRAINER_L26], BIKER)
        // ),
        // JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&KOFFING_TRAINER_L29, &GRIMER_TRAINER_L29], BIKER)),
        // JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GRIMER_TRAINER_L28, &GRIMER_TRAINER_L28, &KOFFING_TRAINER_L28], BIKER)),
        // JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SPEAROW_TRAINER_L29, &FEAROW_TRAINER_L29], BIRD_KEEPER)),
    ];
    let mut win_rates = WinRates::default();
    let mut count = 0u16;
    for step in &journey {
        let matchup = match step {
            JourneyStep::Matchup(matchup) => matchup,
            JourneyStep::Item(item) => {
                player.use_item(item);
                continue;
            }
        };
        for encounter in &matchup.party {
            let ai = encounter.pokemon;
            count += 1u16;
            let mut battle = Battle::new(&player, ai, matchup);
            let started = Instant::now();
            let outcome = battle.run(encounter.max_depth);
            let elapsed = started.elapsed();
            let unresolved = if outcome.unresolved * 100.0f64 > 1e-10f64 {
                format!(" | U: {:.10}%", outcome.unresolved * 100.0f64)
            } else {
                String::new()
            };
            println!(
                "Encounter {count}: {} v L{} {} -> W: {:.2}% | L: {:.2}%{unresolved} | Time: {elapsed:.1?}",
                player.active.name(),
                ai.level,
                ai.name(),
                outcome.win * 100.0f64,
                outcome.lose * 100.0f64,
            );
            win_rates.record(&matchup.kind, &outcome);
            player.gain_experience(ai, &matchup.kind);
        }
        player.finish_matchup();
    }
    println!("\n{win_rates}");
}
