# ZCash Web Benchmarks

A collection of benchmarks for evaluating the viability of a Zcash web wallet.

## Overview

This repo contains Rust crates designed to be compiled to Wasm to implement the most challenging parts of syncing a Zcash wallet, namely trial-decryption and note witness updating.

It also contains a webpage which can be used by anyone to obtain their own results using their own browser and system. This is currently hosted at https://chainsafe.github.io/zcash-wasm-benchmark/. Note that until a hosted lightwalletd web proxy is deployed this requires running a proxy locally (see below)

> Contributions are welcome! It is intended for this repo to be the definitive source of viability for a Zcash web wallet. Any improvements in sync algorithms can be benchmarked here and wallet development can progress once it has been determined to be viable.

## Prerequisites

- This repo uses [just](https://github.com/casey/just) as a command runner. Please install this first.
- Tested with Rust nightly-2024-02-26

In order to connect to a gRPC server from the browser it must pass through a grpc-web proxy. The simplest one to set up is the standalone Go proxy. Grab a binary from [here](https://github.com/improbable-eng/grpc-web/releases) or install it with:

```shell
go install github.com/improbable-eng/grpc-web/go/grpcwebproxy@latest
```

Once you have it, run it with the following to proxy to an existing public lightwalletd:

```shell
just run-proxy
```

### Generating benchmark results

#### Headless browser automated testing

Use a headless browser to generate test results with:

```shell
just test-headless-firefox
```
or
```shell
just test-headless-chrome
```

By default this runs tests with multiple repetitions and across a grid of different parameter cofigurations. The table of results will be displayed in the console.

#### In-browser Tests

Build the Wasm and webpage with

```shell
just build
```

and then run the page with

```shell
just serve
```

From the webpage you can configure and run your own tests and see the results in the web console.
