mod utils;

use orchard::{
    builder::{Builder, BundleType},
    circuit::ProvingKey,
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
    value::NoteValue,
    Anchor, Bundle,
};
use rand::rngs::OsRng;
use rayon::prelude::*;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;
use web_sys::console;
use zcash_note_encryption::try_note_decryption;

#[wasm_bindgen]
pub fn what() {
    let rng = OsRng;
    // Takes a long time...
    console::time_with_label("Build PK");
    let pk = ProvingKey::build();
    console::time_end_with_label("Build PK");

    console::time_with_label("Create Valid IVK");
    let fvk = FullViewingKey::from(&SpendingKey::from_bytes([7; 32]).unwrap());
    let valid_ivk = fvk.to_ivk(Scope::External);
    let recipient = valid_ivk.address_at(0u32);
    let valid_ivk = PreparedIncomingViewingKey::new(&valid_ivk);
    console::time_end_with_label("Create Valid IVK");
    console::time_with_label("Create Invalid IVKs");
    let invalid_ivks: Vec<_> = (0u32..10240)
        // .map(|i| {
        .into_par_iter()
        .map(|i| {
            let mut sk = [0; 32];
            sk[..4].copy_from_slice(&i.to_le_bytes());
            let fvk = FullViewingKey::from(&SpendingKey::from_bytes(sk).unwrap());
            PreparedIncomingViewingKey::new(&fvk.to_ivk(Scope::External))
        })
        .collect();
    console::time_end_with_label("Create Invalid IVKs");

    console::time_with_label("Create Bundle");
    let bundle = {
        let mut builder = Builder::new(BundleType::DEFAULT, Anchor::from_bytes([0; 32]).unwrap());
        // The builder pads to two actions, and shuffles their order. Add two recipients
        // so the first action is always decryptable.
        builder
            .add_output(None, recipient, NoteValue::from_raw(10), None)
            .unwrap();
        builder
            .add_output(None, recipient, NoteValue::from_raw(10), None)
            .unwrap();
        let bundle: Bundle<_, i64> = builder.build(rng).unwrap().unwrap().0;
        bundle
            .create_proof(&pk, rng)
            .unwrap()
            .apply_signatures(rng, [0; 32], &[])
            .unwrap()
    };
    console::time_end_with_label("Create Bundle");

    console::time_with_label("Compact");
    let action = bundle.actions().first();
    let domain = OrchardDomain::for_action(action);

    let compact = CompactAction::from(action);
    console::time_end_with_label("Compact");

    console::time_with_label("Decrypt");
    try_note_decryption(&domain, &valid_ivk, action).unwrap();
    console::time_end_with_label("Decrypt");
}
