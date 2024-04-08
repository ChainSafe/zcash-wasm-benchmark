import { useState, useEffect } from "react";
import "./App.css";
import initWasm, { trial_decryption_bench, generate_proof_bench, sync_commitment_tree_bench, initThreadPool, BenchParams } from "../wasm-pkg/parallel";

const SAPLING_ACTIVATION = 419200;
const ORCHARD_ACTIVATION = 1687104;
const TIP = 2442739;

const MAINNET_LIGHTWALLETD_PROXY = "https://zcash-mainnet.chainsafe.dev";
const TESTNET_LIGHTWALLETD_PROXY = "https://zcash-testnet.chainsafe.dev";

export function App() {

    // Setup
    useEffect(() => {
        async function init() {
            await initWasm();
        }
        init();
    }, []);

    // State
    let [nThreads, setNThreads] = useState(navigator.hardwareConcurrency || 1);
    let [startBlock, setStartBlock] = useState(TIP - 36000);
    let [endBlock, setEndBlock] = useState(TIP);
    let [batchSize, setBatchSize] = useState(1000);
    let [network, setNetwork] = useState("mainnet");
    let [shieldedPool, setShieldedPool] = useState("both");
    let [lightwalletdProxy, setLightwalletdProxy] = useState(MAINNET_LIGHTWALLETD_PROXY);
    let [spamFilterLimit, setSpamFilterLimit] = useState(50);
    let [witnesses, setWitnesses] = useState(10);
    let [proofGenerationSpends, setProofGenerationSpends] = useState(1);

    // Event Handlers
    function onNetworkUpdate(network) {
        setNetwork(network);
        if (network === "mainnet") {
            setLightwalletdProxy(MAINNET_LIGHTWALLETD_PROXY);
        } else {
            setLightwalletdProxy(TESTNET_LIGHTWALLETD_PROXY);
        }
    }

    function onPoolUpdate(pool) {
        setShieldedPool(pool);
    }

    function current_params() {
        return new BenchParams(
            network,
            shieldedPool,
            lightwalletdProxy,
            startBlock,
            endBlock,
            batchSize,
        );
    }

    async function runTrialDecryption() {
        await trial_decryption_bench(current_params(), spamFilterLimit);
    }

    async function runTreeStateSync() {
        sync_commitment_tree_bench(current_params());
    }

    async function runProofGeneration() {
        generate_proof_bench(current_params(), proofGenerationSpends)
    }

    async function setupWorkers() {
        console.log("Initializing thread pool with", nThreads, "threads");
        await initThreadPool(nThreads);
    }

    return (
        <div>
            <h1>ZCash Web - Browser Benchmarks</h1>

            Open the browser console to see results of benchmarks

            <h2>Multi-thread Setup</h2>
                <p>THIS MUST BE SET EXACTLY ONCE BEFORE ANY TESTS CAN BE RUN.</p>
                <p>It will initialize a pool of web workers. If you want to change this you need to refresh the page.</p>
                <label>
                    Number of threads:
                    <input type="number" value={nThreads} onChange={(e) => setNThreads(e.target.value)} />
                </label>
                <button onClick={setupWorkers}>Init Threadpool</button>
            <hr />

            <h2>Global Settings</h2>
            <div>
                <label>
                    Network:
                    <select value={network} onChange={e => onNetworkUpdate(e.target.value)}>
                        <option value={"mainnet"}>Mainnet</option>
                        <option value="testnet">Testnet</option>
                    </select>   
                </label>
                <label>
                    Shielded Pool:
                    <select value={shieldedPool} onChange={e => onPoolUpdate(e.target.value)}>
                        <option value="sapling">Sapling</option>
                        <option value="orchard">Orchard</option>
                        <option value="both">Both</option>
                    </select>  
                </label>
                <label>
                    lightwalletd URL (must have grpc-web proxy):
                    <input type="text" value={lightwalletdProxy} onChange={(e) => setLightwalletdProxy(e.target.value)} />
                </label>
                <label>
                    Start Block:
                    <input type="number" value={startBlock} onChange={(e) => setStartBlock(e.target.value)} />
                </label>
                <label>
                    End Block:
                    <input type="number" value={endBlock} onChange={(e) => setEndBlock(e.target.value)} />
                </label>
                <span >{`${endBlock - startBlock} blocks. Approximately ${Math.round((endBlock - startBlock) * 1.2 / 60 / 24) } days on Zcash mainnet`}</span>
                <br/>
                <label>
                    Block batch size:
                    <input type="number" value={batchSize} onChange={(e) => setBatchSize(e.target.value)} />
                </label>
            </div>

            <hr />

            <div>
                <h2>Trial Decryption</h2>
                <p>
                    Download all blocks in the provided range and trial decrypt all transactions
                </p>
                <label>
                    Skip txns with outputs greater than:
                    <input type="number" value={spamFilterLimit} onChange={(e) => setSpamFilterLimit(e.target.value)} />
                </label>
                <button onClick={runTrialDecryption}>Start</button>
            </div>

            <hr />

            <div>
                <h2>Treestate Sync</h2>
                <p>Retrieve the commitment tree frontier as of start_block and insert all note commitments to advance the tree up to end_block.</p>
                <p>To simulate a wallet that actually has spendable notes the first note from every batch is added to the list to maintain witnesses for.</p>
                <button onClick={runTreeStateSync}>Start</button>
            </div>

            <hr />

            <div>
                <h2>Proof Generation</h2>
                <p>Generate a proof for a dummy transaction with the given number of spends. Note the shielded pool must be set to Orchard for this test.</p>
                <label>
                    Number of spends:
                    <input type="number" value={proofGenerationSpends} onChange={e => setProofGenerationSpends(Number(e.target.value))} />
                </label>
                <button onClick={runProofGeneration}>Start</button>
            </div>
        </div>
    );
}
