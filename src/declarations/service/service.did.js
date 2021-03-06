export const idlFactory = ({ IDL }) => {
  const UserData = IDL.Record({
    'threshold' : IDL.Text,
    'canister_id' : IDL.Principal,
    'email' : IDL.Text,
  });
  const canister_id = IDL.Principal;
  return IDL.Service({
    'create' : IDL.Func([UserData], [], []),
    'getController' : IDL.Func(
        [IDL.Record({ 'canister_id' : canister_id })],
        [IDL.Vec(IDL.Principal)],
        [],
      ),
    'getCount' : IDL.Func([], [IDL.Nat], []),
    'getCycle' : IDL.Func(
        [IDL.Record({ 'canister_id' : canister_id })],
        [IDL.Nat],
        [],
      ),
    'getUser' : IDL.Func([], [IDL.Vec(UserData)], []),
  });
};
export const init = ({ IDL }) => { return []; };
