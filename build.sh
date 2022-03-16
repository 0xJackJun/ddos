#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
dfx stop
dfx start --clean --background
dfx deploy
dfx canister call heartbeat setCaller "(principal \"$(dfx canister id service)\")"
dfx canister call service create '(record {threshold = "11"; canister_id = principal "ryjl3-tyaaa-aaaaa-aaaba-cai"; email = "jack"})'
dfx canister call service create '(record {threshold = "11"; canister_id = principal "rrkah-fqaaa-aaaaa-aaaaq-cai"; email = "jack"})'

dfx canister update-settings --controller rwlgt-iiaaa-aaaaa-aaaaa-cai --controller rrkah-fqaaa-aaaaa-aaaaq-cai ryjl3-tyaaa-aaaaa-aaaba-cai
dfx canister update-settings --controller rwlgt-iiaaa-aaaaa-aaaaa-cai --controller rrkah-fqaaa-aaaaa-aaaaq-cai rrkah-fqaaa-aaaaa-aaaaq-cai

dfx canister call heartbeat task_one '(11 : nat64)'
