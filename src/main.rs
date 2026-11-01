mod battle;
mod data;
mod matchup;
mod player;
mod pokemon;
mod win_rates;

use std::time::Instant;

use crate::{
    battle::Battle,
    data::species::BULBASAUR,
    matchup::{BROCK, BUG_CATCHER, JR_TRAINER_M, LASS, MatchupGroup, PlayerPolicy, RIVAL1, YOUNGSTER},
    player::Player,
    pokemon::{Dvs, Pokemon, data::*},
    win_rates::WinRates,
};

fn main() {
    let mut player = Player::new(Pokemon::from_species(&BULBASAUR, Dvs::AVERAGE, 5));
    #[rustfmt::skip]
    let matchups = [
        MatchupGroup::new_boss(vec![&CHARMANDER_TRAINER_L5], RIVAL1).with_encounter_depth(0, 26),
        MatchupGroup::new_wild(&RATTATA_WILD_L3),
        MatchupGroup::new_wild(&PIDGEY_WILD_L3),
        MatchupGroup::new_wild(&RATTATA_WILD_L2),
        MatchupGroup::new_wild(&RATTATA_WILD_L3),
        MatchupGroup::new_wild(&RATTATA_WILD_L3),
        MatchupGroup::new_boss(vec![&PIDGEY_TRAINER_L9, &CHARMANDER_TRAINER_L8], RIVAL1).with_encounter_depth(0, 49).with_encounter_depth(1, 33),
        MatchupGroup::new_wild(&PIDGEY_WILD_L3),
        MatchupGroup::new_wild(&PIDGEY_WILD_L2),
        MatchupGroup::new_wild(&WEEDLE_WILD_L4),
        MatchupGroup::new_trainer(vec![&NIDORANF_TRAINER_L6, &NIDORANM_TRAINER_L6], LASS).with_encounter_depth(0, 24),
        MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L6, &CATERPIE_TRAINER_L6], BUG_CATCHER),
        MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L7, &KAKUNA_TRAINER_L7, &WEEDLE_TRAINER_L7], BUG_CATCHER).with_encounter_depth(1, 25),
        MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L8, &METAPOD_TRAINER_L8], BUG_CATCHER).with_encounter_depth(1, 26),
        MatchupGroup::new_wild(&KAKUNA_WILD_L5),
        MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L9], BUG_CATCHER),
        MatchupGroup::new_wild(&WEEDLE_WILD_L3),
        MatchupGroup::new_trainer(vec![&DIGLETT_TRAINER_L11, &SANDSHREW_TRAINER_L11], JR_TRAINER_M).with_encounter_depth(1, 47),
        MatchupGroup::new_boss(vec![&GEODUDE_TRAINER_L12, &ONIX_BROCK_L14], BROCK),
        MatchupGroup::new_trainer(vec![&PIDGEY_TRAINER_L9, &PIDGEY_TRAINER_L9], LASS).with_encounter_depth(0, 49).with_encounter_depth(1, 49),
        MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L10, &WEEDLE_TRAINER_L10, &CATERPIE_TRAINER_L10], BUG_CATCHER),
        MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L11, &EKANS_TRAINER_L11], YOUNGSTER).with_encounter_depth(1, 26),
        MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L9, &KAKUNA_TRAINER_L9, &CATERPIE_TRAINER_L9, &METAPOD_TRAINER_L9], BUG_CATCHER),
        MatchupGroup::new_trainer(vec![&SPEAROW_TRAINER_L14], YOUNGSTER),
        MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L10, &NIDORANM_TRAINER_L10], LASS),
        MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L11, &METAPOD_TRAINER_L11], BUG_CATCHER),
        MatchupGroup::new_trainer(vec![&JIGGLYPUFF_TRAINER_L14], LASS).with_encounter_depth(0, 25),
        MatchupGroup::new_wild(&ZUBAT_WILD_L8),
        MatchupGroup::new_wild(&ZUBAT_WILD_L7),
        MatchupGroup::new_trainer(vec![&WEEDLE_TRAINER_L11, &KAKUNA_TRAINER_L11], BUG_CATCHER),
        MatchupGroup::new_wild(&ZUBAT_WILD_L9),
        MatchupGroup::new_trainer(vec![&CLEFAIRY_TRAINER_L14], LASS),
        MatchupGroup::new_wild(&ZUBAT_WILD_L8),
        MatchupGroup::new_wild(&ZUBAT_WILD_L9),
        // HP Up
        // MatchupGroup::new_wild(&GEODUDE_WILD_L9),
        // MatchupGroup::new_trainer(vec![&SANDSHREW_TRAINER_L11, &RATTATA_TRAINER_L11, &ZUBAT_TRAINER_L11], ROCKET),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L7),
        // MatchupGroup::new_wild(&GEODUDE_WILD_L8),
        // MatchupGroup::new_trainer(vec![&MAGNEMITE_TRAINER_L11, &VOLTORB_TRAINER_L11], SUPER_NERD),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L6),
        // Rare Candy
        // MatchupGroup::new_trainer(vec![&CATERPIE_TRAINER_L10, &METAPOD_TRAINER_L10, &CATERPIE_TRAINER_L10], BUG_CATCHER),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L10),
        // MatchupGroup::new_trainer(vec![&ODDISH_TRAINER_L11, &BELLSPROUT_TRAINER_L11], LASS),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L8),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L10),
        // MatchupGroup::new_trainer(vec![&ZUBAT_TRAINER_L12, &EKANS_TRAINER_L12], ROCKET),
        // MatchupGroup::new_wild(&GEODUDE_WILD_L7),
        // MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L10, &RATTATA_TRAINER_L10, &ZUBAT_TRAINER_L10], YOUNGSTER),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L7),
        // MatchupGroup::new_trainer(vec![&GEODUDE_TRAINER_L10, &GEODUDE_TRAINER_L10, &ONIX_TRAINER_L10], HIKER),
        // MatchupGroup::new_wild(&GEODUDE_WILD_L10),
        // MatchupGroup::new_wild(&GEODUDE_WILD_L8),
        // MatchupGroup::new_trainer(vec![&RATICATE_TRAINER_L16], ROCKET),
        // MatchupGroup::new_wild(&GEODUDE_WILD_L10),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L11),
        // MatchupGroup::new_wild(&PARAS_WILD_L10),
        // MatchupGroup::new_trainer(vec![&RATTATA_TRAINER_L13, &ZUBAT_TRAINER_L13], ROCKET),
        // MatchupGroup::new_wild(&ZUBAT_WILD_L9),
        // MatchupGroup::new_trainer(vec![&GRIMER_TRAINER_L12, &VOLTORB_TRAINER_L12, &KOFFING_TRAINER_L12], SUPER_NERD),
        // MatchupGroup::new_wild(&GEODUDE_WILD_L9),
    ];
    let mut win_rates = WinRates::default();
    let mut count = 0u8;
    for matchup in matchups {
        for encounter in &matchup.party {
            let ai = encounter.pokemon;
            count += 1u8;
            let mut battle = Battle::new(&player, &PlayerPolicy::MostDmg, ai, &matchup.ai);
            let started = Instant::now();
            let outcome = battle.run(encounter.max_depth);
            let elapsed = started.elapsed();
            println!(
                "Encounter {count}: {} v L{} {} -> W: {:.2}% | L: {:.2}% | U: {:.10}% | Time: {elapsed:.1?}",
                player.active.name(),
                ai.level,
                ai.name(),
                outcome.win * 100.0f64,
                outcome.lose * 100.0f64,
                outcome.unresolved * 100.0f64
            );
            win_rates.record(&matchup.kind, &outcome);
            player.gain_experience(ai, &matchup.kind);
        }
        player.finish_matchup();
    }
    println!("\n{win_rates}");
}
