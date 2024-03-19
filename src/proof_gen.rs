use orchard::{
    builder::{Builder, BundleType},
    circuit::ProvingKey,
    keys::{FullViewingKey, Scope, SpendingKey},
    value::NoteValue,
    Anchor, Bundle,
};
use rand::rngs::OsRng;

use crate::bench_params::{BenchParams, ShieldedPool};
use wasm_bindgen::prelude::*;
use web_sys::console;

// The following code is mostly copy pasta of benchmarks from orchard repo: https://github.com/zcash/orchard/blob/main/benches/

#[wasm_bindgen]
pub fn generate_proof_bench(params: BenchParams, n_spends: u32) {
    if params.pool != ShieldedPool::Orchard {
        console::log_1(&"This benchmark is only for Orchard".into());
        return;
    }

    let rng = OsRng;
    console::log_1(&"Starting Proof".into());

    console::time_with_label("Spending Key from Bytes");
    let sk = SpendingKey::from_bytes([7; 32]).unwrap();
    console::time_end_with_label("Spending Key from Bytes");

    console::time_with_label("Recipient Viewing Key");
    let recipient = FullViewingKey::from(&sk).address_at(0u32, Scope::External);
    console::time_end_with_label("Recipient Viewing Key");

    console::time_with_label("Create Proving Key");
    let pk = ProvingKey::build();
    console::time_end_with_label("Create Proving Key");

    let create_bundle = |num_recipients| {
        let mut builder = Builder::new(BundleType::DEFAULT, Anchor::from_bytes([0; 32]).unwrap());
        for _ in 0..num_recipients {
            builder
                .add_output(None, recipient, NoteValue::from_raw(10), None)
                .unwrap();
        }
        let bundle: Bundle<_, i64> = builder.build(rng).unwrap().unwrap().0;

        let instances: Vec<_> = bundle
            .actions()
            .iter()
            .map(|a| a.to_instance(*bundle.flags(), *bundle.anchor()))
            .collect();
        (bundle, instances)
    };

    let (bundle, instances) = create_bundle(n_spends);
    console::time_with_label(&format!("Proving with {} spends", n_spends));
    bundle
        .authorization()
        .create_proof(&pk, &instances, rng)
        .unwrap();
    console::time_end_with_label(&format!("Proving with {} spends", n_spends));
}
