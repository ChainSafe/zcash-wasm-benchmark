/**
 * Defines a commitment tree for Orchard that can be used for benchmarking purposes
 */

use incrementalmerkletree::{Position, Retention, frontier::Frontier};
use orchard::tree::MerkleHashOrchard;
use rayon::prelude::*;
use shardtree::store::memory::MemoryShardStore;
use shardtree::ShardTree;
use zcash_primitives::consensus::BlockHeight;

use crate::CompactAction;

pub const ORCHARD_SHARD_HEIGHT: u8 = { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 } / 2;

pub type OrchardMemoryShardStore = MemoryShardStore<orchard::tree::MerkleHashOrchard, BlockHeight>;

pub type OrchardCommitmentTree =
    ShardTree<OrchardMemoryShardStore, { ORCHARD_SHARD_HEIGHT * 2 }, ORCHARD_SHARD_HEIGHT>;

pub type OrchardFrontier = Frontier<orchard::tree::MerkleHashOrchard, { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 }>;

// insert n_nodes integers into a new tree and benchmark it
pub fn batch_insert_mock_data(tree: &mut OrchardCommitmentTree, n_nodes: usize) {
    let mut b = [0_u8; 32];

    let commitments = (0..n_nodes)
        .map(move |i| {
            b[..4].copy_from_slice(&i.to_be_bytes());
            MerkleHashOrchard::from_bytes(&b).unwrap()
        })
        .collect::<Vec<_>>();

        parallel_batch_add_commitments(tree, Position::from(0), &commitments);
}

/// Insert all notes from a batch of transactions into an in-memory commitment tree starting from a given position
pub fn batch_insert_from_actions(tree: &mut OrchardCommitmentTree, start_position: Position, actions: Vec<CompactAction>) {
    let commitments = actions
        .iter()
        .map(|action| {
            MerkleHashOrchard::from_cmx(&action.cmx())
        })
        .collect::<Vec<_>>();

        parallel_batch_add_commitments(tree, start_position, &commitments);
}

fn batch_add_commitments(tree: &mut OrchardCommitmentTree, start_position: Position, commitments: &[MerkleHashOrchard]) {
    let values = commitments.iter().enumerate().map(|(i, cmx)| (*cmx, if i == 0 { Retention::Marked } else { Retention::Ephemeral }));
    // note that all leaves are being marked ephemeral and will be pruned out
    // once they have been used to updated any witnesses the tree is tracking
    tree.batch_insert(start_position, values).unwrap();
}

/// Use rayon to parallelize adding batch of commitments to the tree by building the shards
/// in parallel then adding them in after
/// based on the code here (https://github.com/zcash/librustzcash/blob/b3d06ba41904965f3b8165011e14e1d13b3c7b81/zcash_client_sqlite/src/lib.rs#L730)
fn parallel_batch_add_commitments(tree: &mut OrchardCommitmentTree, start_position: Position, commitments: &[MerkleHashOrchard]) {

    // Create subtrees from the note commitments in parallel.
    const CHUNK_SIZE: usize = 1024;

    let subtrees = commitments
        .par_chunks(CHUNK_SIZE)
        .enumerate()
        .filter_map(|(i, chunk)| {
            let start = start_position + (i * CHUNK_SIZE) as u64;
            let end = start + chunk.len() as u64;

            shardtree::LocatedTree::from_iter(
                start..end,
                ORCHARD_SHARD_HEIGHT.into(),
                chunk
                    .iter()
                    .enumerate()
                    .map(|(i, cmx)| (*cmx, if i == 0 { Retention::Marked } else { Retention::Ephemeral })), 
                // note that all leaves marked ephemeral  (all but the first added) will be pruned out
                // once they have been used to updated any witnesses the tree is tracking
            )
        })
        .map(|res| (res.subtree, res.checkpoints))
        .collect::<Vec<_>>();

    // add the subtrees
    for (subtree, checkpoints) in subtrees {
        tree.insert_tree(subtree, checkpoints).unwrap();
    }
}
