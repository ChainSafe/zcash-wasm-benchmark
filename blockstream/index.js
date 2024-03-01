import { LwdClient, buildBlockRange } from "./blockstream.js";

let client = new LwdClient('http://0.0.0.0:443');

let blockStream = client.getBlockRange(buildBlockRange(2419904, 2411000), {});

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
