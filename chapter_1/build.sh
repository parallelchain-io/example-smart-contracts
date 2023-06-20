#!/bin/bash

# Build Rust hello_contract
cargo build -p hello_contract --target wasm32-unknown-unknown --release