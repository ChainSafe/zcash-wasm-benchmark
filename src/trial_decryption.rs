use std::convert::TryInto;
use std::sync::Arc;

use futures_util::{pin_mut, StreamExt};
use orchard::note_encryption::OrchardDomain;
use rand::rngs::OsRng;
use rayon::prelude::*;
use sapling::note_encryption::{SaplingDomain, Zip212Enforcement};
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

use ff::Field;
use orchard::keys::{FullViewingKey, PreparedIncomingViewingKey, Scope, SpendingKey};
use zcash_primitives::consensus::BranchId;
use zcash_primitives::memo::Memo;
use zcash_primitives::transaction::Transaction;
use zcash_primitives::{consensus, constants};

use crate::proto::service::TxFilter;
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
        let uvk = zcash_keys::keys::UnifiedFullViewingKey::decode(
            &consensus::MAIN_NETWORK,
            &unified_view_key,
        )
        .and_then(|k| {
            Ok(zcash_keys::keys::UnifiedFullViewingKey::to_unified_incoming_viewing_key(&k))
        })
        .and_then(|k| {
            Ok(match (k.orchard(), k.sapling()) {
                (None, None) => (vec![], vec![]),
                (None, Some(s)) => (vec![], vec![s.prepare()]),
                (Some(o), None) => (
                    vec![orchard::keys::PreparedIncomingViewingKey::new(&o)],
                    vec![],
                ),
                (Some(o), Some(s)) => (
                    vec![orchard::keys::PreparedIncomingViewingKey::new(&o)],
                    vec![s.prepare()],
                ),
            })
        });
        // console_log!("Key Provided! Unified View Key: {:?}", unified_view_key);
        match uvk {
            Ok((o, s)) => {
                console_log!("Successfully decoded Unified View Key: {:?}", (&o, &s));
                (o, s)
            }
            Err(e) => {
                console_log!("Provided invalid Unified View Key: {}", e);
                console_log!("Try to decode as Sapling Extended Full View Key");
                let s = zcash_keys::encoding::decode_extended_full_viewing_key(
                    constants::mainnet::HRP_SAPLING_EXTENDED_FULL_VIEWING_KEY,
                    &unified_view_key,
                );
                match s {
                    Ok(s) => {
                        console_log!("Decoded as Sapling Extended Full View Key: {:?}", s);
                        (
                            vec![],
                            vec![s
                                .to_diversifiable_full_viewing_key()
                                .to_external_ivk()
                                .prepare()],
                        )
                    }
                    Err(e) => {
                        console_log!("Provided invalid Sapling Extended Full View Key: {}", e);
                        console_log!("Using dummy keys");
                        (
                            crate::trial_decryption::dummy_ivk_orchard(1),
                            crate::trial_decryption::dummy_ivk_sapling(1),
                        )
                    }
                }
            }
        }
    } else {
        console_log!("No Key Provided. Using dummy keys");
        (
            crate::trial_decryption::dummy_ivk_orchard(1),
            crate::trial_decryption::dummy_ivk_sapling(1),
        )
    };
    console_log!(
        "Trial decryption with IVKs: Orchard: {:?}, Sapling: {:?}",
        ivks_orchard,
        ivks_sapling
    );

    let BenchParams {
        network: _,
        pool,
        lightwalletd_url,
        start_block,
        end_block,
        block_batch_size,
    } = params;
    let mut client = WasmGrpcClient::new(Client::new(lightwalletd_url.clone()));

    let (total_actions, total_outputs) = trial_decrypt_range(
        &mut client,
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
    client: &mut WasmGrpcClient,
    pool: ShieldedPool,
    start_height: u32,
    end_height: u32,
    batch_size: u32,
    spam_filter_limit: u32,
    ivks_orchard: Vec<orchard::keys::PreparedIncomingViewingKey>,
    ivks_sapling: Vec<sapling::keys::PreparedIncomingViewingKey>,
) -> (u32, u32) {
    let mut client_clone = client.clone();
    let s = block_contents_batch_stream(
        &mut client_clone,
        pool,
        start_height,
        end_height,
        batch_size,
        spam_filter_limit,
    );
    pin_mut!(s);
    let (mut total_actions, mut total_outputs) = (0, 0);
    let ivks_orchard = Arc::new(ivks_orchard);
    let ivks_sapling = Arc::new(ivks_sapling.clone());
    while let Some((actions, outputs)) = s.next().await {
        total_actions += actions.len() as u32;
        total_outputs += outputs.len() as u32;

        let (tx_orchard, rx_orchard) = futures_channel::oneshot::channel();
        let (tx_sapling, rx_sapling) = futures_channel::oneshot::channel();
        let (actions, txid_actions): (Vec<(_, _)>, Vec<Vec<u8>>) = actions
            .into_iter()
            .map(|(d, a, txid)| ((d, a), txid))
            .unzip();
        let (outputs, txid_outputs): (Vec<(_, _)>, Vec<Vec<u8>>) = outputs
            .into_iter()
            .map(|(d, a, txid)| ((d, a), txid))
            .unzip();

        let ivks_orchard_c = ivks_orchard.clone();
        let ivks_sapling_c = ivks_sapling.clone();
        rayon::scope(|s| {
            s.spawn(|_| {
                let res_o =
                    batch_decrypt_compact(ivks_orchard_c.as_slice(), &actions, txid_actions);
                let res_s =
                    batch_decrypt_compact(ivks_sapling_c.as_slice(), &outputs, txid_outputs);
                drop(actions);
                drop(outputs);
                tx_orchard.send(res_o).unwrap();
                tx_sapling.send(res_s).unwrap();
            })
        });

        console_debug!("Awaiting Orchard decryption completion");
        let decryped_orchard = rx_orchard.await.unwrap();
        console_debug!("Awaiting Sapling decryption completion");
        let decrypted_sapling = rx_sapling.await.unwrap();
        console_debug!("Batch decrtyp completed");
        for ((_, _), tx_id) in decryped_orchard {
            let tx_filter = TxFilter {
                block: None,
                index: 0,
                hash: tx_id.clone(),
            };
            let tx = client
                .get_transaction(tx_filter)
                .await
                .unwrap()
                .into_inner();
            let tx = Transaction::read(
                &tx.data[..],
                BranchId::for_height(&consensus::MAIN_NETWORK, tx.height.try_into().unwrap()),
            )
            .unwrap();
            let orchard_full_actions = tx
                .orchard_bundle()
                .unwrap() // can unwrap here because we know there are orchard outputs in this tx
                .actions()
                .into_iter()
                .cloned()
                .map(|a| (OrchardDomain::for_action(&a), a))
                .collect::<Vec<_>>();

            let decryped_actions = batch::try_note_decryption(
                ivks_orchard.as_slice(),
                orchard_full_actions.as_slice(),
            )
            .into_iter()
            .filter_map(std::convert::identity)
            .collect::<Vec<_>>();

            for a in decryped_actions.iter() {
                console_log!(
                    "Decrypted Orchard Memo Tx Id: {:?}\nMemo: {:?}",
                    hex::encode(&tx_id),
                    Memo::from_bytes((a.0 .2).as_slice()).unwrap()
                );
            }
        }

        for ((_, _), tx_id) in decrypted_sapling {
            let tx_filter = TxFilter {
                block: None,
                index: 0,
                hash: tx_id.clone(),
            };
            let tx = client
                .get_transaction(tx_filter)
                .await
                .unwrap()
                .into_inner();
            let tx = Transaction::read(
                &tx.data[..],
                BranchId::for_height(&consensus::MAIN_NETWORK, tx.height.try_into().unwrap()),
            )
            .unwrap();
            let sapling_full_outputs = tx
                .sapling_bundle()
                .unwrap()
                .shielded_outputs()
                .into_iter()
                .cloned()
                .map(|o| (SaplingDomain::new(Zip212Enforcement::On), o))
                .collect::<Vec<_>>();
            let decryped_outputs = batch::try_note_decryption(
                ivks_sapling.as_slice(),
                sapling_full_outputs.as_slice(),
            )
            .into_iter()
            .filter_map(std::convert::identity)
            .collect::<Vec<_>>();
            for o in decryped_outputs.iter() {
                console_log!(
                    "Decrypted Sapling Memo Tx Id: {:?}\nMemo: {:?}",
                    hex::encode(&tx_id),
                    Memo::from_bytes((o.0 .2).as_slice()).unwrap()
                );
            }
        }
    }

    console_log!("Decryption complete");
    (total_actions, total_outputs)
}

pub(crate) fn batch_decrypt_compact<D: BatchDomain, Output: ShieldedOutput<D, COMPACT_NOTE_SIZE>>(
    ivks: &[D::IncomingViewingKey],
    compact: &[(D, Output)],
    txid: Vec<Vec<u8>>,
) -> Vec<(
    ((<D as Domain>::Note, <D as Domain>::Recipient), usize),
    Vec<u8>,
)>
where
    (D, Output): Sync + Send,
    <D as Domain>::Note: Send + std::fmt::Debug,
    <D as Domain>::Recipient: Send + std::fmt::Debug,
    <D as Domain>::IncomingViewingKey: Sync + std::fmt::Debug,
{
    if compact.is_empty() {
        console_debug!("No outputs to decrypt");
        return vec![];
    }
    let num_parallel = rayon::current_num_threads();

    let valid_results = compact
        .par_chunks(usize::div_ceil(compact.len(), num_parallel))
        .map(|c| batch::try_compact_note_decryption(ivks, c))
        .zip(
            txid.into_par_iter()
                .chunks(usize::div_ceil(compact.len(), num_parallel)),
        )
        .flatten()
        .filter_map(|(r, txid)| match r {
            Some(r) => Some((r, txid)),
            None => None,
        })
        .collect::<Vec<_>>();

    if valid_results.is_empty() {
        console_debug!("No notes for this address");
    } else {
        console_log!("Decrypted {:?} notes", valid_results.len());
    }
    valid_results
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
