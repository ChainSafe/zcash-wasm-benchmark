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

use sapling::{
    keys::SaplingIvk,
    note_encryption::{CompactOutputDescription, SaplingDomain, Zip212Enforcement},
};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use web_sys::console;
use zcash_note_encryption::{batch, BatchDomain, Domain, ShieldedOutput, COMPACT_NOTE_SIZE};
pub mod types;
mod utils;
use ff::Field;
use types::*;

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
            let _ = bundle.authorization().proof().verify(&vk, &instances);
            console::time_end_with_label(&format!(
                "Verify Proof with {} recipients",
                num_recipients
            ));
        }
    }
}

#[wasm_bindgen]
pub fn decrypt_vtx_orchard(vtxs: Box<[CompactTx]>) -> u32 {
    console::time_with_label("Converting VTX");
    let compact = vtxs
        .into_iter()
        .map(|tx| {
            tx.actions()
                .iter()
                .map(|action| {
                    let action: CompactAction = action.try_into().unwrap();
                    let domain = OrchardDomain::for_nullifier(action.nullifier());
                    (domain, action)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>();
    console::time_end_with_label("Converting VTX");
    let ivks = dummy_ivk_orchard(1);

    console::log_1(
        &format!(
            "Attempting to batch decrypt {} Orchard txns for {} Viewing keys",
            compact.len(),
            ivks.len()
        )
        .into(),
    );
    decrypt_compact(&ivks, &compact)
}

#[wasm_bindgen]
pub fn decrypt_vtx_sapling(vtxs: Box<[CompactTx]>) -> u32 {
    console::time_with_label("Converting VTX");
    let compact = vtxs
        .into_iter()
        .map(|tx| {
            tx.outputs()
                .iter()
                .map(|output| {
                    let output: CompactOutputDescription = output.try_into().unwrap();
                    (SaplingDomain::new(Zip212Enforcement::Off), output)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Box<[_]>>();
    console::time_end_with_label("Converting VTX");

    let ivks = dummy_ivk_sapling(1);

    console::log_1(
        &format!(
            "Attempting to batch decrypt Sapling {} txns for {} Viewing keys",
            compact.len(),
            ivks.len()
        )
        .into(),
    );

    decrypt_compact(ivks.as_slice(), &compact)
}

#[wasm_bindgen]
pub fn decrypt_vtx_both(vtxs: Box<[CompactTx]>) -> u32 {
    console::time_with_label("Converting VTX");
    let (actions, outputs) =
        vtxs.into_iter()
            .fold((vec![], vec![]), |(mut actions, mut outputs), tx| {
                let mut act = tx
                    .actions()
                    .iter()
                    .map(|action| {
                        let action: CompactAction = action.try_into().unwrap();
                        let domain = OrchardDomain::for_nullifier(action.nullifier());
                        (domain, action)
                    })
                    .collect::<Vec<_>>();
                let mut opt = tx
                    .outputs()
                    .iter()
                    .map(|output| {
                        let output: CompactOutputDescription = output.try_into().unwrap();
                        (SaplingDomain::new(Zip212Enforcement::Off), output)
                    })
                    .collect::<Vec<_>>();
                actions.append(&mut act);
                outputs.append(&mut opt);
                (actions, outputs)
            });
    drop(vtxs);
    console::time_end_with_label("Converting VTX");

    let ivks = dummy_ivk_sapling(1);

    console::log_1(
        &format!(
            "Attempting to batch decrypt Sapling {} txns for {} Viewing keys",
            outputs.len(),
            ivks.len()
        )
        .into(),
    );

    let s = decrypt_compact(ivks.as_slice(), &outputs);
    drop(outputs);

    let ivks = dummy_ivk_orchard(1);

    console::log_1(
        &format!(
            "Attempting to batch decrypt {} Orchard txns for {} Viewing keys",
            actions.len(),
            ivks.len()
        )
        .into(),
    );
    let o = decrypt_compact(&ivks, &actions);

    s + o
}

fn decrypt_compact<D: BatchDomain, Output: ShieldedOutput<D, COMPACT_NOTE_SIZE>>(
    ivks: &[D::IncomingViewingKey],
    compact: &[(D, Output)],
) -> u32
where
    (D, Output): Sync + Send,
    <D as Domain>::Note: Send + std::fmt::Debug,
    <D as Domain>::Recipient: Send + std::fmt::Debug,
    <D as Domain>::IncomingViewingKey: Sync + std::fmt::Debug,
{
    let num_parallel = rayon::current_num_threads();
    console::log_1(&format!("Rayon available parallelism Num Parallel: {}", num_parallel).into());

    console::time_with_label(&format!("Decrypted Total {} transactions", compact.len()));
    let results = compact
        .par_chunks(usize::div_ceil(compact.len(), num_parallel))
        .enumerate()
        .map(|(i, c)| {
            console::time_with_label(&format!(
                "Decrypted chunk {} of {} transactions",
                i,
                c.len()
            ));
            let r = batch::try_compact_note_decryption(&ivks, c);
            console::time_end_with_label(&format!(
                "Decrypted chunk {} of {} transactions",
                i,
                c.len()
            ));
            r
        })
        .flatten()
        .collect::<Vec<_>>();

    console::time_end_with_label(&format!("Decrypted Total {} transactions", compact.len()));

    let valid_results = results.into_iter().flatten().collect::<Vec<_>>();
    if valid_results.is_empty() {
        console::log_1(&format!("No notes for this address").into());
    } else {
        console::log_1(&format!("Notes: {:?}", valid_results).into());
    }
    compact.len() as u32
}

fn dummy_ivk_sapling(count: usize) -> Vec<sapling::note_encryption::PreparedIncomingViewingKey> {
    let mut rng = OsRng;

    (1..=count)
        .map(|_| SaplingIvk(jubjub::Fr::random(&mut rng)))
        .map(|k| sapling::note_encryption::PreparedIncomingViewingKey::new(&k))
        .collect::<Vec<_>>()
}

fn dummy_ivk_orchard(count: usize) -> Vec<PreparedIncomingViewingKey> {
    (1..=count)
        .map(|i| {
            let fvk = FullViewingKey::from(&SpendingKey::from_bytes([i as u8; 32]).unwrap());
            PreparedIncomingViewingKey::new(&fvk.to_ivk(Scope::External))
        })
        .collect::<Vec<_>>()
}

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();
}
