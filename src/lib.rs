use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Cursor;

use commitment_tree::OrchardMemoryShardStore;
use orchard::note_encryption::CompactAction;
use orchard::note_encryption::OrchardDomain;
use proto::compact_formats::CompactBlock;
use sapling::note_encryption::SaplingDomain;
use sapling::note_encryption::Zip212Enforcement;
use incrementalmerkletree::Retention;
use zcash_primitives::merkle_tree::read_frontier_v1;
use tonic::Streaming;
use trial_decryption::decrypt_compact;
use wasm_bindgen::prelude::*;

use crate::commitment_tree::{OrchardCommitmentTree, OrchardFrontier};

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

    let ivks = crate::trial_decryption::dummy_ivk_sapling(1);

    let dl_blocks = PERFORMANCE.now();
    let stream = block_range_stream(GRPC_URL, start, end).await;

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

    let ivks = crate::trial_decryption::dummy_ivk_orchard(1);

    let dl_block = PERFORMANCE.now();
    let stream = block_range_stream(GRPC_URL, start, end).await;

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

/// Retrieve the tree frontier at the given start block height and then process all note commitments
/// included in blocks between start and end.
/// Finally checks to ensure the computed tree frontier matches the expected frontier at the end block height
pub async fn orchard_sync_commitment_tree(start: u32, end: u32) {
    let init_frontier = orchard_frontier_at_height(GRPC_URL, start).await.unwrap();
    let final_frontier = orchard_frontier_at_height(GRPC_URL, end).await.unwrap();

    // create the tree and initialize it to the initial frontier
    let mut tree = OrchardCommitmentTree::new(OrchardMemoryShardStore::empty(), MAX_CHECKPOINTS);
    if let Some(frontier) = init_frontier.take() {
        tree.insert_frontier_nodes(frontier, Retention::Ephemeral).unwrap();
    }

    let stream = block_range_stream(GRPC_URL, start, end).await;
    let actions = stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .flat_map(|ctx| stream::iter(ctx.actions))
        .map(|x| {
            let action: CompactAction = x.try_into().unwrap();
            action
        })
        .collect::<Vec<_>>()
        .await;

    commitment_tree::batch_insert_from_actions(&mut tree, actions, 100);
}

pub async fn block_range_stream(base_url: &str, start: u32, end: u32) -> Streaming<CompactBlock> {
    let mut s = proto::service::compact_tx_streamer_client::CompactTxStreamerClient::new(
        Client::new(base_url.to_string()),
    );
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
    s.get_block_range(range).await.unwrap().into_inner()
}

pub async fn orchard_frontier_at_height(base_url: &str, height: u32) -> anyhow::Result<OrchardFrontier>  {
    let mut s = proto::service::compact_tx_streamer_client::CompactTxStreamerClient::new(
        Client::new(base_url.to_string()),
    );
    let start = proto::service::BlockId {
        height: height as u64,
        hash: vec![],
    };
    let pb_tree_state = s.get_tree_state(start).await?.into_inner();
    let orchard_tree_frontier_bytes = hex::decode(pb_tree_state.orchard_tree)?;
    let frontier: OrchardFrontier = read_frontier_v1(Cursor::new(orchard_tree_frontier_bytes))?;
    Ok(frontier)
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
