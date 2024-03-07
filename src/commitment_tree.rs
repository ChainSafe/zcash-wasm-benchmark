/**
 * Defines a commitment tree for Orchard that can be used for benchmarking purposes
 */
use zcash_primitives::consensus::BlockHeight;
use shardtree::store::memory::MemoryShardStore;
use shardtree::ShardTree;
use incrementalmerkletree::{Position, Retention};

use wasm_bindgen::prelude::*;

pub const ORCHARD_SHARD_HEIGHT: u8 = { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 } / 2;

type OrchardMemoryShardStore = MemoryShardStore<orchard::tree::MerkleHashOrchard, BlockHeight>;

pub type OrchardCommitmentTree = ShardTree<OrchardMemoryShardStore, { ORCHARD_SHARD_HEIGHT * 2 }, ORCHARD_SHARD_HEIGHT>;

// max number of checkpoints our tree impl can cache to jump back to
const MAX_CHECKPOINTS: usize = 1;

#[wasm_bindgen]
pub fn bench_tree() {
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);

    let values = (0..(1 << ORCHARD_SHARD_HEIGHT) as u64)
        .map(|i| (
            orchard::tree::MerkleHashOrchard::from_bytes(&[0; 32]).unwrap(),
            Retention::Ephemeral,
        ));

    tree.batch_insert(Position::from(0), values);

    
}
