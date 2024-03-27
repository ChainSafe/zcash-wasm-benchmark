# ZCash Web Wallet - Feasibility Report

## Executive Summary

ChainSafe investigated the current feasibility of a browser based wallet for zcash. In this study we identified the essential computations that a wallet cannot hand-off to an untrusted party and developed benchmarking tests for these using Zcash mainnet block data. These tests can be run by anyone on their own system from the [benchmarking site](https://chainsafe.github.io/zcash-wasm-benchmark/) produced as part of this study.

It was found that trial decryption and note commitment witness updating could be done in reasonable time for recent block ranges but woud result in a poor user experience for blocks during the DoS attack period. Halo2 transaction proofs can be generated in reasonable time for typical numbers of spends.

## Wallet Requirements

A fully features ZCash wallet must be able to perform the following operations:

1. Generation or import of keys
2. Maintaining a correct and spendable balance
3. Ability to construct transactions and accompanying proofs

It is assumed that key generation is trivial so this study focused only on the tasks required for maintaining a spendable balance and transaction proving.

### Maintaining Spendable Balance

As described in [this article][1], to maintain a correct and spendable balance a wallet is required to:

- Downloading compact blocks from a `lightwalletd` instance
- Trial-decrypt all on-chain notes using the wallet view keys to detect new funds
- Looking for nullifiers to detect spends of wallet notes
- Maintaining a set of witnesses for spendable notes

The trial-decryption and witness updating tasks are the most computationally intensive and cannot be outsourced to an untrusted third party without compromising wallet security. The main test for feasibility for a web-wallet is therefore if it can perform these two tasks within a reasonable time frame within the processing and memory constraints of a browser.

### Approach Taken

#### Block downloading

Downloading blocks is made challenging by the fact that `lightwalletd` was developed to use gRPC which cannot be used from within a browser due to limitations on how HTTP/2 can be configured. Fortunately there is an [actively maintained workaround](https://github.com/grpc/grpc-web) by which a proxy can act as a translation layer from requests made using grpc-web.

Using this we were able to effectively request streams of blocks from `lightwalletd` in the browser.

Our initial attemt was to use the grpc-web javascript library to retrieve the blocks and then pass these to the Wasm code for processing. Using this approach a significant amount of time was spent deserializing the responses and encoding them for Wasm. It was found to be significantly faster to make the grpc-web requests from the Rust code itself using the [tonic-web-wasm-client](https://docs.rs/tonic-web-wasm-client/latest/tonic_web_wasm_client/) crate. This also had the advantage of improving the codebase readability.

To support future web wallets a public proxy should be deployed for the public mainnet `lightwalletd` services such as those currently provided by [Nighthawk](https://lightwalletd.com/links) 

#### Trial Decryption

For trial decryption we used the implementation from the librustzcash [zcash_note_encryption](https://crates.io/crates/zcash_note_encryption) crate. Specifically the `batch_note_decryption` function. This was used to decrypt Sapling outouts and Orchard actions retrieved from the blocks. It wraps the RustCrypto implementation of ChaCha20-Poly1305. 

The trial decryption benchmark collects batches of blocks, extracts their outputs or actions (or both depending on pool configuration), decrypts them, and records how many notes successfully decrypted for the provided address. Transactions with more than 50 inputs/outputs/actions were ignored and presumed to be networks spam. 

##### Parallel Implementation

Support for multi-threading in Wasm using Web Workers has seen advancement in recent years. We tested this approach using the `rayon` data parallelism crate in Rust to divide batch into parts equal to the number of available threads and decrypt in parallel

Using this approach we were able to acchieve an significant speed-up in trial decryption (see the results section below) on a consumer laptop. Since this is a highly parallelizable problem any developments in browser multi-threading will see a direct improvement on this benchmark.

#### Commitment Tree Hashing

We used the ShardTree implementation from the [incrementalmerkletree](https://github.com/zcash/incrementalmerkletree/) crate with the pool specific hashing implementations from the [sapling_crypto](https://github.com/zcash/sapling-crypto) and [orchard](https://github.com/zcash/orchard) crates.

The benchmark first retrieves the treestate frontier from `lightwalletd` for the starting block height and uses this to build an initial tree and retrieve the correct tree index to begin inserting from.

It then retrieves batches of blocks, extracts the outputs/actions and inserts these into the tree in batches. Each batch insertion results in the recomputing of the tree root and updates the witnesses for any marked commitments. 

The number of witnesses to track was a configurable parameter and this corresponds to the number of unspent notes in the wallet. This was implemented by marking the first `n_witness` nodes in the tree to have their witnesses updated with each batch.

##### Parallel Implementation

Following the example of the [librustzcash sqlite client](https://github.com/zcash/librustzcash/tree/main/zcash_client_sqlite) the subtrees (shards) can be updated in parallel and then merged into a single incremental merkle tree. 

This approach did not yield any significant improvements. At this point it is unclear if this is due to overheads in merging the subtrees or another reason and this is left as a problem for future research.

#### Proof Generation

To test proof generation in-browser we used the proving benchmark implementation from the [Orchard](https://github.com/zcash/orchard) crate. This was used without any changes and does not require retrieving network data. The existing Rust implementation supports parallel proving and this worked in Wasm out of the box.

## Results

The following results were obtained by running the benchmarks in Firefox 124.0.1 (64-bit) on a 2023 Macbook Air with an M2 processor. All times include the time required to download the blocks from the public lightwalletd service. A thread pool of size 4 was used which corresponds to the number of performance cores on the CPU.

The Rust code to run the tests was build in release mode and the build optimized for speed with

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

A further pass of optimization on the resulting Wasm was performed with `wasm-opt`  at the `-O4` level.

The sync was done using mainnet blocks from the range [2334739, 2442739] which is the previous 108000 (90 days worth) of mainnet blocks at the time of writing. These blocks contain 41034 Sapling outputs and 29828 Orchard actions.

A further test was done for trial decryption on the entire block range from Orchard activation to Tip [1687104, 2442739]. This range contained 2877916 Sapling outputs and 5567465 Orchard actions.

### Trial Decryption

#### Range [2334739, 2442739]

| AVERAGE of Time (ms) | Batch Size |           |           |
| -------------------- | ---------- | --------- | --------- |
| **Pool**             | 100        | 1000      | 10000     |
| Both                 | 20,297.62  | 14,846.76 | 14,225.33 |
| Orchard              | 11,280.07  | 7,744.15  | 7,113.23  |
| Sapling              | 14,368.16  | 10,798.65 | 10,064.66 |

#### Range [1687104, 2442739] (Orchard activation to tip)


 **Pool** | Time (ms) |
| -------------------- | ---------- |
| Both                 | 3,353,402.88  |


### Tree Sync

| AVERAGE of Time(ms) |           | Batch Size |           |           |
| ------------------- | --------- | ---------- | --------- | --------- |
| **Pool**          | **Witnesses** | 100        | 1000      | 10000     |
| Both                | 1         | 40,886.88  | 34,194.27 | 15,533.97 |
|                     | 10        | 41,563.82  | 34,491.33 | 16,251.31 |
|                     | 100       | 43,357.89  | 35,274.38 | 16,873.77 |
| Orchard             | 1         | 28,745.92  | 23,659.24 | 11,986.07 |
|                     | 10        | 29,386.71  | 23,837.12 | 12,384.14 |
|                     | 100       | 30,568.21  | 24,472.55 | 12,949.25 |
| Sapling             | 1         | 18,856.83  | 15,987.43 | 10,434.75 |
|                     | 10        | 19,840.71  | 15,567.85 | 9,540.01  |
|                     | 100       | 20,379.17  | 15,934.47 | 8,948.01  |


### Proving

Note only Orchard Halo2 proving was evaluated

| Spends | Time (ms) |
| ------ | --------- |
| 1   | 5,446.1    |
| 5   | 12,184.32  |
| 10  | 23,650.72  |
| 20     | 122,305.76 |

## Discussion

### Trial Decryption

In the browser we were able to download an trial-decrypt 100k blocks (90 days of Zcash Mainnet) in around 15 seconds for both shielded pools. This corresponds to about 7700 blockes per second or around 5000 actions/outputs per second. Interestingly over this block range a number of the blocks must have been empty of shielded transactions as the number of outputs/actions is less than the total number of blocks.

Increasing the size of the batches of blocks kept in memory was effective in decreasing total decryption time however this is limited by the amount of memory available to the Wasm runtime (4GB).

The successfully decrypted notes and the last syncced height could be cached in browser store and so under ideal circumstances this computation would only need to be repeated for the new blocks added since the user last opened the web wallet.

Trial decryption is the main wallet sync process that cannot be handed off to a third party without revealing the wallet view key and these results suggest that it can be done in a very reasonable amount of time, especially for newer wallets and those that are re-opened regularly.

In the worst case (e.g. syncing from Orchard activation height) the total decryption time was very large taking around 55 minutes to complete. For this test the rate of decrypting actions/outputs dropped to around 2500 actions/outputs per second. It is assumed this decrease in performance is due to the additional network load required to download and deserialize blocks during the DoS attack period. Transaction with large number of inputs/outputs were filtered out but only after they had been downloaded and deserialized. It is expected that the average output processing rate could be significantly improved if these transactions were filtered out earlier by lightwalletd.

### Tree Sync

The tree-sync results showed very significant performance improvement by increasing the block batch size. This is likely due to the high efficiency of the parallel sub-tree updating strategy over large block batches. For smaller batches this reduces to the single thread algorithm.

The number of witnesses being updated corresponds to the number of unspent transactions in the wallet. It was expected that the time to sync would increase significantly with the number of witnesses but the batch updating implementation in the ShardTree eliminates duplicate hashing of shared witness paths and leads to only a minimal decrease in performance.

For this test the commitments for which witnesses were tracked were located in close proximity in the tree. A future improvement to this test might randomly scatter them in the tree which would better approximate the distribution of a wallets unspent outputs. This would likely see a more pronounced decrease in performance as the number of witnesses increases as they share fewer branches in the tree.

Again this implementation shows very reasonable sync time with large batches in the multi-threaded implementation. If both the trial-decryption and tree-sync were performed on the same block data the time to download and deserialize blocks would be amortized between both and the time less than the sum of the measured time between tests.

Furthermore the tree sync does not need to be applied to from the wallet birthday to the chain tip. It only needs to be applied from the most recent uspent note. Strategies such as [BlazeSync](https://github.com/zecwalletco/zips/blob/blazesync/zip-1015.rst) and [DAGSync](https://words.str4d.xyz/dagsync-graph-aware-zcash-wallets/) could be applied to intelligently update the witnesses.

### Proving

Transaction Halo2 proofs were generated for various number of spends. Surprisingly this turned out to be an expensive operation with the proof generation for a non-trivial spend taking over a minute. It is not clear if this is due to the Halo2 proving failing to take advantage of the multiple threads available or if the proving code is poorly optimized for Wasm (e.g uses wide integer operations). This is significantly longer than the time to generate the same proof natively.

However given the block time in Zcash is 1 minute, 20 seconds of proving time before sending a transaction is not unreasonable.

## Conclusion

From these results we conclude it would be possible to build a Zcash browser wallet with a good user experience under certain circumstances. Such a wallet would not be able to sync the entire chain in a reasonable time but for newly created wallets that are opened fairly frequently (every 90 days or so) the user experience could be satisfactory. As the main target audience for such a wallet is new users these conditions could be met.

Syncing from Orchard activation takes a significantly long time that would be unacceptable to most users. It is expected this time could be reduced significantly by applying spam transaction filtering in lightwalletd.

Further optimizations to sync time could be made by pipelining the trial decryption and tree-sync, intelligent tree sync strategies, and lower level optimizations of the expensive cryptographic operations for Wasm or even [WebGPU](https://geometry.dev/notebook/accelerating-client-side-zk-with-webgpu).
