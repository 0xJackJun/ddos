export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'setCaller' : IDL.Func([IDL.Principal], [], []),
    'task_one' : IDL.Func([IDL.Nat64], [IDL.Nat64], []),
  });
};
export const init = ({ IDL }) => { return []; };