use polars::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_rayon::init_thread_pool;
use wasm_bindgen_test::*;

use web_sys::console;
use zcash_wasm_benchmark::*;

wasm_bindgen_test_configure!(run_in_browser);

const TIP: u32 = 2442739;
const SPAM_FILTER: u32 = 50;
const REPS: usize = 3; // repetitions of each test
const THREADS: usize = 4; // number of threads (webworkers) to use

#[wasm_bindgen_test]
async fn test_decryption() {
    init_threadpool(THREADS).await;

    #[derive(Debug, serde::Serialize)]
    struct TestParams {
        rep: usize,
        batch_size: u32,
        pool: ShieldedPool,
        total_decryptions: f64,
        time: f64,
    }

    fn param_grid() -> impl Iterator<Item = TestParams> {
        let rep = 1..=REPS;
        let batch_size = vec![100, 1000, 10000];
        let pool = vec![
            ShieldedPool::Sapling,
            ShieldedPool::Orchard,
            ShieldedPool::Both,
        ];
        itertools::iproduct!(rep, batch_size, pool).map(|(rep, batch_size, pool)| TestParams {
            rep,
            batch_size,
            pool,
            total_decryptions: 0.0,
            time: 0.0,
        })
    }

    let mut results = Vec::new();

    for test_params in param_grid() {
        let params = BenchParams {
            network: Network::Mainnet,
            pool: test_params.pool.clone(),
            lightwalletd_url: "http://localhost:443".to_string(),
            start_block: TIP - 108000, // 90 days worth of blocks
            end_block: TIP,
            block_batch_size: test_params.batch_size,
        };
        let start = PERFORMANCE.now();
        let total_decryptions =
            zcash_wasm_benchmark::trial_decryption_bench(params, SPAM_FILTER, None).await;
        let time = PERFORMANCE.now() - start;

        let result = TestParams {
            time,
            total_decryptions,
            ..test_params
        };
        results.push(result);
    }

    let json = serde_json::to_string(&results).unwrap();
    let mut df = JsonReader::new(std::io::Cursor::new(json))
        .finish()
        .unwrap();

    let mut buf = Vec::new();
    CsvWriter::new(&mut buf).finish(&mut df).unwrap();
    console_log!("{}", String::from_utf8(buf).unwrap()); // can't write a file from a web test so we just have to write to console
    console_log!("{:?}", df);
}

#[wasm_bindgen_test]
async fn tree_sync() {
    init_threadpool(THREADS).await;

    #[derive(Debug, serde::Serialize)]
    struct TestParams {
        rep: usize,
        batch_size: u32,
        pool: ShieldedPool,
        n_witnesses: u32,
        total_updates: f64,
        time: f64,
    }

    fn param_grid() -> impl Iterator<Item = TestParams> {
        let rep = 1..=REPS;
        let batch_size = vec![100, 1000, 10000];
        let n_witnesses = vec![1,10,100];
        let pool = vec![ShieldedPool::Sapling, ShieldedPool::Orchard, ShieldedPool::Both];

        itertools::iproduct!(rep, batch_size, pool, n_witnesses).map(|(rep, batch_size, pool, n_witnesses)| TestParams { rep, batch_size, pool, n_witnesses, total_updates: 0.0, time: 0.0 })
    }

    let mut results = Vec::new();

    for test_params in param_grid() {
        let params = BenchParams {
            network: Network::Mainnet,
            pool: test_params.pool.clone(),
            lightwalletd_url: "http://localhost:443".to_string(),
            start_block: TIP - 108000, // 90 days worth of blocks
            end_block: TIP,
            block_batch_size: test_params.batch_size,
        };
        let start = PERFORMANCE.now();
        let total_updates = zcash_wasm_benchmark::sync_commitment_tree_bench(params, test_params.n_witnesses).await;
        let elapsed = PERFORMANCE.now() - start;

        let result = TestParams {
            time: elapsed,
            total_updates,
            ..test_params
        };
        results.push(result);
    }

    let json = serde_json::to_string(&results).unwrap();
    let mut df = JsonReader::new(std::io::Cursor::new(json))
        .finish()
        .unwrap();

    let mut buf = Vec::new();
    CsvWriter::new(&mut buf).finish(&mut df).unwrap();
    console_log!("{}", String::from_utf8(buf).unwrap()); // can't write a file from a web test so we just have to write to console
    console_log!("{:?}", df);
}

#[wasm_bindgen_test]
async fn proving() {
    init_threadpool(THREADS).await;

    #[derive(Debug, serde::Serialize)]
    struct TestParams {
        spends: u32,
        time: f64,
    }

    fn param_grid() -> impl Iterator<Item = TestParams> {
        let spends = vec![1, 5, 10, 20];

        itertools::iproduct!(spends).map(|(spends)| TestParams { spends, time: 0.0 })
    }

    let mut results = Vec::new();

    for test_params in param_grid() {
        let params = BenchParams {
            network: Network::Mainnet,
            pool: ShieldedPool::Orchard,
            lightwalletd_url: "http://localhost:443".to_string(),
            start_block: TIP - 108000, // 90 days worth of blocks
            end_block: TIP,
            block_batch_size: 0,
        };
        let start = PERFORMANCE.now();
        zcash_wasm_benchmark::generate_proof_bench(params, test_params.spends);
        let elapsed = PERFORMANCE.now() - start;

        let result = TestParams {
            time: elapsed,
            ..test_params
        };
        results.push(result);
    }

    let json = serde_json::to_string(&results).unwrap();
    let mut df = JsonReader::new(std::io::Cursor::new(json))
        .finish()
        .unwrap();

    let mut buf = Vec::new();
    CsvWriter::new(&mut buf).finish(&mut df).unwrap();
    console_log!("{}", String::from_utf8(buf).unwrap()); // can't write a file from a web test so we just have to write to console
    console_log!("{:?}", df);
}

async fn init_threadpool(threads: usize) -> JsFuture {
    JsFuture::from(init_thread_pool(threads))
}
