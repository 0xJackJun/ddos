import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'addDdos' : ActorMethod<[string], undefined>,
  'decrease_data' : ActorMethod<[], undefined>,
  'get_ddos' : ActorMethod<[], Array<string>>,
  'increase_data' : ActorMethod<[], undefined>,
  'setCaller' : ActorMethod<[Principal], undefined>,
  'task_one' : ActorMethod<[bigint], bigint>,
}
