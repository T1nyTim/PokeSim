use core::{hash::BuildHasher as _, mem::take};

use crate::battle::state::{BattleKey, StateMap};

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
        let (tile_key, block_idx, cell_idx) = key.into_hp_tile();
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
        let block = &mut self.tiles[tile_idx].blocks[block_idx];
        block.probabilities[cell_idx] += probability;
        block.occupied |= 1 << cell_idx;
        self.last_tile = tile_idx;
    }

    pub(crate) fn clear(&mut self) {
        self.frontier.clear();
        self.tiles.clear();
        self.last_tile = 0;
    }

    pub(crate) fn enable_cache(&mut self) {
        if self.tiles.is_empty() {
            self.tiles.resize_with(4_096, HpTile::default);
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
    tile_ids: StateMap<BattleKey, usize>,
    tiles: Vec<FrontierTile>,
    hp_blocks: Vec<HpBlock>,
    hp_len: usize,
}

impl StateFrontier {
    pub(crate) fn action_ordered(&self) -> Vec<(u64, &BattleKey, f64)> {
        let mut ordered_states = Vec::with_capacity(self.len());
        ordered_states.extend(
            self.entries
                .iter()
                .map(|(state_key, &probability)| (state_key.action_batch_order(), state_key, probability)),
        );
        for tile in &self.tiles {
            for (block_idx, &block_id) in tile.blocks.iter().enumerate() {
                if block_id == 0 {
                    continue;
                }
                let block = &self.hp_blocks[(block_id - 1) as usize];
                let mut occupied = block.occupied;
                while occupied != 0 {
                    let cell_idx = occupied.trailing_zeros() as usize;
                    let order = BattleKey::batch_order_with_hp_offset(tile.order, block_idx, cell_idx);
                    ordered_states.push((order, &tile.key, block.probabilities[cell_idx]));
                    occupied &= occupied - 1;
                }
            }
        }
        ordered_states.sort_unstable_by_key(|entry| entry.0);
        ordered_states
    }

    pub(crate) fn clear(&mut self) {
        self.entries.clear();
        self.tile_ids.clear();
        self.tiles.clear();
        self.hp_blocks.clear();
        self.hp_len = 0;
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.hp_len == 0
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len() + self.hp_len
    }

    pub(crate) fn total_probability(&self) -> f64 {
        self.entries
            .values()
            .chain(self.hp_blocks.iter().flat_map(|block| &block.probabilities))
            .copied()
            .sum()
    }

    fn merge_tile(&mut self, tile_key: BattleKey, blocks: &mut [HpBlock; 16]) {
        let tiles = &mut self.tiles;
        let tile_id = *self.tile_ids.entry(tile_key.clone()).or_insert_with(|| {
            let id = tiles.len();
            tiles.push(FrontierTile::new(tile_key));
            id
        });
        for (block_id, source) in tiles[tile_id].blocks.iter_mut().zip(blocks) {
            if source.occupied == 0 {
                continue;
            }
            if *block_id == 0 {
                self.hp_len += source.occupied.count_ones() as usize;
                self.hp_blocks.push(take(source));
                *block_id = u32::try_from(self.hp_blocks.len()).expect("Too many HP blocks");
            } else {
                self.hp_len += self.hp_blocks[(*block_id - 1) as usize].merge(source);
            }
        }
    }
}

struct FrontierTile {
    key: BattleKey,
    order: u64,
    blocks: [u32; 16],
}

impl FrontierTile {
    fn new(key: BattleKey) -> Self {
        Self {
            order: key.action_batch_order(),
            key,
            blocks: [0; 16],
        }
    }
}

#[derive(Default)]
struct HpBlock {
    probabilities: [f64; 16],
    occupied: u16,
}

impl HpBlock {
    fn merge(&mut self, source: &mut Self) -> usize {
        let mut occupied = take(&mut source.occupied);
        let added = (occupied & !self.occupied).count_ones() as usize;
        self.occupied |= occupied;
        while occupied != 0 {
            let cell_idx = occupied.trailing_zeros() as usize;
            self.probabilities[cell_idx] += take(&mut source.probabilities[cell_idx]);
            occupied &= occupied - 1;
        }
        added
    }
}

#[derive(Default)]
struct HpTile {
    key: Option<BattleKey>,
    blocks: [HpBlock; 16],
}

impl HpTile {
    fn flush(&mut self, destination: &mut StateFrontier) {
        if let Some(tile_key) = self.key.take() {
            destination.merge_tile(tile_key, &mut self.blocks);
        }
    }
}
