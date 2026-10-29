#![cfg_attr(rustfmt, rustfmt::skip)]
use std::sync::LazyLock;

use crate::{
    data::{moves::BIDE, species::*}, pokemon::{Dvs, Pokemon},
};
 
pub static CATERPIE_TRAINER_L6: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CATERPIE, Dvs::TRAINER, 6));
pub static CATERPIE_TRAINER_L8: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CATERPIE, Dvs::TRAINER, 8));
pub static CATERPIE_TRAINER_L9: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CATERPIE, Dvs::TRAINER, 9));
pub static CATERPIE_TRAINER_L10: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CATERPIE, Dvs::TRAINER, 10));
pub static CATERPIE_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CATERPIE, Dvs::TRAINER, 11));
pub static CHARMANDER_TRAINER_L5: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CHARMANDER, Dvs::TRAINER, 5));
pub static CHARMANDER_TRAINER_L8: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CHARMANDER, Dvs::TRAINER, 8));
pub static CLEFAIRY_TRAINER_L14: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&CLEFAIRY, Dvs::TRAINER, 14));
pub static DIGLETT_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&DIGLETT, Dvs::TRAINER, 11));
pub static EKANS_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&EKANS, Dvs::TRAINER, 11));
pub static GEODUDE_TRAINER_L12: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&GEODUDE, Dvs::TRAINER, 12));
pub static JIGGLYPUFF_TRAINER_L14: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&JIGGLYPUFF, Dvs::TRAINER, 14));
pub static KAKUNA_TRAINER_L7: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&KAKUNA, Dvs::TRAINER, 7));
pub static KAKUNA_TRAINER_L9: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&KAKUNA, Dvs::TRAINER, 9));
pub static KAKUNA_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&KAKUNA, Dvs::TRAINER, 11));
pub static KAKUNA_WILD_L5: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&KAKUNA, Dvs::WILD, 5));
pub static METAPOD_TRAINER_L8: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&METAPOD, Dvs::TRAINER, 8));
pub static METAPOD_TRAINER_L9: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&METAPOD, Dvs::TRAINER, 9));
pub static METAPOD_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&METAPOD, Dvs::TRAINER, 11));
pub static NIDORANF_TRAINER_L6: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&NIDORAN_F, Dvs::TRAINER, 6));
pub static NIDORANM_TRAINER_L6: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&NIDORAN_M, Dvs::TRAINER, 6));
pub static NIDORANM_TRAINER_L10: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&NIDORAN_M, Dvs::TRAINER, 10));
pub static ONIX_BROCK_L14: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&ONIX, Dvs::TRAINER, 14).with_move(&BIDE));
pub static PIDGEY_TRAINER_L9: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&PIDGEY, Dvs::TRAINER, 9));
pub static PIDGEY_WILD_L2: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&PIDGEY, Dvs::WILD, 2));
pub static PIDGEY_WILD_L3: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&PIDGEY, Dvs::WILD, 3));
pub static RATTATA_WILD_L2: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&RATTATA, Dvs::WILD, 2));
pub static RATTATA_WILD_L3: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&RATTATA, Dvs::WILD, 3));
pub static RATTATA_TRAINER_L10: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&RATTATA, Dvs::TRAINER, 10));
pub static RATTATA_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&RATTATA, Dvs::TRAINER, 11));
pub static SANDSHREW_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&SANDSHREW, Dvs::TRAINER, 11));
pub static SPEAROW_TRAINER_L14: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&SPEAROW, Dvs::TRAINER, 14));
pub static WEEDLE_TRAINER_L6: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&WEEDLE, Dvs::TRAINER, 6));
pub static WEEDLE_TRAINER_L7: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&WEEDLE, Dvs::TRAINER, 7));
pub static WEEDLE_TRAINER_L9: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&WEEDLE, Dvs::TRAINER, 9));
pub static WEEDLE_TRAINER_L10: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&WEEDLE, Dvs::TRAINER, 10));
pub static WEEDLE_TRAINER_L11: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&WEEDLE, Dvs::TRAINER, 11));
pub static WEEDLE_WILD_L3: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&WEEDLE, Dvs::WILD, 3)); 
pub static WEEDLE_WILD_L4: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&WEEDLE, Dvs::WILD, 4));
pub static ZUBAT_WILD_L7: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&ZUBAT, Dvs::WILD, 7));
pub static ZUBAT_WILD_L8: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&ZUBAT, Dvs::WILD, 8));
pub static ZUBAT_WILD_L9: LazyLock<Pokemon> = LazyLock::new(|| Pokemon::from_species(&ZUBAT, Dvs::WILD, 9));
