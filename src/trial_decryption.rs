use futures_util::TryStreamExt;
use rand::rngs::OsRng;
use rayon::prelude::*;
use std::convert::TryInto;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;
use web_sys::console;

use ff::Field;
use orchard::{
    keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey},
    note_encryption::{CompactAction, OrchardDomain},
};

use crate::{console_debug, console_log, proto::service::ChainSpec, types::*, PERFORMANCE};
use sapling::{
    keys::SaplingIvk,
    note_encryption::{CompactOutputDescription, SaplingDomain, Zip212Enforcement},
};
use zcash_note_encryption::{batch, BatchDomain, Domain, ShieldedOutput, COMPACT_NOTE_SIZE};

use crate::bench_params::{BenchParams, ShieldedPool};
use crate::block_range_stream::block_range_stream;
use crate::WasmGrpcClient;

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
    let client = WasmGrpcClient::new(Client::new(lightwalletd_url.clone()));

    match pool {
        ShieldedPool::Sapling => unimplemented!(),
        ShieldedPool::Orchard => unimplemented!(),
        ShieldedPool::Both => trial_decrypt_range(client, start_block, end_block),
    }
    .await;
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
            console::log_1(&"Starting decryption".into());
            let r = batch::try_compact_note_decryption(ivks, c);
            console_debug!(
                "Thread {:?} decrypted {} of size: {}ms",
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

const BLOCK_CHUNK_SIZE: usize = 250;

pub async fn trial_decrypt_range(mut client: WasmGrpcClient, start_height: u32, end_height: u32) {
    let ivks_orchard = crate::trial_decryption::dummy_ivk_orchard(1);
    let ivks_sapling = crate::trial_decryption::dummy_ivk_sapling(1);

    // let end_height = latest_block_id.height;
    let overall_start = PERFORMANCE.now();

    let mut blocks_processed = 0;
    let mut actions_processed = 0;
    let mut outputs_processed = 0;

    let mut latest_synced = start_height as u64;

    while latest_synced < end_height as u64 {
        let mut chunked_block_stream =
            block_range_stream(&mut client, latest_synced as u32, end_height as u32)
                .await
                .try_chunks(BLOCK_CHUNK_SIZE);
        while let Ok(Some(blocks)) = chunked_block_stream.try_next().await {
            let start = PERFORMANCE.now();
            let blocks_len = blocks.len();
            let range_start = blocks.first().unwrap().height;
            let range_end = blocks.last().unwrap().height;

            let (actions, outputs) = blocks.into_iter().flat_map(|b| b.vtx.into_iter()).fold(
                (vec![], vec![]),
                |(mut actions, mut outputs), tx| {
                    let mut act = tx
                        .actions
                        .into_iter()
                        .map(|action| {
                            let action: CompactAction = action.try_into().unwrap();
                            let domain = OrchardDomain::for_nullifier(action.nullifier());
                            (domain, action)
                        })
                        .collect::<Vec<_>>();
                    let mut opt = tx
                        .outputs
                        .into_iter()
                        .map(|output| {
                            let output: CompactOutputDescription = output.try_into().unwrap();
                            (SaplingDomain::new(Zip212Enforcement::On), output)
                        })
                        .collect::<Vec<_>>();
                    actions.append(&mut act);
                    outputs.append(&mut opt);
                    (actions, outputs)
                },
            );
            console_log!(
                "Time to convert blocks to actions and outputs: {}ms",
                PERFORMANCE.now() - start
            );
            blocks_processed += blocks_len;
            actions_processed += actions.len();
            outputs_processed += outputs.len();
            let ivks_orchard = ivks_orchard.clone();
            let ivks_sapling = ivks_sapling.clone();
            latest_synced = range_end;
            let (tx, rx) = futures_channel::oneshot::channel();
            rayon::scope(|s| {
                s.spawn(|_| {
                    console_log!("Orchard Trial Decryption");
                    batch_decrypt_compact(ivks_orchard.as_slice(), &actions);
                    console_log!("Sapling Trial Decryption");
                    batch_decrypt_compact(ivks_sapling.as_slice(), &outputs);
                    drop(actions);
                    drop(outputs);
                    tx.send(()).unwrap();
                })
            });

            console_debug!("Awaiting decryption completion");
            rx.await.unwrap();

            console_log!(
                "Processed {} blocks in range: [{}, {}] took: {}ms
        Total Orchard Actions Processed: {}
        Total Sapling Outputs Processed: {}
        Total Blocks Processed: {}
        Blocks until head: {}
        Total Time Elapsed: {}ms",
                blocks_len,
                range_start,
                range_end,
                PERFORMANCE.now() - start,
                actions_processed,
                outputs_processed,
                blocks_processed,
                end_height - start_height - blocks_processed as u32,
                PERFORMANCE.now() - overall_start
            );
        }
        console_log!("GRPC Stream Disconnected, attempting to reconnect");
    }
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
