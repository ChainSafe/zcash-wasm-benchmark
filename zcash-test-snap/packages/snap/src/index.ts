import type { OnRpcRequestHandler } from '@metamask/snaps-sdk';
import { panel, text } from '@metamask/snaps-sdk';

import { generate_proof_bench, BenchParams, trial_decryption_bench } from "../wasm-pkg";

const TIP = 2442739;

function current_params() {
  return new BenchParams(
      "mainnet",
      "orchard",
      "http://localhost:443",
      TIP - 36000,
      TIP,
      1000,
  );
}
// import * as program from "../wasm-pkg/zcash_wasm_benchmark_bg.wasm";
// initSync(program);
/**
 * Handle incoming JSON-RPC requests, sent through `wallet_invokeSnap`.
 *
 * @param args - The request handler args as object.
 * @param args.origin - The origin of the request, e.g., the website that
 * invoked the snap.
 * @param args.request - A validated JSON-RPC request object.
 * @returns The result of `snap_dialog`.
 * @throws If the request method is not valid for this snap.
 */
export const onRpcRequest: OnRpcRequestHandler = async ({
  origin,
  request,
}) => {
  switch (request.method) {
    case 'proof':
      console.log("Proof method called");
      generate_proof_bench(current_params(), 4);
      return 3;
    case 'trial-decrypt':
      console.log("Trial decrypt method called");
      return await trial_decryption_bench(current_params(), 50);

    default:
      throw new Error('Method not found.');
  }
};
