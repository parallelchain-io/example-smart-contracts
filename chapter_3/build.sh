#!/bin/bash

# Build Rust test contracts
cargo build -p my_bank --target wasm32-unknown-unknown --release