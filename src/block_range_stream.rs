use crate::proto::compact_formats::CompactBlock;
use crate::proto::service::{BlockId, BlockRange};
use crate::{WasmGrpcClient, PERFORMANCE};
use futures_util::{Stream, TryStreamExt};
use tonic::Streaming;

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

use crate::{console_debug, console_log};
use sapling::{
    keys::SaplingIvk,
    note_encryption::{CompactOutputDescription, SaplingDomain, Zip212Enforcement},
};
use zcash_note_encryption::{batch, BatchDomain, Domain, ShieldedOutput, COMPACT_NOTE_SIZE};

use crate::bench_params::{BenchParams, ShieldedPool};

/// return a stream over a range of blocks.
pub async fn block_range_stream(
    client: &mut WasmGrpcClient,
    start: u32,
    end: u32,
) -> Streaming<CompactBlock> {
    let start = BlockId {
        height: start as u64,
        hash: vec![],
    };
    let end = BlockId {
        height: end as u64,
        hash: vec![],
    };
    let range = BlockRange {
        start: Some(start),
        end: Some(end),
    };
    client.get_block_range(range).await.unwrap().into_inner()
}

/// Return a stream over the contents of blocks, batched into chunks of `batch_size`.
/// The stream will yield a tuple of accumulated (orchard_actions, sapling_outputs) for each batch.
/// The pool parameter determines which contents should be returned (orchard, sapling or both)
pub fn block_contents_batch_stream(
    mut client: WasmGrpcClient,
    pool: ShieldedPool,
    start_height: u32,
    end_height: u32,
    batch_size: u32,
) -> impl Stream<
    Item = (
        Vec<(OrchardDomain, CompactAction)>,
        Vec<(SaplingDomain, CompactOutputDescription)>,
    ),
> {
    async_stream::stream! {
        let overall_start = PERFORMANCE.now();

        let mut blocks_processed = 0;
        let mut actions_processed = 0;
        let mut outputs_processed = 0;

        let mut latest_synced = start_height as u64;

        while latest_synced < end_height as u64 {
            let mut chunked_block_stream =
                block_range_stream(&mut client, latest_synced as u32, end_height)
                    .await
                    .try_chunks(batch_size as usize);
            while let Ok(Some(blocks)) = chunked_block_stream.try_next().await {
                let start = PERFORMANCE.now();
                let blocks_len = blocks.len();
                let range_start = blocks.first().unwrap().height;
                let range_end = blocks.last().unwrap().height;

                let (actions, outputs) = blocks.into_iter().flat_map(|b| b.vtx.into_iter()).fold(
                    (vec![], vec![]),
                    |(mut actions, mut outputs), tx| {
                        let mut act = if pool.sync_orchard() {
                            tx.actions
                                .into_iter()
                                .map(|action| {
                                    let action: CompactAction = action.try_into().unwrap();
                                    let domain = OrchardDomain::for_nullifier(action.nullifier());
                                    (domain, action)
                                })
                                .collect::<Vec<_>>()
                        } else {
                            vec![]
                        };
                        let mut opt = if pool.sync_sapling() {
                            tx.outputs
                                .into_iter()
                                .map(|output| {
                                    let output: CompactOutputDescription = output.try_into().unwrap();
                                    (SaplingDomain::new(Zip212Enforcement::On), output)
                                })
                                .collect::<Vec<_>>()
                        } else {
                            vec![]
                        };
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
                latest_synced = range_end;

                yield (actions, outputs);

                console_log!(
                    "Processed {} blocks in range: [{}, {}] took: {}ms
            Total Orchard Actions Processed: {}
            Total Sapling Outputs Processed: {}
            Total Blocks Processed: {}
            Blocks remaining: {}
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
            console_log!("GRPC Stream Disconnected or Ended, attempting to reconnect");
        }
        console_log!("Block contents stream complete");
    }
}
