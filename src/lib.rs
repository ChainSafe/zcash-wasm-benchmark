use wasm_bindgen::prelude::*;

mod commitment_tree;
mod trial_decryption;
mod types;

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

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
