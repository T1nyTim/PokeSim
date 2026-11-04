mod battle;
mod data;
mod journey;
mod player;
mod pokemon;
mod win_rates;

use std::time::Instant;

use crate::{
    battle::Battle,
    data::{Item, species::BULBASAUR},
    journey::{
        BROCK, BUG_CATCHER, HIKER, JR_TRAINER_F, JR_TRAINER_M, JourneyStep, LASS, MISTY, MatchupGroup, PlayerPolicy, RIVAL1, ROCKET, SUPER_NERD,
        SWIMMER, YOUNGSTER,
    },
    player::Player,
    pokemon::{Dvs, Pokemon, data::*},
    win_rates::WinRates,
};

fn main() {
    let mut player = Player::new(Pokemon::from_species(&BULBASAUR, Dvs::AVERAGE, 5), PlayerPolicy::MostDmg);
    #[rustfmt::skip]
    let journey = [
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
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L7)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L11, &KAKUNA_TRAINER_L11], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L9)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CLEFAIRY_TRAINER_L14], LASS).with_encounter_depth(0, 34)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L8)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L9)),
        JourneyStep::Item(Item::HpUp),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L9)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&SANDSHREW_TRAINER_L11, &RATTATA_TRAINER_L11, &ZUBAT_TRAINER_L11], ROCKET).with_encounter_depth(0, 52)
        ),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L7)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&GEODUDE_WILD_L8)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MAGNEMITE_TRAINER_L11, &VOLTORB_TRAINER_L11], SUPER_NERD)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L6)),
        JourneyStep::Item(Item::RareCandy),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L10, &METAPOD_TRAINER_L10, &CATERPIE_TRAINER_L10], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L10)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L11, &BELLSPROUT_TRAINER_L11], LASS)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L8)),
        JourneyStep::Matchup(MatchupGroup::new_wild(&ZUBAT_WILD_L10)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ZUBAT_TRAINER_L12, &EKANS_TRAINER_L12], ROCKET).with_encounter_depth(1, 31)),
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
        JourneyStep::Item(Item::RareCandy),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&HORSEA_TRAINER_L16, &SHELLDER_TRAINER_L16], SWIMMER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&GOLDEEN_TRAINER_L19], JR_TRAINER_F)),
        JourneyStep::Matchup(MatchupGroup::new_boss(vec![&STARYU_TRAINER_L18, &STARMIE_MISTY_L21], MISTY).with_item(&Item::XDefend, 0.25, 1)),
        JourneyStep::Matchup(
            MatchupGroup::new_boss(vec![&PIDGEOTTO_TRAINER_L18, &ABRA_TRAINER_L15, &RATTATA_TRAINER_L15, &CHARMANDER_TRAINER_L17], RIVAL1)
                .with_encounter_depth(0, 42)
                .with_encounter_depth(3, 16)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L14, &WEEDLE_TRAINER_L14], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L14, &NIDORANF_TRAINER_L14], LASS).with_encounter_depth(0, 39)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L14, &EKANS_TRAINER_L14, &ZUBAT_TRAINER_L14], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L16, &NIDORANF_TRAINER_L16], LASS).with_encounter_depth(0, 39)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MANKEY_TRAINER_L18], JR_TRAINER_M)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&EKANS_TRAINER_L15, &ZUBAT_TRAINER_L15], ROCKET).with_encounter_depth(0, 22)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L14, &EKANS_TRAINER_L14], JR_TRAINER_M)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L15, &GEODUDE_TRAINER_L15], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L15, &SPEAROW_TRAINER_L15], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&ONIX_TRAINER_L17], HIKER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SLOWPOKE_TRAINER_L17], YOUNGSTER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&NIDORANM_TRAINER_L15, &NIDORANF_TRAINER_L15], LASS)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L13, &GEODUDE_TRAINER_L13, &MACHOP_TRAINER_L13, &GEODUDE_TRAINER_L13], HIKER)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L14, &EKANS_TRAINER_L14], JR_TRAINER_M).with_encounter_depth(1, 23)),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&EKANS_TRAINER_L14, &SANDSHREW_TRAINER_L14], YOUNGSTER)
                .with_encounter_depth(0, 23)
                .with_encounter_depth(1, 28)
        ),
        JourneyStep::Matchup(
            MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L13, &PIDGEY_TRAINER_L13, &ODDISH_TRAINER_L13], LASS).with_encounter_depth(1, 27)
        ),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&MACHOP_TRAINER_L17, &DROWZEE_TRAINER_L17], ROCKET).with_encounter_depth(1, 19)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L16, &CATERPIE_TRAINER_L16, &WEEDLE_TRAINER_L16], BUG_CATCHER)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SQUIRTLE_TRAINER_L20], JR_TRAINER_M)),
        JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L16, &PIKACHU_TRAINER_L16], JR_TRAINER_F)),
        // JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&BUTTERFREE_TRAINER_L20], BUG_CATCHER)),
        // JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L16, &PIDGEY_TRAINER_L16, &PIDGEY_TRAINER_L16], JR_TRAINER_F)),
        // JourneyStep::Matchup(MatchupGroup::new_trainer(vec![&SPEAROW_TRAINER_L16, &RATICATE_TRAINER_L16], JR_TRAINER_M)),
    ];
    let mut win_rates = WinRates::default();
    let mut count = 0u8;
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
            count += 1u8;
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
