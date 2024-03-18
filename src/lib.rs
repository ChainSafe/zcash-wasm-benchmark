use wasm_bindgen::prelude::*;
#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

mod bench_params;
mod benchmarks;
mod block_range_stream;
mod proto;
mod types;

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
