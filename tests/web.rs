//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use rand::rngs::OsRng;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen_test::*;

use rayon::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

use orchard::{
    builder::{Builder, BundleType},
    circuit::ProvingKey,
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
    value::NoteValue,
    Anchor, Bundle,
};
use zcash_note_encryption::{batch, try_compact_note_decryption, try_note_decryption};

#[wasm_bindgen_test]
fn what() {
    let rng = OsRng;
    // Takes a long time...
    // let pk = ProvingKey::build();

    console::log_1(&"Hello using web-sys".into());
    let fvk = FullViewingKey::from(&SpendingKey::from_bytes([7; 32]).unwrap());
    let valid_ivk = fvk.to_ivk(Scope::External);
    let recipient = valid_ivk.address_at(0u32);
    let valid_ivk = PreparedIncomingViewingKey::new(&valid_ivk);
    // let invalid_ivks: Vec<_> = (0u32..10240)
    let invalid_ivks: Vec<_> = (0u32..100)
        // .map(|i| {
        .into_par_iter()
        .map(|i| {
            let mut sk = [0; 32];
            sk[..4].copy_from_slice(&i.to_le_bytes());
            let fvk = FullViewingKey::from(&SpendingKey::from_bytes(sk).unwrap());
            PreparedIncomingViewingKey::new(&fvk.to_ivk(Scope::External))
        })
        .collect();

    // let bundle = {
    //     let mut builder = Builder::new(BundleType::DEFAULT, Anchor::from_bytes([0; 32]).unwrap());
    //     // The builder pads to two actions, and shuffles their order. Add two recipients
    //     // so the first action is always decryptable.
    //     builder
    //         .add_output(None, recipient, NoteValue::from_raw(10), None)
    //         .unwrap();
    //     builder
    //         .add_output(None, recipient, NoteValue::from_raw(10), None)
    //         .unwrap();
    //     let bundle: Bundle<_, i64> = builder.build(rng).unwrap().unwrap().0;
    //     bundle
    //         .create_proof(&pk, rng)
    //         .unwrap()
    //         .apply_signatures(rng, [0; 32], &[])
    //         .unwrap()
    // };
    // let action = bundle.actions().first();
    // let domain = OrchardDomain::for_action(action);

    // let compact = CompactAction::from(action);
}
