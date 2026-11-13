#[derive(Eq, Hash, PartialEq)]
pub enum Type {
    Bug,
    Dragon,
    Electric,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,
    Normal,
    Poison,
    Psychic,
    Rock,
    Water,
}

impl Type {
    pub(crate) const fn effectiveness(&self, target: &Self) -> u32 {
        self.ordered_effectiveness(target).1
    }

    pub(crate) const fn is_special(&self) -> bool {
        matches!(
            self,
            Self::Dragon | Self::Electric | Self::Fire | Self::Grass | Self::Ice | Self::Psychic | Self::Water
        )
    }

    pub(crate) fn ai_effectiveness(&self, targets: &[Self; 2]) -> u32 {
        self.ordered_effectiveness(&targets[0]).min(self.ordered_effectiveness(&targets[1])).1
    }

    #[expect(clippy::match_same_arms, reason = "Order is important here")]
    const fn ordered_effectiveness(&self, target: &Self) -> (u8, u32) {
        match (self, target) {
            (&Self::Bug, &Self::Fire) => (0, 5),
            (&Self::Bug, &Self::Grass) => (1, 20),
            (&Self::Bug, &Self::Fighting | &Self::Flying) => (2, 5),
            (&Self::Bug, &Self::Psychic) => (3, 20),
            (&Self::Bug, &Self::Ghost) => (4, 5),
            (&Self::Bug, &Self::Poison) => (5, 20),
            (&Self::Dragon, &Self::Dragon) => (0, 20),
            (&Self::Electric, &Self::Water) => (0, 20),
            (&Self::Electric, &Self::Electric | &Self::Grass) => (1, 5),
            (&Self::Electric, &Self::Ground) => (2, 0),
            (&Self::Electric, &Self::Flying) => (3, 20),
            (&Self::Electric, &Self::Dragon) => (4, 5),
            (&Self::Fighting, &Self::Normal) => (0, 20),
            (&Self::Fighting, &Self::Poison | &Self::Flying | &Self::Psychic | &Self::Bug) => (1, 5),
            (&Self::Fighting, &Self::Rock | &Self::Ice) => (2, 20),
            (&Self::Fighting, &Self::Ghost) => (3, 0),
            (&Self::Fire, &Self::Grass | &Self::Ice) => (0, 20),
            (&Self::Fire, &Self::Fire | &Self::Water) => (1, 5),
            (&Self::Fire, &Self::Bug) => (2, 20),
            (&Self::Fire, &Self::Rock) => (3, 5),
            (&Self::Fire, &Self::Dragon) => (4, 5),
            (&Self::Flying, &Self::Electric) => (0, 5),
            (&Self::Flying, &Self::Fighting | &Self::Bug | &Self::Grass) => (1, 20),
            (&Self::Flying, &Self::Rock) => (2, 5),
            (&Self::Ghost, &Self::Ghost) => (0, 20),
            (&Self::Ghost, &Self::Normal | &Self::Psychic) => (1, 0),
            (&Self::Grass, &Self::Water) => (0, 20),
            (&Self::Grass, &Self::Grass | &Self::Fire) => (1, 5),
            (&Self::Grass, &Self::Ground) => (2, 20),
            (&Self::Grass, &Self::Bug | &Self::Poison) => (3, 5),
            (&Self::Grass, &Self::Rock) => (4, 20),
            (&Self::Grass, &Self::Flying) => (5, 5),
            (&Self::Grass, &Self::Dragon) => (6, 5),
            (&Self::Ground, &Self::Flying) => (0, 0),
            (&Self::Ground, &Self::Fire | &Self::Electric) => (1, 20),
            (&Self::Ground, &Self::Grass | &Self::Bug) => (2, 5),
            (&Self::Ground, &Self::Rock | &Self::Poison) => (3, 20),
            (&Self::Ice, &Self::Ice | &Self::Water) => (0, 5),
            (&Self::Ice, &Self::Grass | &Self::Ground | &Self::Flying) => (1, 20),
            (&Self::Ice, &Self::Dragon) => (2, 20),
            (&Self::Normal, &Self::Rock) => (0, 5),
            (&Self::Normal, &Self::Ghost) => (1, 0),
            (&Self::Poison, &Self::Grass) => (0, 20),
            (&Self::Poison, &Self::Poison | &Self::Ground) => (1, 5),
            (&Self::Poison, &Self::Bug) => (2, 20),
            (&Self::Poison, &Self::Rock | &Self::Ghost) => (3, 5),
            (&Self::Psychic, &Self::Psychic) => (0, 5),
            (&Self::Psychic, &Self::Fighting | &Self::Poison) => (1, 20),
            (&Self::Rock, &Self::Fire) => (0, 20),
            (&Self::Rock, &Self::Fighting | &Self::Ground) => (1, 5),
            (&Self::Rock, &Self::Flying | &Self::Bug | &Self::Ice) => (2, 20),
            (&Self::Water, &Self::Fire | &Self::Rock) => (0, 20),
            (&Self::Water, &Self::Water | &Self::Grass) => (1, 5),
            (&Self::Water, &Self::Ground) => (2, 20),
            (&Self::Water, &Self::Dragon) => (3, 5),
            _ => (u8::MAX, 10),
        }
    }
}
