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
    wasm-pack build -t web  --{{ profile }} --out-dir pkg/parallel --features=parallel

# Build WASM binary without Parallel feature
build-no-parallel profile=default_profile: (_check-profile profile)
    @echo Building without Parallel feature in {{ profile }} mode 
    wasm-pack build -t web  --{{ profile }} --out-dir pkg/serial

_check-profile profile:
    @echo {{ if profile =~ "release|debug|profiling" { "" } else { error("Profile must be one of: release|debug|profiling") } }} > /dev/null 2>&1

# Start the Python webserver
serve:
    python3 server.py

# Builds the WASM binaries and starts the Python webserver
start profile=default_profile: (build-all profile) serve

# Clean the WASM binaries and other artifacts from wasm-pack
clean-wasm:
    rm -rf pkg

# Clean all
clean: clean-wasm
    rm -rf target
