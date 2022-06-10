import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface UserData {
  'threshold' : string,
  'canister_id' : Principal,
  'email' : string,
}
export type canister_id = Principal;
export interface _SERVICE {
  'create' : ActorMethod<[UserData], undefined>,
  'getController' : ActorMethod<
    [{ 'canister_id' : canister_id }],
    Array<Principal>,
  >,
  'getCount' : ActorMethod<[], bigint>,
  'getCycle' : ActorMethod<[{ 'canister_id' : canister_id }], bigint>,
  'getUser' : ActorMethod<[], Array<UserData>>,
}
