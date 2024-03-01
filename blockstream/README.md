## Blockstream

An experiment streaming blocks from a lightwalletd instance to a web client.

## Prerequisites

In order to connect to a gRPC server from the browser it must pass through a grpc-web proxy. The simplest one to set up is the standalone Go proxy. Grab a binary from [here](https://github.com/improbable-eng/grpc-web/releases) or install it with:

```shell
go install github.com/improbable-eng/grpc-web/go/grpcwebproxy@latest
```

Once you have it, run it with the following to proxy to an existing public lightwalletd:

```shell
just run-proxy
```

Now you can bundle and serve the demo with:

```shell
yarn
yarn parcel index.html
```

### Code Generation

Generating the gRPC web client requires the `protoc` compiler and the plugins for generating js and grpc-web interfaces.

```shell
brew install protobuf protoc-gen-grpc-web
npm install -g protoc-gen-js
```

### Random Info

Read up on grpc-web [here](https://github.com/grpc/grpc-web)

At some point we will need to figure out how to deploy a production grade proxy rather than the debug golang one we are using right now
