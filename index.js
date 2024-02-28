let num_concurrency = navigator.hardwareConcurrency;
console.log("num_concurrency: ", num_concurrency);

function setupBtn(id, { proof }) {
    // Assign onclick handler + enable the button.
    Object.assign(document.getElementById(id), {
      async onclick() {
        const start = performance.now();
        proof()
        const time = performance.now() - start;
  
        console.log(`${time.toFixed(2)} ms`);
      },
      disabled: false
    });
  }

(async function initSingleThread() {
    const singleThread = await import('./pkg/serial/zcash_wasm_benchmark.js');
    await singleThread.default();
    setupBtn('singleThread', singleThread);
  })();
  
(async function initMultiThread() {
    const multiThread = await import('./pkg/parallel/zcash_wasm_benchmark.js');
    await multiThread.default();

    await multiThread.initThreadPool(num_concurrency);
    setupBtn('multiThread', multiThread);
})();