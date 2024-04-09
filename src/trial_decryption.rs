use futures_util::{pin_mut, StreamExt};
use rand::rngs::OsRng;
use rayon::prelude::*;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;
use web_sys::console;

use ff::Field;
use orchard::keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey};
use zcash_primitives::consensus;

use crate::{console_debug, console_log};
use sapling::keys::SaplingIvk;
use zcash_note_encryption::{batch, BatchDomain, Domain, ShieldedOutput, COMPACT_NOTE_SIZE};

use crate::bench_params::{BenchParams, ShieldedPool};
use crate::block_range_stream::block_contents_batch_stream;
use crate::WasmGrpcClient;

/// This is the top level function that will be called from the JS side
#[wasm_bindgen]
pub async fn trial_decryption_bench(
    params: BenchParams,
    spam_filter_limit: u32,
    unified_view_key: Option<String>,
) -> f64 {
    console_log!("Starting Trial Decryption with params: {:?}", params);
    let (ivks_orchard, ivks_sapling) = if let Some(unified_view_key) = unified_view_key {
        console_log!("Key Provided! Unified View Key: {:?}", unified_view_key);
        if let Ok((orchard, sapling)) = zcash_keys::keys::UnifiedFullViewingKey::decode(
            &consensus::MAIN_NETWORK,
            &unified_view_key,
        )
        .and_then(|k| {
            Ok(zcash_keys::keys::UnifiedFullViewingKey::to_unified_incoming_viewing_key(&k))
        })
        .and_then(|k| {
            if let (Some(orchard), Some(sapling)) = (k.orchard(), k.sapling()) {
                Ok((
                    vec![orchard::keys::PreparedIncomingViewingKey::new(&orchard)],
                    vec![sapling.prepare()],
                ))
            } else {
                Err("Invalid Unified View Key".to_string())
            }
        }) {
            (orchard, sapling)
        } else {
            console_log!("Invalid Unified View Keys. Using dummy keys");
            (
                crate::trial_decryption::dummy_ivk_orchard(1),
                crate::trial_decryption::dummy_ivk_sapling(1),
            )
        }
    } else {
        console_log!("No Key Provided. Using dummy keys");
        (
            crate::trial_decryption::dummy_ivk_orchard(1),
            crate::trial_decryption::dummy_ivk_sapling(1),
        )
    };

    let BenchParams {
        network: _,
        pool,
        lightwalletd_url,
        start_block,
        end_block,
        block_batch_size,
    } = params;
    let client = WasmGrpcClient::new(Client::new(lightwalletd_url.clone()));

    let (total_actions, total_outputs) = trial_decrypt_range(
        client,
        pool,
        start_block,
        end_block,
        block_batch_size,
        spam_filter_limit,
        ivks_orchard,
        ivks_sapling,
    )
    .await;
    (total_actions + total_outputs) as f64
}

pub async fn trial_decrypt_range(
    client: WasmGrpcClient,
    pool: ShieldedPool,
    start_height: u32,
    end_height: u32,
    batch_size: u32,
    spam_filter_limit: u32,
    ivks_orchard: Vec<orchard::keys::PreparedIncomingViewingKey>,
    ivks_sapling: Vec<sapling::keys::PreparedIncomingViewingKey>,
) -> (u32, u32) {
    let s = block_contents_batch_stream(
        client,
        pool,
        start_height,
        end_height,
        batch_size,
        spam_filter_limit,
    );
    pin_mut!(s);
    let (mut total_actions, mut total_outputs) = (0, 0);
    while let Some((actions, outputs)) = s.next().await {
        total_actions += actions.len() as u32;
        total_outputs += outputs.len() as u32;

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
    (total_actions, total_outputs)
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
        .map(|c| batch::try_compact_note_decryption(ivks, c))
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
