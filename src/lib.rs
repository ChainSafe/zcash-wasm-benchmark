use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Cursor;

use commitment_tree::OrchardMemoryShardStore;
use incrementalmerkletree::Position;
use incrementalmerkletree::Retention;
use orchard::note_encryption::CompactAction;
use orchard::note_encryption::OrchardDomain;
use proto::compact_formats::CompactBlock;
use sapling::note_encryption::SaplingDomain;
use sapling::note_encryption::Zip212Enforcement;
use tonic::Streaming;
// use trial_decryption::decrypt_compact;
use wasm_bindgen::prelude::*;
use zcash_primitives::merkle_tree::read_frontier_v0;

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
mod bench_params;
mod block_range_stream;

const GRPC_URL: &str = "http://localhost:443";

pub type WasmGrpcClient =
    crate::proto::service::compact_tx_streamer_client::CompactTxStreamerClient<
        tonic_web_wasm_client::Client,
    >;

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

// pub async fn block_range_stream(base_url: &str, start: u32, end: u32) -> Streaming<CompactBlock> {
//     let mut s = proto::service::compact_tx_streamer_client::CompactTxStreamerClient::new(
//         Client::new(base_url.to_string()),
//     );
//     console::log_1(&format!("Block Range: [{}, {}]", start, end).into());
//     let start = proto::service::BlockId {
//         height: start as u64,
//         hash: vec![],
//     };
//     let end = proto::service::BlockId {
//         height: end as u64,
//         hash: vec![],
//     };
//     let range = proto::service::BlockRange {
//         start: Some(start),
//         end: Some(end),
//     };
//     s.get_block_range(range).await.unwrap().into_inner()
// }

// #[wasm_bindgen(start)]
// pub fn start() {
//     set_panic_hook();
// }
