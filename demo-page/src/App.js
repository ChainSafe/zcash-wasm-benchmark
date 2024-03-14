import { useState } from "react";

export function App() {

    let [startBlock, setStartBlock] = useState(1);
    let [endBlock, setEndBlock] = useState(1);
    let [paymentFrequency, setPaymentFrequency] = useState(1);

    function trialDecryption() {
    }

    function treeStateSync() {
    }

    function proofGeneration() {

    }

    return (
        <div>
            <h1>ZCash Web - Browser Benchmarks</h1>

            Open the browser console to see results of benchmarks

            <div>
                <label>
                    Start Block:
                    <input type="number" value={startBlock} onChange={(e) => setStartBlock(e.target.value)} />
                </label>
                <label>
                    End Block:
                    <input type="number" value={endBlock} onChange={(e) => setEndBlock(e.target.value)} />
                </label>
            </div>

            <div>
                <h2>Trial Decryption</h2>
                <label>
                    Payment Frequency:
                    <input type="number" value={paymentFrequency} onChange={(e) => setPaymentFrequency(e.target.value)} />
                </label>
                <button onClick={trialDecryption}>Start</button>
            </div>
            <div>
                <h2>Treestate Sync</h2>
                <button onClick={treeStateSync}>Start</button>
            </div>
            <div>
                <h2>Proof Generation</h2>
                <label>
                    Number of spends:
                    <input type="number" defaultValue={1} />
                </label>
                <button onClick={proofGeneration}>Start</button>
            </div>
        </div>
    );
}
