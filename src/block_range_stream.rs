
use tonic::Streaming;
use tonic_web_wasm_client::Client;

use crate::proto::service::{compact_tx_streamer_client::CompactTxStreamerClient, BlockId, BlockRange};
use crate::proto::compact_formats::CompactBlock;

/// return a stream over a range of blocks.
/// TODO: this should handle doing multiple requests if the range is too large and gets rejected by the server
pub async fn block_range_stream(base_url: &str, start: u32, end: u32) -> Streaming<CompactBlock> {
    let mut s = CompactTxStreamerClient::new(
        Client::new(base_url.to_string()),
    );
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
    s.get_block_range(range).await.unwrap().into_inner()
}
