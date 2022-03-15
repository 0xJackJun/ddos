// use ic_cdk_macros::*;
// use crate::pow::{get_state};
// #[query]
// fn hello() {
//     ic_cdk::print("Hello World!");
// }

// #[link(wasm_import_module = "ic0")]
// extern "C" {
//   pub fn accept_message() -> ();
//   pub fn reject_message() -> ();
// }

// #[export_name = "canister_inspect_message"]
// fn canister_inspect_message() {
//   unsafe{
//     let diffculty = get_state().difcculty;
//     let pow = |dif| {
//       let mut rng = thread_rng();
//       let random: u32 = rng.gen_range(0,1000);
//       let nonce: (u32,) = ic_cdk::api::call::arg_data();
//       let mut s = DefaultHasher::new();
//       let total = nonce.0 + random;
//       total.hash(&mut s);
//       let ans = s.finish();
//       if ans % 10**&difcculty == 0 {
//           return true;
//       }
//       return false;
//   };
//     if pow(diffculty) {
//       accept_message();
//     } else {
//       reject_message();
//     }
//   }
// }
