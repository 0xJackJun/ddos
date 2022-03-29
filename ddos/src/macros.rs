#[macro_export]
macro_rules! implement_ddos {
    () => {
        use candid::candid_method;
        use ic_cdk_macros::*;
        //---------pow------------
        #[derive(Default)]
        pub struct Diffculty {
            pub difcculty: u32,
        }
        #[update]
        #[candid_method(update)]
        pub fn init() {
            get_state().difcculty = 0;
        }

        #[update]
        #[candid_method(update)]
        pub fn increase_diffculty() {
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

        pub fn pow() -> bool {
            unsafe {
                let dif = get_state().difcculty;
                let random: u32 = ic_cdk::api::time as u32;
                let nonce: (u32,) = ic_cdk::api::call::arg_data();
                let mut s = DefaultHasher::new();
                let total = nonce.0 + random;
                total.hash(&mut s);
                let ans = s.finish() as u32;
                ic_cdk::print(format!("the ans is : {}", ans));
                let base: u32 = 10;
                if ans % base.pow(dif) == 0 {
                    return true;
                }
                return false;
            }
        }

        #[inspect_message]
        fn inspect_message() {
            let diffculty = get_state().difcculty;
            ic_cdk::print(format!("{}", diffculty));
            if pow() {
                ic_cdk::println!("accepted");
                ic_cdk::api::call::accept_message();
            } else {
                ic_cdk::println!("Do prove of work again");
            }
        }
    };
}

// #[cfg(test)]
// mod tests {
//     use crate as ddos;
//     use crate::implement_ddos;
//     use ic_cdk::*;
//     use ic_cdk_macros::*;
//     use rand::{thread_rng, Rng};
//     use std::{
//         collections::hash_map::DefaultHasher,
//         hash::{Hash, Hasher},
//     };

//     implement_ddos!();
//     // #[update(name = "test")]
//     // #[candid_method(update, rename = "test")]
//     // fn test_inspect_message(arg: u32) {
//     //     ic_cdk::print("test succeed");
//     // }
//     #[test]
//     fn test_init() {
//         let state = init();
//         assert_eq!(0, get_state().difcculty);
//     }
//     #[test]
//     fn test_improve_diffculty() {
//         let state = increase_diffculty();
//         assert_eq!(1, get_state().difcculty);
//     }
// }
