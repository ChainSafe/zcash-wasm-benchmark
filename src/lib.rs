mod utils;

use orchard::{
    builder::{Builder, BundleType},
    circuit::ProvingKey,
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
    value::NoteValue,
    Anchor, Bundle,
};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let pk = ProvingKey::build();
    let fvk = FullViewingKey::from(&SpendingKey::from_bytes([7; 32]).unwrap());
    alert("Hello, zcash-wasm-benchmark!");
}
