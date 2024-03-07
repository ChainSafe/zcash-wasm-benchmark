import { LwdClient, buildBlockRange } from "./blockstream";
import { CompactBlock as CompactBlockPb } from "./generated/compact_formats_pb";

let num_concurrency = navigator.hardwareConcurrency;
console.log("num_concurrency: ", num_concurrency);

const ORCHARD_ACTIVATION = 1687104;
const START = ORCHARD_ACTIVATION + 10000;
const END = START + 10000;

let blocks: Map<number, CompactBlockPb> = new Map();

function setupBtnDownload(
  id,
  withTransactions, // callback to call with the downloaded transactions extracted from the blocks
  { CompactOrchardAction, CompactSaplingOutput, CompactSaplingSpend, CompactTx } // wasm lib to load constructors from
) {
  // Assign onclick handler + enable the button.
  Object.assign(document.getElementById(id), {
    async onclick() {
      let blocksProcessed = 0;
      let notesProcessed = 0;
      let start = performance.now();

      let client = new LwdClient("http://0.0.0.0:443", null, null);

      let blockStream = client.getBlockRange(buildBlockRange(START, END), {});

      blockStream.on("data", function (response: CompactBlockPb) {
        blocksProcessed++;
        if (blocksProcessed % 2000 === 0) {
          console.log("blocks downloaded: ", blocksProcessed);
        }
        blocks.set(response.getHeight(), response);
      });

      blockStream.on("status", function (status) {
        console.log("status code: ", status.code);
        console.log("details: ", status.details);
        console.log("metadata: ", status.metadata);
      });

      blockStream.on("end", function (end) {
        console.log("Download stream ended after: ", performance.now() - start);

        let start_construction = performance.now();
        let vtx = Array.from(blocks.values())
          .map((block) => block.getVtxList())
          .reduce((accumulator, value) => accumulator.concat(value), [])
          .map((tx) => {
            let actions = tx.getActionsList().map((act) => {
              return new CompactOrchardAction(
                act.getNullifier_asU8(),
                act.getCmx_asU8(),
                act.getEphemeralkey_asU8(),
                act.getCiphertext_asU8(),
              );
            });
            let outputs = tx.getOutputsList().map((out) => {
              return new CompactSaplingOutput(
                out.getCmu_asU8(),
                out.getEphemeralkey_asU8(),
                out.getCiphertext_asU8(),
              );
            });
            let spends = tx.getSpendsList().map((spend) => {
              return new CompactSaplingSpend(spend.getNf_asU8());
            });
            let ret = new CompactTx(
              BigInt(tx.getIndex()),
              tx.getHash_asU8(),
              tx.getFee(),
              spends,
              outputs,
              actions,
            );
            return ret;
          });
        console.log(
          "time to construct wasm passable obj: ",
          performance.now() - start_construction,
        );
        try {
          let result = withTransactions(vtx);
          console.log("result: ", result);
        } catch (e) {
          console.log("ourrr nourrr");
          console.log(e);
        }

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
  setupBtnDownload("trialDecryptOrchard", multiThread.decrypt_vtx_orchard, multiThread);
  setupBtnDownload("trialDecryptSapling", multiThread.decrypt_vtx_sapling, multiThread);
  setupBtnDownload("trialDecryptBoth", multiThread.decrypt_vtx_both, multiThread);
  setupBtnDownload("treeBench", multiThread.batch_insert_txn_notes, multiThread);
})();
