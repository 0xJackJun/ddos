import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type canister_id = Principal;
export interface canister_status {
  'status' : { 'stopped' : null } |
    { 'stopping' : null } |
    { 'running' : null },
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : definite_canister_settings,
  'module_hash' : [] | [Array<number>],
}
export interface definite_canister_settings {
  'freezing_threshold' : bigint,
  'controllers' : Array<Principal>,
  'memory_allocation' : bigint,
  'compute_allocation' : bigint,
}
export interface _SERVICE {
  'canister_status' : ActorMethod<
    [{ 'canister_id' : canister_id }],
    canister_status,
  >,
}
