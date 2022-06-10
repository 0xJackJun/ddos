export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'hello' : IDL.Func([IDL.Nat32], [IDL.Text], []),
    'increase_data' : IDL.Func([IDL.Nat32], [], []),
    'increase_diffculty' : IDL.Func([IDL.Nat32], [], []),
    'init' : IDL.Func([IDL.Nat32], [], []),
    'setCaller' : IDL.Func([IDL.Principal], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
