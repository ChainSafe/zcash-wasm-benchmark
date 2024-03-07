use wasm_bindgen::prelude::*;

mod commitment_tree;
mod trial_decryption;
mod types;
use futures_util::StreamExt;

use web_sys::{console, ReadableStream};

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

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

// The following code is mostly copy pasta of benchmarks from orchard repo: https://github.com/zcash/orchard/blob/main/benches/

// #[wasm_bindgen(js_namespace = ["proto", "cash", "z","wallet"])]
#[wasm_bindgen(module = "/blockstream/blockstream.js")]
extern "C" {
    #[wasm_bindgen(js_name = LwdClient)]
    type LwdClient;

    type BlockRange;

    #[wasm_bindgen(constructor)]
    fn new(url: &str) -> LwdClient;

    #[wasm_bindgen(method)]
    fn getBlockRange(this: &LwdClient, range: BlockRange, metadata: JsValue) -> ReadableStream;

    fn buildBlockRange(start: u32, end: u32) -> BlockRange;

}

fn _ensure_emitted() {
    // Just ensure that the worker is emitted into the output folder, but don't actually use the URL.
    wasm_bindgen::link_to!(module = "/blockstream/blockstream.js");
}

#[wasm_bindgen]
pub async fn stream() {
    let client = LwdClient::new("http://localhost:443");
    let range = buildBlockRange(1687104 + 10000, 1687104 + 10002);

    let resp = client.getBlockRange(range, JsValue::null());
    console::log_1(&format!("Locked: {}", resp.locked()).into());
    console::log_1(&resp);
    let body = wasm_streams::ReadableStream::from_raw(resp);
    console::log_1(&format!("wasm stream locked {}", body.is_locked()).into());

    // Convert the JS ReadableStream to a Rust stream
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
