use candid::{CandidType, Deserialize, Nat, Principal};
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

static mut STATE: Option<AutomaticCounter> = None;

static mut CYCLE: Option<CycleData> = None;

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
