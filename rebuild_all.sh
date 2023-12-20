#!/usr/bin/env bash

(
    cd contract_marketplace
    cargo build --release --target wasm32-unknown-unknown --no-default-features --features onchain
)

(
    cd contract_custodial
    cargo build --release --target wasm32-unknown-unknown --no-default-features --features onchain
)