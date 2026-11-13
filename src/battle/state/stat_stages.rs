use crate::data::Move;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct StatStages {
    pub atk: i8,
    pub def: i8,
    pub spd: i8,
    pub spc: i8,
    pub acc: i8,
    pub eva: i8,
}

impl StatStages {
    pub(crate) fn apply(stat: u16, stage: i8) -> u32 {
        Self::scale(u32::from(stat), stage).clamp(1, 999)
    }

    pub(crate) fn hit_probability(&self, mv: &Move, target_eva: i8) -> f64 {
        f64::from(self.hit_threshold(mv.accuracy(), target_eva)) / 256.0
    }

    pub(super) const fn new(atk: i8, def: i8, spd: i8, spc: i8, acc: i8, eva: i8) -> Self {
        Self {
            atk,
            def,
            spd,
            spc,
            acc,
            eva,
        }
    }

    pub(super) fn hit_threshold(&self, accuracy: u8, target_eva: i8) -> u8 {
        let adjusted = Self::scale(u32::from(accuracy), self.acc).max(1);
        Self::scale(adjusted, -target_eva).clamp(1, 255) as u8
    }

    fn scale(stat: u32, stage: i8) -> u32 {
        let (numerator, denominator) = match stage {
            -6 => (25, 100),
            -5 => (28, 100),
            -4 => (33, 100),
            -3 => (40, 100),
            -2 => (50, 100),
            -1 => (66, 100),
            0 => (1, 1),
            1 => (15, 10),
            2 => (2, 1),
            3 => (25, 10),
            4 => (3, 1),
            5 => (35, 10),
            6 => (4, 1),
            _ => unreachable!("Stat stages must be between -6 and 6"),
        };
        stat * numerator / denominator
    }
}
