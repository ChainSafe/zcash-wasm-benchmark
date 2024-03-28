use wasm_bindgen::prelude::*;

mod commitment_tree;
mod proof_gen;
mod trial_decryption;

use tonic_web_wasm_client::Client;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

mod bench_params;
mod block_range_stream;
pub type WasmGrpcClient =
    zcash_client_backend::proto::service::compact_tx_streamer_client::CompactTxStreamerClient<
        tonic_web_wasm_client::Client,
    >;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}
macro_rules! console_debug {
    ($($t:tt)*) => (web_sys::console::debug_1(&format!($($t)*).into()))
}

pub(crate) use console_debug;
pub(crate) use console_log;

pub use bench_params::*;
pub use commitment_tree::*;
pub use proof_gen::*;
pub use trial_decryption::*;

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

pub fn new_compact_streamer_client(base_url: &str) -> WasmGrpcClient {
    zcash_client_backend::proto::service::compact_tx_streamer_client::CompactTxStreamerClient::new(
        Client::new(base_url.to_string()),
    )
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
