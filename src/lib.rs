use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Cursor;

use commitment_tree::OrchardMemoryShardStore;
use incrementalmerkletree::Position;
use incrementalmerkletree::Retention;
use orchard::note_encryption::CompactAction;
use orchard::note_encryption::OrchardDomain;
use proto::compact_formats::CompactBlock;
use proto::service::compact_tx_streamer_client::CompactTxStreamerClient;
use proto::service::ChainSpec;
use sapling::note_encryption::SaplingDomain;
use sapling::note_encryption::Zip212Enforcement;
use tonic::Streaming;
use trial_decryption::decrypt_compact;
use wasm_bindgen::prelude::*;
use zcash_primitives::merkle_tree::read_frontier_v0;

use crate::commitment_tree::{OrchardCommitmentTree, OrchardFrontier};
use futures_util::stream::TryStreamExt;

mod commitment_tree;
mod proof_gen;
mod trial_decryption;
mod types;
use futures_util::stream;
use futures_util::StreamExt;
use tonic_web_wasm_client::Client;
use web_sys::console::{self};
mod proto;
#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

const GRPC_URL: &str = "http://localhost:443";

// max number of checkpoints our tree impl can cache to jump back to
const MAX_CHECKPOINTS: usize = 1;

type WasmGrpcClient = CompactTxStreamerClient<tonic_web_wasm_client::Client>;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "performance")]
    pub static PERFORMANCE: web_sys::Performance;
}

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub async fn sapling_decrypt_wasm(start: u32, end: u32) -> u32 {
    console::log_1(&"Starting Sapling Trial Decryption all in WASM".into());
    let mut client = new_compact_streamer_client(GRPC_URL);

    let ivks = crate::trial_decryption::dummy_ivk_sapling(1);

    let dl_blocks = PERFORMANCE.now();
    let stream = block_range_stream(&mut client, start, end).await;

    let compact = stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .flat_map(|ctx| stream::iter(ctx.outputs))
        .map(sapling::note_encryption::CompactOutputDescription::try_from)
        .map(|x| (SaplingDomain::new(Zip212Enforcement::Off), x.unwrap()))
        .collect::<Vec<_>>()
        .await;
    console::log_1(
        &format!(
            "Download blocks and deserialization: {}ms",
            PERFORMANCE.now() - dl_blocks,
        )
        .into(),
    );
    decrypt_compact(ivks.as_slice(), &compact)
}

#[wasm_bindgen]
pub async fn orchard_decrypt_wasm(start: u32, end: u32) -> u32 {
    console::log_1(&"Starting Orchard Trial Decryption all in WASM".into());
    let mut client = new_compact_streamer_client(GRPC_URL);
    let ivks = crate::trial_decryption::dummy_ivk_orchard(1);

    let dl_block = PERFORMANCE.now();
    let stream = block_range_stream(&mut client, start, end).await;

    let compact = stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .flat_map(|ctx| stream::iter(ctx.actions))
        .map(|x| {
            let action: CompactAction = x.try_into().unwrap();
            let domain = OrchardDomain::for_nullifier(action.nullifier());
            (domain, action)
        })
        .collect::<Vec<_>>()
        .await;
    console::log_1(
        &format!(
            "Download blocks and deserialization: {}ms",
            PERFORMANCE.now() - dl_block
        )
        .into(),
    );
    decrypt_compact(ivks.as_slice(), &compact)
}

const BLOCK_CHUNK_SIZE: usize = 100;

#[wasm_bindgen]
pub async fn orchard_decrypt_continuous(start: u32) {
    let mut client = new_compact_streamer_client(GRPC_URL);
    let ivks = crate::trial_decryption::dummy_ivk_orchard(1);

    let latest_block_id = client
        .get_latest_block(ChainSpec {})
        .await
        .unwrap()
        .into_inner();
    console_log!(
        "Start trial decrypting range block height {} until head {}, total: {}",
        start,
        latest_block_id.height,
        latest_block_id.height - start as u64
    );

    let end = latest_block_id.height;

    let mut chunked_block_stream = block_range_stream(&mut client, start, end as u32)
        .await
        .try_chunks(BLOCK_CHUNK_SIZE);

    while let Ok(Some(blocks)) = chunked_block_stream.try_next().await {
        console_log!(
            "Pulled {} blocks off GRPC stream in range: [{}, {}]",
            blocks.len(),
            blocks.first().unwrap().height,
            blocks.last().unwrap().height,
        );
        let start = PERFORMANCE.now();

        let compact = blocks
            .iter()
            .flat_map(|b| b.vtx.iter())
            .flat_map(|ctx| ctx.actions.iter())
            .map(|x| {
                let action: CompactAction = x.try_into().unwrap();
                let domain = OrchardDomain::for_nullifier(action.nullifier());
                (domain, action)
            })
            .collect::<Vec<_>>();

        decrypt_compact(ivks.as_slice(), &compact);
        console_log!(
            "Processing {} blocks in range: [{}, {}] took: {}ms",
            blocks.len(),
            blocks.first().unwrap().height,
            blocks.last().unwrap().height,
            PERFORMANCE.now() - start
        );
    }
}

/// Retrieve the tree frontier at the given start block height and then process all note commitments
/// included in blocks between start and end.
/// Finally checks to ensure the computed tree frontier matches the expected frontier at the end block height
#[wasm_bindgen]
pub async fn orchard_sync_commitment_tree_demo(start: u32, end: u32) {
    let mut client = new_compact_streamer_client(GRPC_URL);
    let init_frontier = fetch_orchard_frontier_at_height(&mut client, start - 1)
        .await
        .unwrap();

    // create the tree and initialize it to the initial frontier
    // This also gives us the position to start adding to the tree
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);
    let mut start_position = Position::from(0);

    if let Some(frontier) = init_frontier.take() {
        console::log_1(
            &format!(
                "Frontier was found for height {}: {:?}",
                start - 1,
                frontier
            )
            .into(),
        );
        start_position = frontier.position() + 1;
        tree.insert_frontier_nodes(
            frontier,
            Retention::Checkpoint {
                id: (start - 1).into(),
                is_marked: false,
            },
        )
        .unwrap();
    } else {
        // checkpoint the tree at the start
        let _success = tree.checkpoint(0.into()).unwrap();
    }

    console::log_1(
        &format!(
            "orchard commitment tree starting from position: {:?}",
            start_position
        )
        .into(),
    );

    let stream = block_range_stream(&mut client, start, end).await;

    let actions = stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .flat_map(|ctx| stream::iter(ctx.actions))
        .map(|x| {
            let action: CompactAction = x.try_into().unwrap();
            action
        })
        .collect::<Vec<_>>()
        .await;

    console::log_1(&format!("Downloaded and deserialized {} actions", actions.len()).into());

    let update_tree = PERFORMANCE.now();
    commitment_tree::batch_insert_from_actions(&mut tree, start_position, actions);
    console::log_1(
        &format!(
            "Update commitment tree: {}ms",
            PERFORMANCE.now() - update_tree
        )
        .into(),
    );

    // produce a witness for the first added leaf
    let calc_witness = PERFORMANCE.now();
    let _witness = tree.witness_at_checkpoint_depth(start_position, 0).unwrap();
    console::log_1(
        &format!(
            "Produce witness for leftmost leaf: {}ms",
            PERFORMANCE.now() - calc_witness
        )
        .into(),
    );

    // the end frontier should be the witness of the last added commitment
    // this can give us the root of the new tree produced by adding all the commitments
    let end_frontier = fetch_orchard_frontier_at_height(&mut client, end)
        .await
        .unwrap();
    assert_eq!(
        end_frontier.root(),
        tree.root_at_checkpoint_depth(0).unwrap()
    );
    console::log_1(&format!("✅ Computed root for block {} matches lightwalletd ✅", end).into());
}

pub async fn block_range_stream(
    client: &mut WasmGrpcClient,
    start: u32,
    end: u32,
) -> Streaming<CompactBlock> {
    console::log_1(&format!("Block Range: [{}, {}]", start, end).into());
    let start = proto::service::BlockId {
        height: start as u64,
        hash: vec![],
    };
    let end = proto::service::BlockId {
        height: end as u64,
        hash: vec![],
    };
    let range = proto::service::BlockRange {
        start: Some(start),
        end: Some(end),
    };
    client.get_block_range(range).await.unwrap().into_inner()
}

pub async fn fetch_orchard_frontier_at_height(
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

pub fn new_compact_streamer_client(base_url: &str) -> WasmGrpcClient {
    proto::service::compact_tx_streamer_client::CompactTxStreamerClient::new(Client::new(
        base_url.to_string(),
    ))
}

#[wasm_bindgen(start)]
pub fn start() {
    let num_parallel = rayon::current_num_threads();
    console_log!("Rayon available parallelism Num Parallel: {}", num_parallel);
    set_panic_hook();
}
