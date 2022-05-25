import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'addDdos' : (arg_0: string) => Promise<undefined>,
  'get_ddos' : () => Promise<Array<string>>,
  'setCaller' : (arg_0: Principal) => Promise<undefined>,
  'task_one' : (arg_0: bigint) => Promise<bigint>,
}
