import { LwdClient, buildBlockRange } from "./blockstream.js";
import { CompactBlock } from './generated/compact_formats_pb.ts';

let num_concurrency = navigator.hardwareConcurrency;
console.log("num_concurrency: ", num_concurrency);

const ORCHARD_ACTIVATION = 1687104;
const START = ORCHARD_ACTIVATION;
const END = ORCHARD_ACTIVATION+10000;

function setupBtnDownload(id, { decrypt_all_notes }) {
  // Assign onclick handler + enable the button.
  Object.assign(document.getElementById(id), {
    async onclick() {
      let blocksProcessed = 0;
      let notesProcessed = 0;
      let start = performance.now();

      let client = new LwdClient("http://0.0.0.0:443", null, null);

      let blockStream = client.getBlockRange(
        buildBlockRange(START, END),
        {},
      );

      blockStream.on("data", function (response: CompactBlock) {
        // console.log(response.toObject());
        blocksProcessed++;
        // console.log("blocksProcessed: ", blocksProcessed);
        notesProcessed += decrypt_all_notes(response.serializeBinary());
      });

      blockStream.on("status", function (status) {
        console.log("status code: ", status.code);
        console.log("details: ", status.details);
        console.log("metadata: ", status.metadata);
      });

      blockStream.on("end", function (end) {
        console.log("stream ended");
        console.log("notesProcessed: ", notesProcessed);
        console.log("blocksProcessed: ", blocksProcessed);
        console.log("time: ", performance.now() - start);
      });
    },
    disabled: false,
  });
}

function setupBtn(id, { proof, what }) {
  // Assign onclick handler + enable the button.
  Object.assign(document.getElementById(id), {
    async onclick() {
      const start = performance.now();
      proof();
      const time = performance.now() - start;

      console.log(`${time.toFixed(2)} ms`);
    },
    disabled: false,
  });
}

(async function initSingleThread() {
  const singleThread = await import(
    "./wasm-pkg/serial/zcash_wasm_benchmark.js"
  );
  await singleThread.default();
  setupBtn("singleThread", singleThread);
})();

(async function initMultiThread() {
  const multiThread = await import(
    "./wasm-pkg/parallel/zcash_wasm_benchmark.js"
  );
  await multiThread.default();

  await multiThread.initThreadPool(num_concurrency);
  setupBtn("multiThread", multiThread);
  setupBtnDownload("passval", multiThread);
})();
