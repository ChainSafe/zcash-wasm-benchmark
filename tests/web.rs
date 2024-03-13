//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

pub use wasm_bindgen_rayon::init_thread_pool;

wasm_bindgen_test_configure!(run_in_browser);

use zcash_wasm_benchmark::PERFORMANCE;

const ORCHARD_ACTIVATION: u32 = 1687104;
const START: u32 = ORCHARD_ACTIVATION + 15000;
const END: u32 = START + 5000;

async fn init_threadpool() {
    let _ = JsFuture::from(init_thread_pool(
        web_sys::window()
            .unwrap()
            .navigator()
            .hardware_concurrency() as usize,
    ))
    .await;
}

// #[wasm_bindgen_test]
// async fn test_decrypt_sapling() {
//     let _ = JsFuture::from(init_thread_pool(
//         web_sys::window()
//             .unwrap()
//             .navigator()
//             .hardware_concurrency() as usize,
//     ))
//     .await;

//     let start = PERFORMANCE.now();

//     zcash_wasm_benchmark::sapling_decrypt_wasm(START, END).await;

//     console_log!("Elapsed: {}", PERFORMANCE.now() - start);
// }

#[wasm_bindgen_test]
async fn test_decrypt_orchard() {
    init_threadpool().await;

    let start = PERFORMANCE.now();

    zcash_wasm_benchmark::orchard_decrypt_wasm(START, END).await;

    console_log!("Elapsed: {}", PERFORMANCE.now() - start);
}

#[wasm_bindgen_test]
async fn test_tree_from_frontier() {
    init_threadpool().await;
    let start = PERFORMANCE.now();
    zcash_wasm_benchmark::orchard_sync_commitment_tree_demo(START, END).await;
    console_log!("Elapsed: {}", PERFORMANCE.now() - start);
}
