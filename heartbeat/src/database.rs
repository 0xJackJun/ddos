use candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_cdk_macros::*;
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Debug)]
pub struct UserData {
    pub canister_id: Principal,
    pub email: String,
    pub threshold: String,
}

#[derive(Default)]
pub struct AutomaticCounter {
    pub counter_1: u64,
    pub counter_1_started: bool,
    pub user_data: Vec<UserData>,
}

#[derive(Default, Deserialize)]
pub struct CycleData {
    pub cycle: HashMap<Principal, Nat>,
}
#[derive(CandidType, Default, Deserialize, Clone, Debug)]
pub struct DdosList {
    pub ddos_list: Vec<String>,
}

static mut STATE: Option<AutomaticCounter> = None;

static mut CYCLE: Option<CycleData> = None;

static mut DDOS: Option<DdosList> = None;

pub fn get_state() -> &'static mut AutomaticCounter {
    unsafe {
        match STATE.as_mut() {
            Some(s) => s,
            None => {
                STATE = Some(AutomaticCounter::default());
                get_state()
            }
        }
    }
}

pub fn get_cycle() -> &'static mut CycleData {
    unsafe {
        match CYCLE.as_mut() {
            Some(s) => s,
            None => {
                CYCLE = Some(CycleData::default());
                get_cycle()
            }
        }
    }
}

pub fn get_ddos_list() -> &'static mut DdosList {
    unsafe {
        match DDOS.as_mut() {
            Some(s) => s,
            None => {
                DDOS = Some(DdosList::default());
                get_ddos_list()
            }
        }
    }
}

#[update(name = "addDdos")]
#[candid_method(update, rename = "addDdos")]
fn add_ddos(email: String) {
    let state = get_ddos_list();
    state.ddos_list = vec![email];
}

#[query]
#[candid_method]
fn get_ddos() -> Vec<String> {
    let res = get_ddos_list().ddos_list.clone();
    ic_cdk::print(format!("{:?}", res));
    res
}

//example
#[update]
#[candid_method(update)]
pub fn increase_data() {
    // let _caller = ic_cdk::caller();
    // let caller = storage::get::<Caller>();
    // assert_eq!(_caller, caller.0);
    for i in 0..1000 {
        get_data().data += i;
    }
}

#[update]
#[candid_method(update)]
pub fn decrease_data() {
    // let _caller = ic_cdk::caller();
    // let caller = storage::get::<Caller>();
    // assert_eq!(_caller, caller.0);
    get_data().data = 0;
}

#[derive(Default)]
pub struct Data {
    pub data: u32,
}

static mut DATA: Option<Data> = None;

pub fn get_data() -> &'static mut Data {
    unsafe {
        match DATA.as_mut() {
            Some(s) => s,
            None => {
                DATA = Some(Data::default());
                get_data()
            }
        }
    }
}
