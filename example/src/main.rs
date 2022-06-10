use ddos::implement_ddos;
implement_ddos!();

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

#[query]
#[candid_method]
fn hello() -> String {
    ic_cdk::print("Hello World!");
    return "Hello World!".to_string();
}

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

//----------GENERATE CANDID FILE------------
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    candid::export_service!();
    std::print!("{}", __export_service());
}
