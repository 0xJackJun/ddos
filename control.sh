#!/bin/bash
dfx canister call heartbeat setCaller "(principal \"$(dfx canister id service)\")"
dfx canister update-settings --controller rwlgt-iiaaa-aaaaa-aaaaa-cai --controller rrkah-fqaaa-aaaaa-aaaaq-cai ryjl3-tyaaa-aaaaa-aaaba-cai
dfx canister update-settings --controller rwlgt-iiaaa-aaaaa-aaaaa-cai --controller rrkah-fqaaa-aaaaa-aaaaq-cai rno2w-sqaaa-aaaaa-aaacq-cai

dfx canister --wallet "$(dfx identity get-wallet)" update-settings --add-controller ezw55-al2r4-u5pm6-jaew5-43qve-46acg-ypjdh-caeh4-3iv3o-eh5qw-kae --all