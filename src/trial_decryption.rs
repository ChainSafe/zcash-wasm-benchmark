use orchard::{
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
};
use rand::rngs::OsRng;

use std::convert::TryInto;

use crate::{console_debug, console_log, types::*, PERFORMANCE};
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
    let start = PERFORMANCE.now();
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
    console::log_1(&format!("Converting VTX: {}ms", PERFORMANCE.now() - start).into());
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
    let start = PERFORMANCE.now();
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
    console::log_1(&format!("Converting VTX: {}ms", PERFORMANCE.now() - start).into());

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
    let start = PERFORMANCE.now();
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
    console::log_1(&format!("Converting VTX: {}ms", PERFORMANCE.now() - start).into());

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
    if compact.is_empty() {
        console_debug!("No outputs to decrypt");
        return 0;
    }
    let num_parallel = rayon::current_num_threads();

    if let Some(thread_id) = rayon::current_thread_index() {
        console_debug!("Spawning par_iter from thread {:?}", thread_id);
    } else {
        console_debug!("Spawning par_iter from main thread or non-rayon thread");
    }

    let start = PERFORMANCE.now();
    let valid_results = compact
        .par_chunks(usize::div_ceil(compact.len(), num_parallel))
        .map(|c| {
            let start = PERFORMANCE.now();

            let r = batch::try_compact_note_decryption(ivks, c);
            console_debug!(
                "Thread {:?} decrypted {} outputs: {}ms",
                rayon::current_thread_index(),
                c.len(),
                PERFORMANCE.now() - start
            );
            r
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

    console_log!(
        "Decrypted Total {} outputs: {}ms",
        compact.len(),
        PERFORMANCE.now() - start
    );

    if valid_results.is_empty() {
        console_debug!("No notes for this address");
    } else {
        console_log!("Notes: {:?}", valid_results);
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
