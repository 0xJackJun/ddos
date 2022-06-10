export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'addDdos' : IDL.Func([IDL.Text], [], []),
    'decrease_data' : IDL.Func([], [], []),
    'get_ddos' : IDL.Func([], [IDL.Vec(IDL.Text)], []),
    'increase_data' : IDL.Func([], [], []),
    'setCaller' : IDL.Func([IDL.Principal], [], []),
    'task_one' : IDL.Func([IDL.Nat64], [IDL.Nat64], []),
  });
};
export const init = ({ IDL }) => { return []; };
