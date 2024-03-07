/**
 * Defines a commitment tree for Orchard that can be used for benchmarking purposes
 */
use std::convert::TryInto;

use incrementalmerkletree::{Position, Retention};
use orchard::note::ExtractedNoteCommitment;
use orchard::tree::MerkleHashOrchard;
use shardtree::store::memory::MemoryShardStore;
use shardtree::ShardTree;
use wasm_bindgen::prelude::*;
use web_sys::console;
use zcash_primitives::consensus::BlockHeight;
use rayon::prelude::*;
use zcash_client_backend::data_api::SAPLING_SHARD_HEIGHT;

use crate::types::CompactTx;

pub const ORCHARD_SHARD_HEIGHT: u8 = { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 } / 2;

type OrchardMemoryShardStore = MemoryShardStore<orchard::tree::MerkleHashOrchard, BlockHeight>;

pub type OrchardCommitmentTree =
    ShardTree<OrchardMemoryShardStore, { ORCHARD_SHARD_HEIGHT * 2 }, ORCHARD_SHARD_HEIGHT>;

// max number of checkpoints our tree impl can cache to jump back to
const MAX_CHECKPOINTS: usize = 1;

// insert n_nodes integers into a new tree and benchmark it
#[wasm_bindgen]
pub fn batch_insert_mock_data(n_nodes: usize, n_genwitness: usize) {
    let mut b = [0_u8; 32];

    let commitments = (0..n_nodes).map(move |i| {
        b[..4].copy_from_slice(&i.to_be_bytes());
        MerkleHashOrchard::from_bytes(&b).unwrap()
    }).collect::<Vec<_>>();

    benchmark_tree(&commitments, n_genwitness);
}

/// Insert all notes from a batch of transactions into an in-memory commitment tree
#[wasm_bindgen]
pub fn batch_insert_txn_notes(vtxs: Box<[CompactTx]>, n_genwitness: usize) {
    let commitments = vtxs // create an iterator over the commitments to add
        .iter()
        .flat_map(|tx| {
            tx.actions
                .iter()
                .map(|action| {
                    ExtractedNoteCommitment::from_bytes(action.cmx.as_ref().try_into().unwrap())
                        .unwrap()
                })
                .collect::<Vec<_>>()
        })
        .map(|cmx| MerkleHashOrchard::from_cmx(&cmx)).collect::<Vec<_>>();

    benchmark_tree(&commitments, n_genwitness);
}

/// Run a benchmark of the tree with an iterator of commitments to add.
/// The benchmarks marks the first 10 elements as ones we are interested in maintaining the witnesses for
/// and then adds the remainder as ephemeral nodes (ones that will be pruned and just serve to update the witness)
/// n_genwitness is the number of nodes to mark as needing a witness generated for them
fn benchmark_tree(
    commitments: &[MerkleHashOrchard],
    n_genwitness: usize,
) {
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);

    // checkpoint the tree at the start so it actually builds witnesses as we go
    // and prunes out ephemeral nodes. Otherwise it will just store all ephemeral nodes and
    // to hash later which uses more memory and isn't a useful benchmark
    let _success = tree.checkpoint(0.into()).unwrap();

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

    // mark the first n_genwitness, the rest as ephemeral
    let ours = commitments
        .iter()
        .clone()
        .take(n_genwitness)
        .map(|cmx| (*cmx, Retention::Marked));
    // let rest = commitments
    //     .skip(n_genwitness)
    //     .map(|cmx| (cmx, Retention::<BlockHeight>::Ephemeral));

    console::time_with_label("Adding our notes to tree");
    let (last_added, _incomplete) = tree.batch_insert(Position::from(0), ours).unwrap().unwrap();
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
                chunk.iter().map(|cmx| (*cmx, Retention::<BlockHeight>::Ephemeral)),
            )
        })
        .map(|res| (res.subtree, res.checkpoints))
        .collect::<Vec<_>>();


    // let (_, _incomplete) = tree.batch_insert(last_added + 1, rest).unwrap().unwrap();
    console::time_end_with_label("Updating witnesses with rest");

    console::time_with_label("Calculating witness for first added");
    let witness = tree
        .witness_at_checkpoint_depth(Position::from(0), 0)
        .unwrap();
    console::time_end_with_label("Calculating witness for first added");

    console::log_1(&format!("Witness {:?}", witness).as_str().into());
}
