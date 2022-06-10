#!/bin/bash
dfx canister call heartbeat setCaller "(principal \"$(dfx canister id service)\")"
dfx canister update-settings --controller rwlgt-iiaaa-aaaaa-aaaaa-cai --controller rrkah-fqaaa-aaaaa-aaaaq-cai ryjl3-tyaaa-aaaaa-aaaba-cai
dfx canister update-settings --controller rwlgt-iiaaa-aaaaa-aaaaa-cai --controller rrkah-fqaaa-aaaaa-aaaaq-cai rno2w-sqaaa-aaaaa-aaacq-cai