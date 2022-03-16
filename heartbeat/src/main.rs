use candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_cdk::{api, storage, trap};
use ic_cdk_macros::*;
use ic_cron::types::{Iterations, SchedulingInterval, TaskId};
use std::collections::HashMap;

//--------DATA STRUCTURE-------
struct Caller(Principal);
struct Owner(Principal);
#[derive(CandidType, Deserialize, Debug)]
pub struct Cycle {
    pub canister_id: Principal,
}
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

#[derive(CandidType, Deserialize, Debug)]
pub struct UserData {
    pub canister_id: Principal,
    pub email: String,
    pub threshold: String,
}

#[derive(Default, Deserialize)]
pub struct CycleData {
    pub cycle: HashMap<Principal, Nat>,
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
#[update(name = "setCaller")]
#[candid_method(update, rename = "setCaller")]
fn set_caller(_caller: Principal) {
    let caller = ic_cdk::caller();
    let owner = storage::get::<Owner>();
    //assert_eq!(caller, owner.0);
    let storage = storage::get_mut::<Caller>();
    *storage = Caller(_caller);
}

#[update]
#[candid_method(update)]
async fn task_one(duration_nano: u64) -> TaskId {
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
                ic_cdk::print(format!("Task One executed: {}", message.as_str()));
                let caller = storage::get::<Caller>();
                let result: Result<(Vec<UserData>,), _> =
                    api::call::call(caller.0, "getUser", ()).await;
                let state = get_state();
                state.user_data = result.unwrap().0;
                for data in state.user_data.iter() {
                    let id = data.canister_id.clone();
                    let arg = Cycle { canister_id: id };
                    let cycle: Result<(Nat,), _> =
                        api::call::call(caller.0, "getCycle", (arg,)).await;
                    get_cycle().cycle.entry(id).or_insert(cycle.unwrap().0);

                    let arg = Cycle { canister_id: id };
                    let cycle: Result<(Nat,), _> =
                        api::call::call(caller.0, "getCycle", (arg,)).await;
                    if cycle.unwrap().0 - get_cycle().cycle.get(&id).unwrap().clone()
                        > Nat::from(10000)
                    {
                        let _: Result<(), _> = api::call::call(id, "improve_diffculty", ()).await;
                    }

                    let arg = Cycle { canister_id: id };
                    let cycle: Result<(Nat,), _> =
                        api::call::call(caller.0, "getCycle", (arg,)).await;

                    ic_cdk::print(format!("The result is {:?}", cycle.as_ref().unwrap().0));
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
