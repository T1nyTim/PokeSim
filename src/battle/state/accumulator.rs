use core::hash::BuildHasher as _;

use crate::battle::state::key::{BattleKey, StateMap};

#[derive(Default)]
pub struct StateAccumulator {
    pub frontier: StateFrontier,
    tiles: Vec<HpTile>,
    last_tile: usize,
}

impl StateAccumulator {
    #[inline]
    pub(crate) fn accumulate(&mut self, key: BattleKey, probability: f64) {
        if self.tiles.is_empty() {
            *self.frontier.entries.entry(key).or_insert(0.0f64) += probability;
            return;
        }
        let (tile_key, offset) = key.into_hp_tile();
        let previous = &self.tiles[self.last_tile];
        let tile_idx = if previous.key.as_ref() == Some(&tile_key) {
            self.last_tile
        } else {
            let cache_idx = self.frontier.tile_ids.hasher().hash_one(&tile_key) as usize & (self.tiles.len() - 1);
            let cached = &mut self.tiles[cache_idx];
            if cached.key.as_ref() != Some(&tile_key) {
                cached.flush(&mut self.frontier);
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
        self.frontier.clear();
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
            tile.flush(&mut self.frontier);
        }
    }
}

#[derive(Default)]
pub struct StateFrontier {
    pub entries: StateMap<BattleKey>,
    tile_ids: StateMap<BattleKey, u64>,
    tile_keys: Vec<(BattleKey, u64)>,
    hp_entries: StateMap<u64>,
}

impl StateFrontier {
    pub(crate) fn action_ordered(&self) -> Vec<(u64, &BattleKey, f64)> {
        let mut ordered_states = Vec::with_capacity(self.len());
        ordered_states.extend(
            self.entries
                .iter()
                .map(|(state_key, &probability)| (state_key.action_batch_order(), state_key, probability)),
        );
        ordered_states.extend(self.hp_entries.iter().map(|(&hp_key, &probabillity)| {
            let tile_id = hp_key >> 8u8;
            let offset = (hp_key ^ tile_id) & 255;
            let (tile_key, order) = &self.tile_keys[tile_id as usize];
            (BattleKey::batch_order_with_hp_offset(*order, offset), tile_key, probabillity)
        }));
        ordered_states.sort_unstable_by_key(|entry| entry.0);
        ordered_states
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
        self.tile_ids.clear();
        self.tile_keys.clear();
        self.hp_entries.clear();
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.hp_entries.is_empty()
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len() + self.hp_entries.len()
    }

    pub(crate) fn total_probability(&self) -> f64 {
        self.entries.values().chain(self.hp_entries.values()).copied().sum()
    }

    fn tile_id(&mut self, tile_key: BattleKey) -> u64 {
        let tile_keys = &mut self.tile_keys;
        *self.tile_ids.entry(tile_key.clone()).or_insert_with(|| {
            let id = tile_keys.len() as u64;
            let order = tile_key.action_batch_order();
            tile_keys.push((tile_key, order));
            id
        })
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

    fn flush(&mut self, destination: &mut StateFrontier) {
        let Some(tile_key) = self.key.take() else {
            return;
        };
        let tile_id = destination.tile_id(tile_key);
        for (word_idx, occupied) in self.occupied.iter_mut().enumerate() {
            while *occupied != 0 {
                let offset = word_idx * 64 + occupied.trailing_zeros() as usize;
                let hp_key = (tile_id << 8u8) | (offset as u64 ^ (tile_id & 255));
                *destination.hp_entries.entry(hp_key).or_insert(0.0f64) += self.probabilities[offset];
                self.probabilities[offset] = 0.0f64;
                *occupied &= *occupied - 1;
            }
        }
    }
}
