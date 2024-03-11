use std::convert::TryFrom;
use std::convert::TryInto;

use orchard::note_encryption::CompactAction;
use orchard::note_encryption::OrchardDomain;
use proto::compact_formats::CompactBlock;
use sapling::note_encryption::SaplingDomain;
use sapling::note_encryption::Zip212Enforcement;
use tonic::Streaming;
use trial_decryption::decrypt_compact;
use wasm_bindgen::prelude::*;

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
    console::log_1(&format!("Block Range: [{}, {}]", start, end).into());

    let ivks = crate::trial_decryption::dummy_ivk_sapling(1);

    let dl_blocks = PERFORMANCE.now();
    let stream = block_range_stream("http://localhost:433", start, end).await;

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
    console::log_1(&format!("Block Range: [{}, {}]", start, end).into());

    let ivks = crate::trial_decryption::dummy_ivk_orchard(1);

    let dl_block = PERFORMANCE.now();
    let stream = block_range_stream("http://localhost:433", start, end).await;

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

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
