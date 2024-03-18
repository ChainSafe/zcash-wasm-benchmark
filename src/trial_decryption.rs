use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use web_sys::console;
use rand::rngs::OsRng;
use rayon::prelude::*;
use tonic::Streaming;
use futures_util::{stream, StreamExt};

use ff::Field;
use zcash_note_encryption::{batch, BatchDomain, Domain, ShieldedOutput, COMPACT_NOTE_SIZE};
use orchard::{
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
};
use sapling::{
    keys::SaplingIvk,
    note_encryption::{CompactOutputDescription, SaplingDomain, Zip212Enforcement},
};

use crate::block_range_stream::block_range_stream;
use crate::proto::compact_formats::CompactBlock;
use crate::{
    bench_params::{BenchParams, ShieldedPool},
    PERFORMANCE,
};

/// This is the top level function that will be called from the JS side
#[wasm_bindgen]
pub async fn trial_decryption_bench(params: BenchParams, view_key: Option<Vec<u8>>) {
    console::log_1(&format!("Starting Trial Decryption with params: {:?}", params).into());

    let BenchParams {
        network,
        pool,
        lightwalletd_url,
        start_block,
        end_block,
    } = params;

    let block_stream = block_range_stream(&lightwalletd_url, start_block, end_block).await;

    match pool {
        ShieldedPool::Sapling => decrypt_vtx_sapling(block_stream).await,
        ShieldedPool::Orchard => decrypt_vtx_orchard(block_stream).await,
        ShieldedPool::Both => decrypt_vtx_both(block_stream).await,
    };
}

async fn decrypt_vtx_orchard(block_stream: Streaming<CompactBlock>) -> u32 {
    let compact = block_stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .flat_map(|ctx| stream::iter(ctx.actions))
        .map(|x| {
            let action: CompactAction = x.try_into().unwrap();
            let domain = OrchardDomain::for_nullifier(action.nullifier());
            (domain, action)
        })
        .collect::<Vec<_>>()
        .await;

    // TODO: Instead of trying to collect the whole stream here (which will blow out memory if not careful)
    // we need to take chunks and pass these to batch_decrypt_compact

    let ivks = dummy_ivk_orchard(1);
    batch_decrypt_compact(&ivks, &compact)
}

async fn decrypt_vtx_sapling(block_stream: Streaming<CompactBlock>) -> u32 {
    let compact = block_stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .flat_map(|ctx| stream::iter(ctx.outputs))
        .map(|output| {
            let output: CompactOutputDescription = output.try_into().unwrap();
            (SaplingDomain::new(Zip212Enforcement::Off), output)
        })
        .collect::<Vec<_>>()
        .await;

    let ivks = dummy_ivk_sapling(1);
    batch_decrypt_compact(ivks.as_slice(), &compact)
}

async fn decrypt_vtx_both(block_stream: Streaming<CompactBlock>) -> u32 {
    let start = PERFORMANCE.now();
    
    let (actions, outputs) = block_stream
        .flat_map(|b| stream::iter(b.unwrap().vtx))
        .fold(
            (vec![], vec![]),
            |(mut actions, mut outputs), tx| async move {
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
            },
        )
        .await;

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

    let s = batch_decrypt_compact(ivks.as_slice(), &outputs);
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
    let o = batch_decrypt_compact(&ivks, &actions);

    s + o
}

pub(crate) fn batch_decrypt_compact<D: BatchDomain, Output: ShieldedOutput<D, COMPACT_NOTE_SIZE>>(
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
    let n_txns = compact.len();
    console::log_1(&format!("Decrypting batch of size {}", compact.len()).into());
    if n_txns == 0 {
        return 0;
    }

    let start = PERFORMANCE.now();
    let results = compact
        .par_chunks(usize::div_ceil(compact.len(), num_parallel))
        .enumerate()
        .map(|(i, c)| {
            let start = PERFORMANCE.now();
            console::log_1(&"Starting decryption".into());
            let r = batch::try_compact_note_decryption(ivks, c);
            console::log_1(
                &format!(
                    "Decrypted chunk {} of size {}: {}ms",
                    i,
                    c.len(),
                    PERFORMANCE.now() - start
                )
                .into(),
            );
            r
        })
        .flatten()
        .collect::<Vec<_>>();

    console::log_1(
        &format!(
            "Decrypted Total {} transactions: {}ms",
            compact.len(),
            PERFORMANCE.now() - start
        )
        .into(),
    );

    let valid_results = results.into_iter().flatten().collect::<Vec<_>>();
    if valid_results.is_empty() {
        console::log_1(&"No notes for this address".to_string().into());
    } else {
        console::log_1(&format!("Notes: {:?}", valid_results).into());
    }
    n_txns as u32
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
