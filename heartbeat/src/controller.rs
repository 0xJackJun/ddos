use candid::{candid_method, Principal};
use ic_cdk::storage;
use ic_cdk_macros::*;

pub struct Caller(pub Principal);

pub struct Owner(pub Principal);

impl Default for Caller {
    fn default() -> Self {
        Caller(Principal::anonymous())
    }
}

impl Default for Owner {
    fn default() -> Self {
        Owner(Principal::anonymous())
    }
}

#[update(name = "setCaller")]
#[candid_method(update, rename = "setCaller")]
pub fn set_caller(caller: Principal) {
    let _caller = ic_cdk::caller();
    let _owner = storage::get::<Owner>();
    //assert_eq!(caller, owner.0);
    let storage = storage::get_mut::<Caller>();
    *storage = Caller(caller);
}
