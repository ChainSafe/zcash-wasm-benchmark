import { useState, useEffect } from "react";
import "./App.css";
import initWasm, { sapling_decrypt_wasm, initThreadPool } from "../wasm-pkg/parallel";

const SAPLING_ACTIVATION = 419200;
const ORCHARD_ACTIVATION = 653600;

const MAINNET_LIGHTWALLETD_PROXY = "http://localhost:443";
const TESTNET_LIGHTWALLETD_PROXY = "http://testnet.localhost:443";

export function App() {

    // Setup
    useEffect(() => {
        async function init() {
            await initWasm();
            let threads = navigator.hardwareConcurrency || 1;
            console.log("Initializing thread pool with", threads, "threads");
            await initThreadPool(threads);
        }
        init();
    }, []);

    // State
    let [startBlock, setStartBlock] = useState(SAPLING_ACTIVATION);
    let [endBlock, setEndBlock] = useState(SAPLING_ACTIVATION + 1000);
    let [network, setNetwork] = useState("mainnet");
    let [shieldedPool, setShieldedPool] = useState("sapling");
    let [lightwalletdProxy, setLightwalletdProxy] = useState(MAINNET_LIGHTWALLETD_PROXY);
    let [paymentFrequency, setPaymentFrequency] = useState(0);
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
        if (pool === "sapling") {
            setStartBlock(SAPLING_ACTIVATION);
            setEndBlock(SAPLING_ACTIVATION + 1000);
        } else {
            setStartBlock(ORCHARD_ACTIVATION);
            setEndBlock(ORCHARD_ACTIVATION + 1000);
        }
    }

    async function runTrialDecryption() {
        await sapling_decrypt_wasm(startBlock, endBlock);
    }

    async function runTreeStateSync() {
    }

    async function runProofGeneration() {

    }

    return (
        <div>
            <h1>ZCash Web - Browser Benchmarks</h1>

            Open the browser console to see results of benchmarks

            <h2>Global Settings</h2>
            <div>
                <label>
                    Network:
                    <select value={network} onChange={e => onNetworkUpdate(e.target.value)}>
                        <option value="mainnet">Mainnet</option>
                        <option value="testnet">Testnet</option>
                    </select>   
                </label>
                <label>
                    Shielded Pool:
                    <select value={shieldedPool} onChange={e => onPoolUpdate(e.target.value)}>
                        <option value="sapling">Sapling</option>
                        <option value="orchard">Orchard</option>
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
            </div>

            <hr />

            <div>
                <h2>Trial Decryption</h2>
                <p>
                    Download all blocks in the provided range and trial decrypt all transactions
                </p>
                <button onClick={runTrialDecryption}>Start</button>
            </div>

            <hr />

            <div>
                <h2>Treestate Sync</h2>
                <p>Retrieve the commitment tree frontier as of start_block and insert all note commitments to advance the tree up to end_block.</p>
                <p>To simulate a wallet that is receiving outputs and tracking witnesses this test optionally mark random outputs every X blocks to update a witness for.</p>
                <label>
                    Payment Frequency:
                    <input type="number" value={paymentFrequency} onChange={(e) => setPaymentFrequency(Number(e.target.value))} />
                </label>
                <button onClick={runTreeStateSync}>Start</button>
            </div>

            <hr />

            <div>
                <h2>Proof Generation</h2>
                <p>Generate a proof for a dummy transaction with the given number of spends</p>
                <label>
                    Number of spends:
                    <input type="number" value={proofGenerationSpends} onChange={e => setProofGenerationSpends(Number(e.target.value))} />
                </label>
                <button onClick={runProofGeneration}>Start</button>
            </div>
        </div>
    );
}
