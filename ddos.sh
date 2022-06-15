#!/bin/bash

dfx canister call heartbeat task_one '(11111111111:nat64)'
dfx canister call heartbeat increase_data