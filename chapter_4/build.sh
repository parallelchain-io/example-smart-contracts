#!/bin/bash

# Build Rust contract_proxy
cargo build -p contract_proxy --target wasm32-unknown-unknown --release