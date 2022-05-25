export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'addDdos' : IDL.Func([IDL.Text], [], []),
    'get_ddos' : IDL.Func([], [IDL.Vec(IDL.Text)], []),
    'setCaller' : IDL.Func([IDL.Principal], [], []),
    'task_one' : IDL.Func([IDL.Nat64], [IDL.Nat64], []),
  });
};
export const init = ({ IDL }) => { return []; };
