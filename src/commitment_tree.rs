/**
 * Defines a commitment tree for Orchard that can be used for benchmarking purposes
 */
use std::convert::TryInto;
use std::io::Cursor;

use futures_util::{stream, StreamExt};
use rayon::prelude::*;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;
use web_sys::console;

use incrementalmerkletree::{frontier::Frontier, Position, Retention};
use orchard::note_encryption::CompactAction;
use orchard::tree::MerkleHashOrchard;
use shardtree::store::memory::MemoryShardStore;
use shardtree::ShardTree;
use zcash_primitives::consensus::BlockHeight;
use zcash_primitives::merkle_tree::read_frontier_v0;

use crate::bench_params::{BenchParams, ShieldedPool};
use crate::block_range_stream::block_range_stream;
use crate::console_log;
use crate::proto;
use crate::WasmGrpcClient;
use crate::PERFORMANCE;

pub const ORCHARD_SHARD_HEIGHT: u8 = { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 } / 2;

// max number of checkpoints our tree impl can cache to jump back to
const MAX_CHECKPOINTS: usize = 1;

pub type OrchardMemoryShardStore = MemoryShardStore<orchard::tree::MerkleHashOrchard, BlockHeight>;

pub type OrchardCommitmentTree =
    ShardTree<OrchardMemoryShardStore, { ORCHARD_SHARD_HEIGHT * 2 }, ORCHARD_SHARD_HEIGHT>;

pub type OrchardFrontier =
    Frontier<orchard::tree::MerkleHashOrchard, { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 }>;

/// Retrieve the tree frontier at the given start block height and then process all note commitments
/// included in blocks between start and end.
/// Finally checks to ensure the computed tree frontier matches the expected frontier at the end block height
#[wasm_bindgen]
pub async fn sync_commitment_tree_bench(params: BenchParams) {
    let BenchParams {
        network,
        pool,
        lightwalletd_url,
        start_block,
        end_block,
        block_batch_size,
    } = params;

    if pool != ShieldedPool::Orchard {
        console::log_1(&"This benchmark is only for Orchard".into());
        return;
    }

    let mut client = WasmGrpcClient::new(Client::new(lightwalletd_url.clone()));

    let init_frontier = fetch_orchard_frontier_at_height(&mut client, start_block - 1)
        .await
        .unwrap();

    // create the tree and initialize it to the initial frontier
    // This also gives us the position to start adding to the tree
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);
    let mut start_position = Position::from(0);

    if let Some(frontier) = init_frontier.take() {
        console_log!(
            "Frontier was found for height {}: {:?}",
            start_block - 1,
            frontier
        );
        start_position = frontier.position() + 1;
        tree.insert_frontier_nodes(
            frontier,
            Retention::Checkpoint {
                id: (start_block - 1).into(),
                is_marked: false,
            },
        )
        .unwrap();
    } else {
        // checkpoint the tree at the start
        let _success = tree.checkpoint(0.into()).unwrap();
    }

    console_log!(
        "orchard commitment tree starting from position: {:?}",
        start_position
    );

    let block_stream = block_range_stream(&mut client, start_block, end_block).await;

    let actions = block_stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .flat_map(|ctx| stream::iter(ctx.actions))
        .map(|x| {
            let action: CompactAction = x.try_into().unwrap();
            action
        })
        .collect::<Vec<_>>()
        .await;

    console_log!("Downloaded and deserialized {} actions", actions.len());
    let update_tree = PERFORMANCE.now();
    batch_insert_from_actions(&mut tree, start_position, actions);
    console_log!(
        "Update commitment tree: {}ms",
        PERFORMANCE.now() - update_tree
    );

    // produce a witness for the first added leaf
    let calc_witness = PERFORMANCE.now();
    let _witness = tree.witness_at_checkpoint_depth(start_position, 0).unwrap();
    console_log!(
        "Produce witness for leftmost leaf: {}ms",
        PERFORMANCE.now() - calc_witness
    );

    // the end frontier should be the witness of the last added commitment
    // this can give us the root of the new tree produced by adding all the commitments
    let end_frontier = fetch_orchard_frontier_at_height(&mut client, end_block)
        .await
        .unwrap();
    assert_eq!(
        end_frontier.root(),
        tree.root_at_checkpoint_depth(0).unwrap()
    );
    console_log!(
        "✅ Computed root for block {} matches lightwalletd ✅",
        end_block
    );
}

async fn fetch_orchard_frontier_at_height(
    client: &mut WasmGrpcClient,
    height: u32,
) -> anyhow::Result<OrchardFrontier> {
    let start = proto::service::BlockId {
        height: height as u64,
        hash: vec![],
    };
    let pb_tree_state = client.get_tree_state(start).await?.into_inner();
    let orchard_tree_frontier_bytes = hex::decode(pb_tree_state.orchard_tree)?;

    let frontier: OrchardFrontier = read_frontier_v0(Cursor::new(orchard_tree_frontier_bytes))?;
    Ok(frontier)
}

// insert n_nodes integers into a new tree and benchmark it
fn batch_insert_mock_data(tree: &mut OrchardCommitmentTree, n_nodes: usize) {
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
fn batch_insert_from_actions(
    tree: &mut OrchardCommitmentTree,
    start_position: Position,
    actions: Vec<CompactAction>,
) {
    let commitments = actions
        .iter()
        .map(|action| MerkleHashOrchard::from_cmx(&action.cmx()))
        .collect::<Vec<_>>();

    parallel_batch_add_commitments(tree, start_position, &commitments);
}

fn batch_add_commitments(
    tree: &mut OrchardCommitmentTree,
    start_position: Position,
    commitments: &[MerkleHashOrchard],
) {
    let values = commitments.iter().enumerate().map(|(i, cmx)| {
        (
            *cmx,
            if i == 0 {
                Retention::Marked
            } else {
                Retention::Ephemeral
            },
        )
    });
    // note that all leaves are being marked ephemeral and will be pruned out
    // once they have been used to updated any witnesses the tree is tracking
    tree.batch_insert(start_position, values).unwrap();
}

/// Use rayon to parallelize adding batch of commitments to the tree by building the shards
/// in parallel then adding them in after
/// based on the code here (https://github.com/zcash/librustzcash/blob/b3d06ba41904965f3b8165011e14e1d13b3c7b81/zcash_client_sqlite/src/lib.rs#L730)
fn parallel_batch_add_commitments(
    tree: &mut OrchardCommitmentTree,
    start_position: Position,
    commitments: &[MerkleHashOrchard],
) {
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
                chunk.iter().enumerate().map(|(i, cmx)| {
                    (
                        *cmx,
                        if i == 0 {
                            Retention::Marked
                        } else {
                            Retention::Ephemeral
                        },
                    )
                }),
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
