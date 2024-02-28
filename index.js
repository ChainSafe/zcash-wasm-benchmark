/*
 * Copyright 2022 Google Inc. All Rights Reserved.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *     http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import init, { initThreadPool, what, proof } from './pkg/zcash_wasm_benchmark.js';

await init();
let num_concurrency = navigator.hardwareConcurrency;
console.log("num_concurrency: ", num_concurrency);
await initThreadPool(num_concurrency);
// // 1...10
// let arr = Int32Array.from({ length: 10 }, (_, i) => i + 1);
// if (sum(arr) !== 55) {
//   throw new Error('Wrong result.');
// }
proof();
// Note: this will be overridden by the Playwright test runner.
// The default implementation is provided only for manual testing.
globalThis.onDone ??= () => console.log('OK');

onDone();