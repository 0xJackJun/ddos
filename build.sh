#!/bin/bash

sudo cargo build --release --target wasm32-unknown-unknown
sudo ic-cdk-optimizer target/wasm32-unknown-unknown/release/heartbeat.wasm -o target/wasm32-unknown-unknown/release/opt.wasm
sudo ic-cdk-optimizer target/wasm32-unknown-unknown/release/example.wasm -o target/wasm32-unknown-unknown/release/opt_example.wasm
dfx stop
dfx start --clean --background
dfx deploy

# dfx canister call heartbeat task_one '(1111111111 : nat64)'