import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'hello' : ActorMethod<[number], string>,
  'increase_data' : ActorMethod<[number], undefined>,
  'increase_diffculty' : ActorMethod<[number], undefined>,
  'init' : ActorMethod<[number], undefined>,
  'setCaller' : ActorMethod<[Principal], undefined>,
}
