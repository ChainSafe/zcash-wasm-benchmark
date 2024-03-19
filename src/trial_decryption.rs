use futures_util::{pin_mut, StreamExt};
use rand::rngs::OsRng;
use rayon::prelude::*;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;
use web_sys::console;

use ff::Field;
use orchard::keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey};

use crate::{console_debug, console_log};
use sapling::keys::SaplingIvk;
use zcash_note_encryption::{batch, BatchDomain, Domain, ShieldedOutput, COMPACT_NOTE_SIZE};

use crate::bench_params::{BenchParams, ShieldedPool};
use crate::block_range_stream::block_contents_batch_stream;
use crate::WasmGrpcClient;

/// This is the top level function that will be called from the JS side
#[wasm_bindgen]
pub async fn trial_decryption_bench(params: BenchParams, _view_key: Option<Vec<u8>>) {
    console::log_1(&format!("Starting Trial Decryption with params: {:?}", params).into());

    let BenchParams {
        network: _,
        pool,
        lightwalletd_url,
        start_block,
        end_block,
        block_batch_size,
    } = params;
    let client = WasmGrpcClient::new(Client::new(lightwalletd_url.clone()));

    trial_decrypt_range(client, pool, start_block, end_block, block_batch_size).await;
}

pub async fn trial_decrypt_range(
    client: WasmGrpcClient,
    pool: ShieldedPool,
    start_height: u32,
    end_height: u32,
    batch_size: u32,
) {
    let ivks_orchard = crate::trial_decryption::dummy_ivk_orchard(1);
    let ivks_sapling = crate::trial_decryption::dummy_ivk_sapling(1);

    let s = block_contents_batch_stream(client, pool, start_height, end_height, batch_size);
    pin_mut!(s);
    while let Some((actions, outputs)) = s.next().await {
        let ivks_orchard = ivks_orchard.clone();
        let ivks_sapling = ivks_sapling.clone();
        let (tx, rx) = futures_channel::oneshot::channel();
        rayon::scope(|s| {
            s.spawn(|_| {
                batch_decrypt_compact(ivks_orchard.as_slice(), &actions);
                batch_decrypt_compact(ivks_sapling.as_slice(), &outputs);
                drop(actions);
                drop(outputs);
                tx.send(()).unwrap();
            })
        });

        console_debug!("Awaiting decryption completion");
        rx.await.unwrap();
    }

    console_log!("Decryption complete");
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

    let valid_results = compact
        .par_chunks(usize::div_ceil(compact.len(), num_parallel))
        .map(|c| {
            batch::try_compact_note_decryption(ivks, c)
        })
        .flatten()
        .flatten()
        .collect::<Vec<_>>();

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
