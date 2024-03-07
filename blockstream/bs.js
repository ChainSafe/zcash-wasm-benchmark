/* Re-exports and helpers for creating streams of compact blocks from lightwalletd

Example usage:
```
const client = new LwdClient('http://0.0.0.0:8080'); // must be a grpc-web proxy to lightwalletd
const blockRange = buildBlockRange(2419904, 2419910);
const blockStream = client.getBlockRange(blockRange, {});

blockStream.on('data', function(response) {
  console.log(response.toObject());
});

blockStream.on('status', function(status) {
    console.log("status code: ",status.code);
    console.log("details: ", status.details);
    console.log("metadata: ", status.metadata);
});

blockStream.on('end', function(end) {
  console.log("stream ended")
});
```
*/

import { CompactTxStreamerClient as LwdClient } from "./generated/ServiceServiceClientPb.ts";
import { BlockRange, BlockID } from "./generated/service_pb.js";

/// Accepts a start and end block height as numbers and returns a BlockRange object
/// as defined in the protobuf schema. This can be passed directly to LwdClient.getBlockRange
export function buildBlockRange(startBlockHeight, endBlockHeight) {
  let blockRange = new BlockRange();

  let start = new BlockID();
  start.setHeight(startBlockHeight);

  let end = new BlockID();
  end.setHeight(endBlockHeight);

  blockRange.setStart(start);
  blockRange.setEnd(end);

  return blockRange;
}

export { LwdClient };
