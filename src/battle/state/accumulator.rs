use core::hash::BuildHasher as _;

use crate::battle::state::key::{BattleKey, StateMap};

#[derive(Default)]
pub struct StateAccumulator {
    pub entries: StateMap<BattleKey>,
    tiles: Vec<HpTile>,
    last_tile: usize,
}

impl StateAccumulator {
    #[inline]
    pub(crate) fn accumulate(&mut self, key: BattleKey, probability: f64) {
        if self.tiles.is_empty() {
            *self.entries.entry(key).or_insert(0.0f64) += probability;
            return;
        }
        let (tile_key, offset) = key.into_hp_tile();
        let previous = &self.tiles[self.last_tile];
        let tile_idx = if previous.key.as_ref() == Some(&tile_key) {
            self.last_tile
        } else {
            let cache_idx = self.entries.hasher().hash_one(&tile_key) as usize & (self.tiles.len() - 1);
            let cached = &mut self.tiles[cache_idx];
            if cached.key.as_ref() != Some(&tile_key) {
                cached.flush(&mut self.entries);
                cached.key = Some(tile_key);
            }
            cache_idx
        };
        let tile = &mut self.tiles[tile_idx];
        tile.probabilities[offset] += probability;
        tile.occupied[offset / 64] |= 1 << (offset % 64);
        self.last_tile = tile_idx;
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
        self.tiles.clear();
        self.last_tile = 0;
    }

    pub(crate) fn enable_cache(&mut self) {
        if self.tiles.is_empty() {
            self.tiles.resize_with(4_096, HpTile::new);
        }
    }

    pub(crate) fn flush(&mut self) {
        for tile in &mut self.tiles {
            tile.flush(&mut self.entries);
        }
    }
}

struct HpTile {
    key: Option<BattleKey>,
    probabilities: [f64; 256],
    occupied: [u64; 4],
}

impl HpTile {
    const fn new() -> Self {
        Self {
            key: None,
            probabilities: [0.0; 256],
            occupied: [0; 4],
        }
    }

    fn flush(&mut self, destination: &mut StateMap<BattleKey>) {
        let Some(tile_key) = self.key.take() else {
            return;
        };
        for (word_idx, occupied) in self.occupied.iter_mut().enumerate() {
            while *occupied != 0 {
                let offset = word_idx * 64 + occupied.trailing_zeros() as usize;
                *destination.entry(tile_key.with_hp_offset(offset)).or_insert(0.0f64) += self.probabilities[offset];
                self.probabilities[offset] = 0.0f64;
                *occupied &= *occupied - 1;
            }
        }
    }
}
