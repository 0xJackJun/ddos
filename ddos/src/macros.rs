use ic_cdk_macros::*;
use rand::{thread_rng, Rng};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

macro_rules! implement_ddos {
    () => {
        //---------pow------------
        #[derive(Default)]
        pub struct Diffculty {
            pub difcculty: u64,
        }

        pub fn init() {
            get_state().difcculty = 0;
        }

        pub fn increace_diffculty() {
            get_state().difcculty += 1;
            if get_state().difcculty >= 4 {
                get_state().difcculty = 4;
            }
        }

        static mut STATE: Option<Diffculty> = None;

        pub fn get_state() -> &'static mut Diffculty {
            unsafe {
                match STATE.as_mut() {
                    Some(s) => s,
                    None => {
                        STATE = Some(Diffculty::default());
                        get_state()
                    }
                }
            }
        }
        //----------------inspect-------------
        #[link(wasm_import_module = "ic0")]
        extern "C" {
            pub fn accept_message() -> ();
            pub fn reject_message() -> ();
        }

        #[export_name = "canister_inspect_message"]
        fn canister_inspect_message() {
            unsafe {
                let diffculty = get_state().difcculty;
                let pow = |dif| {
                    let mut rng = thread_rng();
                    let random: u32 = rng.gen_range(0, 1000);
                    let nonce: (u32,) = ic_cdk::api::call::arg_data();
                    let mut s = DefaultHasher::new();
                    let total = nonce.0 + random;
                    total.hash(&mut s);
                    let ans = s.finish();
                    if ans % 10 * *&difcculty == 0 {
                        return true;
                    }
                    return false;
                };
                if pow(diffculty) {
                    accept_message();
                } else {
                    reject_message();
                }
            }
        }
    };
}
