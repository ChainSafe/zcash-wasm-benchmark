import { LwdClient, buildBlockRange } from "./blockstream.js";

let client = new LwdClient("http://0.0.0.0:443");

let blockStream = client.getBlockRange(buildBlockRange(2419904, 2411000), {});

blockStream.on("data", function (response) {
  console.log(response.toObject());
});

blockStream.on("status", function (status) {
  console.log("status code: ", status.code);
  console.log("details: ", status.details);
  console.log("metadata: ", status.metadata);
});

blockStream.on("end", function (end) {
  console.log("stream ended");
});

let num_concurrency = navigator.hardwareConcurrency;
console.log("num_concurrency: ", num_concurrency);

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
})();
