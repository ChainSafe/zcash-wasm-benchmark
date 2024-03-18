use crate::proto::compact_formats::CompactBlock;
use crate::proto::service::{BlockId, BlockRange};
use crate::WasmGrpcClient;
use tonic::Streaming;

/// return a stream over a range of blocks.
/// TODO: this should handle doing multiple requests if the range is too large and gets rejected by the server
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
