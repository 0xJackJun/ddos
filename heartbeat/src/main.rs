use candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_cdk::{api, storage, trap};
use ic_cdk_macros::*;
use ic_cron::types::{Iterations, SchedulingInterval, TaskId};
use std::collections::HashMap;

//--------DATA STRUCTURE-------
struct Counter(Principal);
struct Owner(Principal);
#[derive(CandidType, Deserialize, Debug)]
pub struct Cycle {
    pub canister_id: Principal,
}
impl Default for Counter {
    fn default() -> Self {
        Counter(Principal::anonymous())
    }
}

impl Default for Owner {
    fn default() -> Self {
        Owner(Principal::anonymous())
    }
}

#[derive(CandidType, Deserialize, Debug)]
pub struct UserData {
    pub canister_id: Principal,
    pub email: String,
    pub threshold: String,
}

#[derive(Default, Deserialize)]
pub struct CycleData {
    pub cycle: HashMap<String, u128>,
}

#[derive(CandidType, Deserialize)]
pub enum CronTaskKind {
    One(String),
}

#[derive(Default)]
pub struct AutomaticCounter {
    pub counter_1: u64,
    pub counter_1_started: bool,
    pub user_data: Vec<UserData>,
}

//----------MAIN LOGIC----------
#[update(name = "setCounter")]
#[candid_method(update, rename = "setCounter")]
fn set_counter(counter: Principal) {
    let caller = ic_cdk::caller();
    let owner = storage::get::<Owner>();
    assert_eq!(caller, owner.0);
    let _counter = storage::get_mut::<Counter>();
    *_counter = Counter(counter);
}

#[update]
#[candid_method(update)]
async fn get(duration_nano: u64) -> TaskId {
    let state = get_state();
    if state.counter_1_started {
        trap("Counter 1 already started");
    }
    let jobs = cron_enqueue(
        1 as u8,
        CronTaskKind::One(String::from("sweet")),
        SchedulingInterval {
            duration_nano,
            iterations: Iterations::Infinite,
        },
    );
    state.counter_1_started = true;

    jobs.unwrap()
}

// --------------- RECURRENCE ------------------
ic_cron::implement_cron!();

#[init]
#[candid_method(init)]
fn init() {
    ic_cdk::print("INIT");
    let caller = ic_cdk::caller();
    let owner = storage::get_mut::<Owner>();
    *owner = Owner(caller);
}

#[heartbeat]
async fn tick() {
    for task in cron_ready_tasks() {
        let kind = task
            .get_payload::<CronTaskKind>()
            .expect("Unable to deserialize cron task kind");

        match kind {
            CronTaskKind::One(message) => {
                ic_cdk::print(format!("Task One executed: {}", message.as_str()).as_str());
                let counter = storage::get::<Counter>();
                let result: Result<(Vec<UserData>,), _> =
                    api::call::call(counter.0, "getUser", ()).await;
                let state = get_state();
                state.user_data = result.unwrap().0;
                let data = &state.user_data[0];
                let id = data.canister_id.clone();
                let cycles = Cycle { canister_id: id };
                let cycle: Result<(Nat,), _> =
                    api::call::call(counter.0, "getCycle", (cycles,)).await;
                ic_cdk::print(format!("The result is {:?}", cycle));
                for data in state.user_data.iter() {
                    let id = data.canister_id.clone();
                    let cycle: Result<(Nat,), _> =
                        api::call::call(counter.0, "getCycle", (&id,)).await;
                    // if cycle.unwrap().0 - get_cycle().cycle.get(&id).unwrap() > 100{
                    //     let diff = api::call::call(counter.0, "increace_diffculty", ()).await;
                    // }
                    ic_cdk::print(format!("The result is {:?}", cycle.unwrap().0));
                }
            }
        }
    }
}

// ------------------ STATE ----------------------
static mut STATE: Option<AutomaticCounter> = None;

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

static mut CYCLE: Option<CycleData> = None;

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

//----------GENERATE CANDID FILE------------
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
