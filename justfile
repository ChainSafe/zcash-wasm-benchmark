default_profile := "release"

default:
    just --list

alias b := build-rust

run: clean-web build serve

# Build both the WASM binary and the web page
build: build-rust build-page

# Build both parallel and non parallel WASM binaries
build-rust profile=default_profile: (_check-profile profile) && (build-parallel profile)
    @echo Building both Parallel and non Parallel features in {{ profile }} mode

# Build WASM binary with Parallel feature
build-parallel profile=default_profile: (_check-profile profile)
    @echo Building with Parallel feature in {{ profile }} mode
    wasm-pack build -t web  --{{ profile }} --out-dir demo-page/wasm-pkg/parallel --features=parallel

_check-profile profile:
    @echo {{ if profile =~ "release|debug|profiling" { "" } else { error("Profile must be one of: release|debug|profiling") } }} > /dev/null 2>&1

# Serves the web page using Parcel
serve:
    cd demo-page && yarn && yarn dev

# Serves the web page using Parcel
build-page:
    cd demo-page && yarn && yarn build

# Test in headless mode on firefox
test-headless-firefox *FLAGS:
    WASM_BINDGEN_TEST_TIMEOUT=99999 wasm-pack test  --release --headless --firefox --features=no-bundler {{FLAGS}}

# Test in headless mode on chrome
test-headless-chrome *FLAGS:
   WASM_BINDGEN_TEST_TIMEOUT=99999  wasm-pack test  --release --headless --chrome --features=no-bundler {{FLAGS}}

# Clean the WASM binaries and other artifacts from wasm-pack
clean-wasm:
    rm -rf demo-page/wasm-pkg

# Clean parcel-cache and dist
clean-dist:
    rm -rf demo-page/dist demo-page/.parcel-cache

# Clean Web build
clean-web: clean-dist clean-wasm

# Clean all
clean: clean-web
    rm -rf target

# Use the protobuf definitions to generate the javascript files for grpc-web
generate-pb-js:
    mkdir -p blockstream/generated
    protoc -I=./protos service.proto compact_formats.proto --js_out=import_style=commonjs:./blockstream/generated \
    --grpc-web_out=import_style=typescript,mode=grpcwebtext:./blockstream/generated

# run a local proxy to the lightwalletd server on port 443
run-proxy:
    grpcwebproxy  --backend_max_call_recv_msg_size=10485760 --server_http_max_write_timeout=1000s --server_http_max_read_timeout=1000s \
    --backend_addr=mainnet.lightwalletd.com:9067 --run_tls_server=false --backend_tls --allow_all_origins --server_http_debug_port 443
