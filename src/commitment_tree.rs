/**
 * Defines a commitment tree for Orchard that can be used for benchmarking purposes
 */
use std::convert::TryInto;

use zcash_primitives::consensus::BlockHeight;
use shardtree::store::memory::MemoryShardStore;
use shardtree::ShardTree;
use incrementalmerkletree::{Position, Retention};
use orchard::tree::MerkleHashOrchard;
use orchard::note::ExtractedNoteCommitment;
use wasm_bindgen::prelude::*;
use web_sys::console;

use crate::types::CompactTx;

pub const ORCHARD_SHARD_HEIGHT: u8 = { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 } / 2;

type OrchardMemoryShardStore = MemoryShardStore<orchard::tree::MerkleHashOrchard, BlockHeight>;

pub type OrchardCommitmentTree = ShardTree<OrchardMemoryShardStore, { ORCHARD_SHARD_HEIGHT * 2 }, ORCHARD_SHARD_HEIGHT>;

// max number of checkpoints our tree impl can cache to jump back to
const MAX_CHECKPOINTS: usize = 1;
const N_NODES: usize = 100000;

/// Insert all notes from a batch of transactions into an in-memory commitment tree
#[wasm_bindgen]
pub fn batch_insert_txn_notes(vtxs: Box<[CompactTx]>) {
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);
    
    // checkpoint the tree at the start so it actually builds witnesses as we go
    let _success = tree.checkpoint(0.into()).unwrap();

    let commitments = vtxs // create an iterator over the commitments to add
        .into_iter()
        .map(|tx| {
            tx.actions
                .iter()
                .map(|action| {
                    ExtractedNoteCommitment::from_bytes(action.cmx.as_ref().try_into().unwrap()).unwrap()
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .map(|cmx| MerkleHashOrchard::from_cmx(&cmx));


    console::log_1(&format!("Adding {} commitments to tree", commitments.clone().count()).as_str().into());

    // mark the first 10 as notes belonging to us, the rest as ephemeral only for updating witnesses
    let ours = commitments.clone().take(10).map(|cmx| (cmx, Retention::Marked));
    let rest = commitments.skip(10).map(|cmx| (cmx, Retention::<BlockHeight>::Ephemeral));

    console::time_with_label("Adding our notes to tree");
    let (last_added, _incomplete) = tree.batch_insert(Position::from(0), ours).unwrap().unwrap();
    console::time_end_with_label("Adding our notes to tree");

    console::time_with_label("Updating witnesses with rest");
    let (_, _incomplete) = tree.batch_insert(last_added+1, rest).unwrap().unwrap();
    console::time_end_with_label("Updating witnesses with rest");

    console::time_with_label("Calculating witness for first added");
    let witness = tree.witness_at_checkpoint_depth(Position::from(0), 0).unwrap();
    console::time_end_with_label("Calculating witness for first added");

    console::log_1(&format!("Witness {:?}", witness).as_str().into());
}

#[wasm_bindgen]
pub fn bench_tree() {
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);
    
    let marked_node: u64 = 666;

    let values = (0..N_NODES)
        .map(|i| {
            let mut b = [0_u8; 32];
            b[..4].copy_from_slice(&i.to_be_bytes());

            if i as u64 == marked_node {
                ( MerkleHashOrchard::from_bytes(&b).unwrap(), Retention::Marked )
            } else {
                ( MerkleHashOrchard::from_bytes(&b).unwrap(), Retention::Ephemeral )
            }
        });
        
    console::time_with_label(&format!("Adding {} nodes to tree", N_NODES));
    let _ = tree.batch_insert(Position::from(0), values);
    console::time_end_with_label(&format!("Adding {} nodes to tree", N_NODES));

    console::time_with_label("Calculating witness for marked node");
    let _ = tree.witness_at_checkpoint_depth(Position::from(marked_node), 0);
    console::time_end_with_label("Calculating witness for marked node");

    console::time_with_label("Calculating witness for unmarked node");
    let _ = tree.witness_at_checkpoint_depth(Position::from(10), 0);
    console::time_end_with_label("Calculating witness for unmarked node");

}
