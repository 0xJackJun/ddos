use rand::{thread_rng, Rng};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

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
