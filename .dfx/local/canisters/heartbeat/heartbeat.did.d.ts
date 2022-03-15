import type { Principal } from '@dfinity/principal';
export interface UserData {
  'threshold' : string,
  'canister_id' : string,
  'email' : string,
}
export interface _SERVICE {
  'get' : (arg_0: bigint) => Promise<bigint>,
  'setCounter' : (arg_0: Principal) => Promise<undefined>,
}
