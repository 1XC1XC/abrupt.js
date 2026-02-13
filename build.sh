#!/bin/bash
set -euo pipefail

cargo build --release --manifest-path ./Cargo.toml
mkdir -p ./bin
cp -f ./target/release/libabrupt.so ./bin/index.node
