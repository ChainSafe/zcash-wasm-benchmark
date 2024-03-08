use orchard::{
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
};
use rand::rngs::OsRng;

use std::convert::TryInto;

use crate::types::*;
use ff::Field;
use rayon::prelude::*;
use sapling::{
    keys::SaplingIvk,
    note_encryption::{CompactOutputDescription, SaplingDomain, Zip212Enforcement},
};
use wasm_bindgen::prelude::*;
use web_sys::console;
use zcash_note_encryption::{batch, BatchDomain, Domain, ShieldedOutput, COMPACT_NOTE_SIZE};

#[wasm_bindgen]
pub fn decrypt_vtx_orchard(vtxs: Box<[CompactTx]>) -> u32 {
    console::time_with_label("Converting VTX");
    let compact = vtxs
        .iter()
        .flat_map(|tx| {
            tx.actions
                .iter()
                .map(|action| {
                    let action: CompactAction = action.try_into().unwrap();
                    let domain = OrchardDomain::for_nullifier(action.nullifier());
                    (domain, action)
                })
                .collect::<Vec<_>>()
        })
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
        .iter()
        .flat_map(|tx| {
            tx.outputs
                .iter()
                .map(|output| {
                    let output: CompactOutputDescription = output.try_into().unwrap();
                    (SaplingDomain::new(Zip212Enforcement::Off), output)
                })
                .collect::<Vec<_>>()
        })
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
        vtxs.iter()
            .fold((vec![], vec![]), |(mut actions, mut outputs), tx| {
                let mut act = tx
                    .actions
                    .iter()
                    .map(|action| {
                        let action: CompactAction = action.try_into().unwrap();
                        let domain = OrchardDomain::for_nullifier(action.nullifier());
                        (domain, action)
                    })
                    .collect::<Vec<_>>();
                let mut opt = tx
                    .outputs
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

pub(crate) fn decrypt_compact<D: BatchDomain, Output: ShieldedOutput<D, COMPACT_NOTE_SIZE>>(
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
            let r = batch::try_compact_note_decryption(ivks, c);
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
        console::log_1(&"No notes for this address".to_string().into());
    } else {
        console::log_1(&format!("Notes: {:?}", valid_results).into());
    }
    compact.len() as u32
}

pub(crate) fn dummy_ivk_sapling(
    count: usize,
) -> Vec<sapling::note_encryption::PreparedIncomingViewingKey> {
    let mut rng = OsRng;

    (1..=count)
        .map(|_| SaplingIvk(jubjub::Fr::random(&mut rng)))
        .map(|k| sapling::note_encryption::PreparedIncomingViewingKey::new(&k))
        .collect::<Vec<_>>()
}

pub(crate) fn dummy_ivk_orchard(count: usize) -> Vec<PreparedIncomingViewingKey> {
    (1..=count)
        .map(|i| {
            let fvk = FullViewingKey::from(&SpendingKey::from_bytes([i as u8; 32]).unwrap());
            PreparedIncomingViewingKey::new(&fvk.to_ivk(Scope::External))
        })
        .collect::<Vec<_>>()
}
