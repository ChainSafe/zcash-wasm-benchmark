use orchard::{
    builder::{Builder, BundleType},
    circuit::{ProvingKey, VerifyingKey},
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
    value::NoteValue,
    Anchor, Bundle,
};
use rand::rngs::OsRng;

use std::convert::TryInto;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use protobuf::Message;
use web_sys::console;
use zcash_note_encryption::{batch, try_compact_note_decryption, try_note_decryption};
use codegen::compact_formats as pb;

mod utils;
mod codegen;
mod conversions;

#[cfg(feature = "parallel")]
pub use wasm_bindgen_rayon::init_thread_pool;

use rayon::prelude::*;

// The following code is mostly copy pasta of benchmarks from orchard repo: https://github.com/zcash/orchard/blob/main/benches/

#[wasm_bindgen]
pub fn proof() {
    let rng = OsRng;
    console::log_1(&"Starting Proof".into());

    console::time_with_label("Spending Key from Bytes");
    let sk = SpendingKey::from_bytes([7; 32]).unwrap();
    console::time_end_with_label("Spending Key from Bytes");

    console::time_with_label("Recipient Viewing Key");
    let recipient = FullViewingKey::from(&sk).address_at(0u32, Scope::External);
    console::time_end_with_label("Recipient Viewing Key");

    console::time_with_label("Create Verifying Key");
    let vk = VerifyingKey::build();
    console::time_end_with_label("Create Verifying Key");
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

    let recipients_range = 1..=4;
    // Proving
    {
        for num_recipients in recipients_range.clone() {
            let (bundle, instances) = create_bundle(num_recipients);
            console::time_with_label(&format!("Proving with {} recipients", num_recipients));
            bundle
                .authorization()
                .create_proof(&pk, &instances, rng)
                .unwrap();
            console::time_end_with_label(&format!("Proving with {} recipients", num_recipients));
        }
    }

    // Verifying
    {
        for num_recipients in recipients_range {
            let (bundle, instances) = create_bundle(num_recipients);
            let bundle = bundle
                .create_proof(&pk, rng)
                .unwrap()
                .apply_signatures(rng, [0; 32], &[])
                .unwrap();
            assert!(bundle.verify_proof(&vk).is_ok());
            console::time_with_label(&format!("Verify Proof with {} recipients", num_recipients));
            bundle.authorization().proof().verify(&vk, &instances);
            console::time_end_with_label(&format!(
                "Verify Proof with {} recipients",
                num_recipients
            ));
        }
    }
}

#[wasm_bindgen]
pub fn what() {
    let rng = OsRng;

    console::time_with_label("Create Valid IVK");
    let fvk = FullViewingKey::from(&SpendingKey::from_bytes([7; 32]).unwrap());
    let valid_ivk = fvk.to_ivk(Scope::External);
    let recipient = valid_ivk.address_at(0u32);
    let valid_ivk = PreparedIncomingViewingKey::new(&valid_ivk);
    console::time_end_with_label("Create Valid IVK");

    console::time_with_label("Parallel Create Invalid IVKs");
    let invalid_ivks: Vec<_> = (0u32..10240)
        .into_par_iter()
        // .with_min_len(10240/8)
        .map(|i| {
            let mut sk = [0; 32];
            sk[..4].copy_from_slice(&i.to_le_bytes());
            let fvk = FullViewingKey::from(&SpendingKey::from_bytes(sk).unwrap());
            PreparedIncomingViewingKey::new(&fvk.to_ivk(Scope::External))
        })
        .collect();
    console::time_end_with_label("Parallel Create Invalid IVKs");

    // Takes a long time...
    console::time_with_label("Build PK");
    let pk = ProvingKey::build();
    console::time_end_with_label("Build PK");

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

    console::time_with_label("Decrypt Valid");
    try_compact_note_decryption(&domain, &valid_ivk, &compact).unwrap();
    console::time_end_with_label("Decrypt Valid");

    let ivks = 2;
    let valid_ivks = vec![valid_ivk; ivks];
    let actions: Vec<_> = (0..100)
        .map(|_| (OrchardDomain::for_action(action), action.clone()))
        .collect();
    let compact: Vec<_> = (0..100)
        .map(|_| {
            (
                OrchardDomain::for_action(action),
                CompactAction::from(action),
            )
        })
        .collect();

    for size in [10, 50, 100] {
        console::time_with_label(&format!("Decrypt Valid {}", size));
        batch::try_compact_note_decryption(&valid_ivks, &compact[..size]);

        // group.bench_function(BenchmarkId::new("compact-invalid", size), |b| {
        //     b.iter(|| {
        //         batch::try_compact_note_decryption(&invalid_ivks[..ivks], &compact[..size])
        //     })
        // });
        console::time_end_with_label(&format!("Decrypt Valid {}", size));
    }
}

#[wasm_bindgen]
pub fn b(block_ser: Vec<u8>) {
    let block =  pb::CompactBlock::parse_from_bytes(&block_ser).unwrap();

    console::log_1(&format!("height {:?}", block.height).into());
    console::log_1(&format!("{:?}", block).into());
}

#[wasm_bindgen]
/// Generate a random view key and trial-decrypts all notes in a given block
/// Each trial decryption is timed and logged to the console
/// Returns the total number of notes in the block
pub fn decrypt_all_notes(block_bytes: Vec<u8>) -> u32 {
    let block =  pb::CompactBlock::parse_from_bytes(&block_bytes).unwrap();

    let fvk = FullViewingKey::from(&SpendingKey::from_bytes([7; 32]).unwrap());
    let ivk = PreparedIncomingViewingKey::new(&fvk.to_ivk(Scope::External));

    let mut note_count = 0;

    block.vtx.into_iter().for_each(|tx| {
        tx.actions.into_iter().for_each(|pb_action| {
            note_count += 1;

            let compact: CompactAction = pb_action.try_into().unwrap();
            let domain = OrchardDomain::for_nullifier(compact.nullifier());

            console::time_with_label(&format!("Decrypt {:?}", compact)); // this needs to be unique for each note otherwise it gets confused
            let result = try_compact_note_decryption(&domain, &ivk, &compact);
            console::time_end_with_label(&format!("Decrypt {:?}", compact));

            match result {
                None => console::log_1(&"Note not for this address".into()),
                Some((note, _recipient)) => console::log_1(&format!("Note: {:?}", note).into()),
            }
        });
    });
    note_count
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
