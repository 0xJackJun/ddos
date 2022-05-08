use crate::controller::{Caller, Owner};
use crate::database::{get_cycle, get_state, UserData};
use candid::{candid_method, CandidType, Deserialize, Nat, Principal};
use ic_cdk::{api, storage, trap};
use ic_cdk_macros::*;
use ic_cron::types::{Iterations, SchedulingInterval, TaskId};

ic_cron::implement_cron!();

#[derive(CandidType, Deserialize, Debug)]
pub struct Cycle {
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum CronTaskKind {
    One(String),
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
                    let gap = cycle.unwrap().0 - get_cycle().cycle.get(&id).unwrap().clone();

                    let arg = Cycle { canister_id: id };
                    let cycle: Result<(Nat,), _> =
                        api::call::call(caller.0, "getCycle", (arg,)).await;
                    get_cycle().cycle.insert(id, cycle.unwrap().0.clone());
                    if gap > Nat::from(354000000) {
                        let _: Result<(), _> = api::call::call(id, "increase_diffculty", ()).await;
                        if gap < Nat::from(70800000) {
                            let _: Result<(), _> = api::call::call(id, "initilize", ()).await;
                        }
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
