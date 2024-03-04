default_profile := "release"

default:
    just --list

alias b := build-all

# Build both parallel and non parallel WASM binaries
build-all profile=default_profile: (_check-profile profile) && (build-parallel profile) (build-no-parallel profile)
    @echo Building both Parallel and non Parallel features in {{ profile }} mode

# Build WASM binary with Parallel feature
build-parallel profile=default_profile: (_check-profile profile)
    @echo Building with Parallel feature in {{ profile }} mode
    wasm-pack build -t web  --{{ profile }} --out-dir blockstream/wasm-pkg/parallel --features=parallel

# Build WASM binary without Parallel feature
build-no-parallel profile=default_profile: (_check-profile profile)
    @echo Building without Parallel feature in {{ profile }} mode
    wasm-pack build -t web  --{{ profile }} --out-dir blockstream/wasm-pkg/serial

_check-profile profile:
    @echo {{ if profile =~ "release|debug|profiling" { "" } else { error("Profile must be one of: release|debug|profiling") } }} > /dev/null 2>&1

# Start the Python webserver
serve:
    python3 server.py

# Builds the WASM binaries and starts the Python webserver
start profile=default_profile: (build-all profile) serve

# Test in headless mode on firefox
test-headless-firefox +FLAGS:
    wasm-pack test  --release  --headless --firefox {{FLAGS}}

# Test in headless mode on chrome
test-headless-chrome +FLAGS:
    wasm-pack test  --release  --headless --chrome {{FLAGS}}

# Clean the WASM binaries and other artifacts from wasm-pack
clean-wasm:
    rm -rf blockstream/wasm-pkg

# Clean all
clean: clean-wasm
    rm -rf target

# Use the protobuf definitions to generate the javascript files for grpc-web
generate-pb-js:
    mkdir -p blockstream/generated
    protoc -I=./protos service.proto compact_formats.proto --js_out=import_style=commonjs:./blockstream/generated --grpc-web_out=import_style=commonjs,mode=grpcwebtext:./blockstream/generated

# run a local proxy to the lightwalletd server on port 443
run-proxy:
    grpcwebproxy --backend_addr=ai.lightwalletd.com:443 --run_tls_server=false --backend_tls --allow_all_origins --server_http_debug_port 443
