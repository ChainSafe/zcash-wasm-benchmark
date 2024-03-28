use core::panic;
/**
 * Defines a commitment tree for Orchard that can be used for benchmarking purposes
 */
use std::io::Cursor;

use futures_util::{pin_mut, StreamExt};
use incrementalmerkletree::Hashable;
use rayon::prelude::*;
use sapling::note_encryption::{CompactOutputDescription, SaplingDomain};
use shardtree::store::ShardStore;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

use incrementalmerkletree::{frontier::Frontier, Position, Retention};
use orchard::note_encryption::{CompactAction, OrchardDomain};
use orchard::tree::MerkleHashOrchard;
use shardtree::store::memory::MemoryShardStore;
use shardtree::ShardTree;
use zcash_primitives::consensus::BlockHeight;
use zcash_primitives::merkle_tree::read_frontier_v0;

use crate::bench_params::BenchParams;
use crate::block_range_stream::block_contents_batch_stream;
use crate::console_log;
use crate::WasmGrpcClient;

pub const ORCHARD_SHARD_HEIGHT: u8 = { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 } / 2;
pub const SAPLING_SHARD_HEIGHT: u8 = { sapling::NOTE_COMMITMENT_TREE_DEPTH } / 2;

// max number of checkpoints our tree impl can cache to jump back to
const MAX_CHECKPOINTS: usize = 1;

pub type OrchardMemoryShardStore = MemoryShardStore<orchard::tree::MerkleHashOrchard, BlockHeight>;
pub type OrchardCommitmentTree =
    ShardTree<OrchardMemoryShardStore, { ORCHARD_SHARD_HEIGHT * 2 }, ORCHARD_SHARD_HEIGHT>;
pub type OrchardFrontier =
    Frontier<orchard::tree::MerkleHashOrchard, { orchard::NOTE_COMMITMENT_TREE_DEPTH as u8 }>;

pub type SaplingMemoryShardStore = MemoryShardStore<sapling::Node, BlockHeight>;
pub type SaplingCommitmentTree =
    ShardTree<SaplingMemoryShardStore, { SAPLING_SHARD_HEIGHT * 2 }, SAPLING_SHARD_HEIGHT>;
pub type SaplingFrontier = Frontier<sapling::Node, { sapling::NOTE_COMMITMENT_TREE_DEPTH }>;

/// Retrieve the tree frontier at the given start block height and then process all note commitments
/// included in blocks between start and end.
/// Finally checks to ensure the computed tree frontier matches the expected frontier at the end block height
#[wasm_bindgen]
pub async fn sync_commitment_tree_bench(params: BenchParams, n_witnesses: u32) -> f64 {
    let BenchParams {
        network: _,
        pool,
        lightwalletd_url,
        start_block,
        end_block,
        block_batch_size,
    } = params;

    let mut client = WasmGrpcClient::new(Client::new(lightwalletd_url.clone()));
    let (mut orchard_tree, mut orchard_cursor) =
        bootstrap_orchard_tree_from_lightwalletd(&mut client, start_block - 1).await;

    let (mut sapling_tree, mut sapling_cursor) =
        bootstrap_sapling_tree_from_lightwalletd(&mut client, start_block - 1).await;

    // the end frontier should be the witness of the last added commitment
    // this is used to check the sync matches the network
    let end_frontier = fetch_orchard_frontier_at_height(&mut client, end_block)
        .await
        .unwrap();

    let s = block_contents_batch_stream(
        client,
        pool,
        start_block,
        end_block,
        block_batch_size,
        u32::MAX,
    );
    pin_mut!(s);

    let mut orchard_witnesses_tracked = 0;
    let mut sapling_witnesses_tracked = 0;

    while let Some((actions, outputs)) = s.next().await {
        let (added_orchard, added_sapling) = (actions.len(), outputs.len());

        // Not the most readable code but what this is saying is to mark the first n_witness actions/outputs to maintain witnesses for
        batch_insert_from_orchard_actions(
            &mut orchard_tree,
            orchard_cursor,
            actions.into_iter().map(|(domain, action)| {
                (
                    domain,
                    action,
                    if orchard_witnesses_tracked < n_witnesses {
                        orchard_witnesses_tracked += 1;
                        Retention::Marked
                    } else {
                        Retention::Ephemeral
                    },
                )
            }),
        );
        batch_insert_from_sapling_outputs(
            &mut sapling_tree,
            sapling_cursor,
            outputs.into_iter().map(|(domain, output)| {
                (
                    domain,
                    output,
                    if sapling_witnesses_tracked < n_witnesses {
                        sapling_witnesses_tracked += 1;
                        Retention::Marked
                    } else {
                        Retention::Ephemeral
                    },
                )
            }),
        );

        orchard_cursor += added_orchard as u64;
        sapling_cursor += added_sapling as u64;
    }

    if orchard_witnesses_tracked > 0 {
        assert_eq!(
            end_frontier.root(),
            orchard_tree.root_at_checkpoint_depth(0).unwrap()
        );
        console_log!(
            "✅ Computed orchard root for block {} matches lightwalletd ✅",
            end_block
        );
    }

    (Into::<u64>::into(orchard_cursor) + Into::<u64>::into(sapling_cursor)) as f64
}

async fn bootstrap_orchard_tree_from_lightwalletd(
    client: &mut WasmGrpcClient,
    height: u32,
) -> (OrchardCommitmentTree, Position) {
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);

    // fetch frontier at the end of the previous block
    let init_frontier = fetch_orchard_frontier_at_height(client, height)
        .await
        .unwrap();

    if let Some(frontier) = init_frontier.take() {
        console_log!("Frontier was found for height {}: {:?}", height, frontier);
        let start_position = frontier.position() + 1;
        tree.insert_frontier_nodes(
            frontier,
            Retention::Checkpoint {
                id: height.into(),
                is_marked: false,
            },
        )
        .unwrap();
        (tree, start_position)
    } else {
        panic!("No frontier found for height {}", height);
    }
}

async fn bootstrap_sapling_tree_from_lightwalletd(
    client: &mut WasmGrpcClient,
    height: u32,
) -> (SaplingCommitmentTree, Position) {
    let mut tree = SaplingCommitmentTree::new(SaplingMemoryShardStore::empty(), MAX_CHECKPOINTS);

    // fetch frontier at the end of the previous block
    let init_frontier = fetch_sapling_frontier_at_height(client, height)
        .await
        .unwrap();

    if let Some(frontier) = init_frontier.take() {
        console_log!("Frontier was found for height {}: {:?}", height, frontier);
        let start_position = frontier.position() + 1;
        tree.insert_frontier_nodes(
            frontier,
            Retention::Checkpoint {
                id: height.into(),
                is_marked: false,
            },
        )
        .unwrap();
        (tree, start_position)
    } else {
        panic!("No frontier found for height {}", height);
    }
}

async fn fetch_orchard_frontier_at_height(
    client: &mut WasmGrpcClient,
    height: u32,
) -> anyhow::Result<OrchardFrontier> {
    let start = zcash_client_backend::proto::service::BlockId {
        height: height as u64,
        hash: vec![],
    };
    let pb_tree_state = client.get_tree_state(start).await?.into_inner();
    let tree_frontier_bytes = hex::decode(pb_tree_state.orchard_tree)?;

    let frontier: OrchardFrontier = read_frontier_v0(Cursor::new(tree_frontier_bytes))?;
    Ok(frontier)
}

async fn fetch_sapling_frontier_at_height(
    client: &mut WasmGrpcClient,
    height: u32,
) -> anyhow::Result<SaplingFrontier> {
    let start = zcash_client_backend::proto::service::BlockId {
        height: height as u64,
        hash: vec![],
    };
    let pb_tree_state = client.get_tree_state(start).await?.into_inner();
    let tree_frontier_bytes = hex::decode(pb_tree_state.sapling_tree)?;

    let frontier: SaplingFrontier = read_frontier_v0(Cursor::new(tree_frontier_bytes))?;
    Ok(frontier)
}

/// Insert all notes from a batch of transactions into an in-memory commitment tree starting from a given position
fn batch_insert_from_orchard_actions(
    tree: &mut OrchardCommitmentTree,
    start_position: Position,
    actions: impl Iterator<Item = (OrchardDomain, CompactAction, Retention<BlockHeight>)>,
) {
    let commitments = actions
        .map(|(_, action, retention)| (MerkleHashOrchard::from_cmx(&action.cmx()), retention))
        .collect::<Vec<_>>();

    parallel_batch_add_commitments(tree, start_position, &commitments);
}

/// Insert all notes from a batch of transactions into an in-memory commitment tree starting from a given position
fn batch_insert_from_sapling_outputs(
    tree: &mut SaplingCommitmentTree,
    start_position: Position,
    outputs: impl Iterator<
        Item = (
            SaplingDomain,
            CompactOutputDescription,
            Retention<BlockHeight>,
        ),
    >,
) {
    let commitments = outputs
        .map(|(_, output, retention)| (sapling::Node::from_cmu(&output.cmu), retention))
        .collect::<Vec<_>>();

    parallel_batch_add_commitments(tree, start_position, &commitments);
}

/// Use rayon to parallelize adding batch of commitments to the tree by building the shards
/// in parallel then adding them in after
/// based on the code here (https://github.com/zcash/librustzcash/blob/b3d06ba41904965f3b8165011e14e1d13b3c7b81/zcash_client_sqlite/src/lib.rs#L730)
fn parallel_batch_add_commitments<S, H, const DEPTH: u8, const SHARD_HEIGHT: u8>(
    tree: &mut ShardTree<S, DEPTH, SHARD_HEIGHT>,
    start_position: Position,
    commitments: &[(S::H, Retention<BlockHeight>)],
) where
    S: ShardStore<CheckpointId = BlockHeight, H = H>,
    H: Hashable + Send + Sync + Clone + PartialEq + Copy,
{
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
                chunk.iter().map(|(cmx, retention)| (*cmx, *retention)),
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
