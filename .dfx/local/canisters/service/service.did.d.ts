import type { Principal } from '@dfinity/principal';
export interface UserData {
  'threshold' : string,
  'canister_id' : Principal,
  'email' : string,
}
export type canister_id = Principal;
export interface _SERVICE {
  'create' : (arg_0: UserData) => Promise<undefined>,
  'getCount' : () => Promise<bigint>,
  'getCycle' : (arg_0: { 'canister_id' : canister_id }) => Promise<bigint>,
  'getUser' : () => Promise<Array<UserData>>,
}
