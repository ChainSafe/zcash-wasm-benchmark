/**
 * Defines a commitment tree for Orchard that can be used for benchmarking purposes
 */
use std::convert::TryInto;

use incrementalmerkletree::{Position, Retention, frontier::Frontier};
use orchard::note::ExtractedNoteCommitment;
use orchard::tree::MerkleHashOrchard;
use rayon::prelude::*;
use shardtree::store::memory::MemoryShardStore;
use shardtree::ShardTree;
use wasm_bindgen::prelude::*;
use web_sys::console;
use zcash_client_backend::data_api::SAPLING_SHARD_HEIGHT;
use zcash_primitives::consensus::BlockHeight;

use crate::CompactAction;

pub const ORCHARD_SHARD_HEIGHT: u8 = { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 } / 2;

pub type OrchardMemoryShardStore = MemoryShardStore<orchard::tree::MerkleHashOrchard, BlockHeight>;

pub type OrchardCommitmentTree =
    ShardTree<OrchardMemoryShardStore, { ORCHARD_SHARD_HEIGHT * 2 }, ORCHARD_SHARD_HEIGHT>;

pub type OrchardFrontier = Frontier<orchard::tree::MerkleHashOrchard, { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 }>;

// insert n_nodes integers into a new tree and benchmark it
pub fn batch_insert_mock_data(tree: &mut OrchardCommitmentTree, n_nodes: usize, n_genwitness: usize) {
    let mut b = [0_u8; 32];

    let commitments = (0..n_nodes)
        .map(move |i| {
            b[..4].copy_from_slice(&i.to_be_bytes());
            MerkleHashOrchard::from_bytes(&b).unwrap()
        })
        .collect::<Vec<_>>();

    benchmark_tree(tree, Position::from(0), &commitments, n_genwitness);
}

/// Insert all notes from a batch of transactions into an in-memory commitment tree starting from a given position
pub fn batch_insert_from_actions(tree: &mut OrchardCommitmentTree, start_position: Position, actions: Vec<CompactAction>, n_genwitness: usize) {
    let commitments = actions
        .iter()
        .map(|action| {
            MerkleHashOrchard::from_cmx(&action.cmx())
        })
        .collect::<Vec<_>>();

    benchmark_tree(tree, start_position, &commitments, n_genwitness);
}

/// Run a benchmark of the tree with an iterator of commitments to add.
/// 
/// starts with an initial_frontier. Can pass OrchardFrontier::empty() to start with an empty tree
/// 
/// The benchmarks marks the first 10 elements as ones we are interested in maintaining the witnesses for
/// and then adds the remainder as ephemeral nodes (ones that will be pruned and just serve to update the witness)
/// n_genwitness is the number of nodes to mark as needing a witness generated for them
fn benchmark_tree(tree: &mut OrchardCommitmentTree, start_position: Position, commitments: &[MerkleHashOrchard], n_genwitness: usize) {

    console::log_1(
        &format!("Adding {} commitments to tree", commitments.len())
            .as_str()
            .into(),
    );
    console::log_1(
        &format!("Maintaining witness for {} leaves", n_genwitness)
            .as_str()
            .into(),
    );

    // mark the first n_genwitness to generate witnesses for
    let ours = commitments
        .iter()
        .clone()
        .take(n_genwitness)
        .map(|cmx| (*cmx, Retention::Marked));

    console::time_with_label("Adding our notes to tree");
    let (last_added, _incomplete) = tree.batch_insert(start_position, ours).unwrap().unwrap();
    console::time_end_with_label("Adding our notes to tree");

    console::time_with_label("Updating witnesses with rest");
    // Create subtrees from the note commitments in parallel.
    const CHUNK_SIZE: usize = 1024;
    let start_position = last_added + 1;

    let subtrees = commitments[n_genwitness..]
        .par_chunks(CHUNK_SIZE)
        .enumerate()
        .filter_map(|(i, chunk)| {
            let start = start_position + (i * CHUNK_SIZE) as u64;
            let end = start + chunk.len() as u64;

            shardtree::LocatedTree::from_iter(
                start..end,
                SAPLING_SHARD_HEIGHT.into(),
                chunk
                    .iter()
                    .map(|cmx| (*cmx, Retention::<BlockHeight>::Marked)),
            )
        })
        .map(|res| (res.subtree, res.checkpoints))
        .collect::<Vec<_>>();

    // add the subtrees
    for (subtree, checkpoints) in subtrees {
        tree.insert_tree(subtree, checkpoints).unwrap();
    }
    console::time_end_with_label("Updating witnesses with rest");

    console::time_with_label("Calculating witness for first added");
    let witness = tree
        .witness_at_checkpoint_depth(start_position, 0)
        .unwrap();
    console::time_end_with_label("Calculating witness for first added");

    console::log_1(&format!("Witness {:?}", witness).as_str().into());
}
