#[macro_export]
macro_rules! implement_ddos {
    () => {
        use candid::{candid_method, Principal};
        use ic_cdk::storage;
        use ic_cdk_macros::*;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // pub struct Caller(pub Principal);
        // pub struct Owner(pub Principal);

        // impl Default for Caller {
        //     fn default() -> Self {
        //         Caller(Principal::anonymous())
        //     }
        // }

        // impl Default for Owner {
        //     fn default() -> Self {
        //         Owner(Principal::anonymous())
        //     }
        // }

        // #[init]
        // #[candid_method(init)]
        // fn init() {
        //     let caller = ic_cdk::caller();
        //     let owner = storage::get_mut::<Owner>();
        //     *owner = Owner(caller);
        // }

        #[derive(Default)]
        pub struct Diffculty {
            pub difcculty: u32,
        }

        // #[update(name = "setCaller")]
        // #[candid_method(update, rename = "setCaller")]
        // fn set_caller(caller: Principal) {
        //     let _caller = ic_cdk::caller();
        //     let _owner = storage::get::<Owner>();
        //     assert_eq!(_caller, _owner.0);
        //     let storage = storage::get_mut::<Caller>();
        //     *storage = Caller(caller);
        // }

        #[update]
        #[candid_method(update)]
        pub fn initilize() {
            // let _caller = ic_cdk::caller();
            // let caller = storage::get::<Caller>();
            // assert_eq!(_caller, caller.0);
            get_state().difcculty = 0;
        }

        #[update]
        #[candid_method(update)]
        pub fn increase_diffculty() {
            // let _caller = ic_cdk::caller();
            // let caller = storage::get::<Caller>();
            // assert_eq!(_caller, caller.0);
            get_state().difcculty = 3;
            // if get_state().difcculty <=3 {
            //     get_state().difcculty += 1;
            // }
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
