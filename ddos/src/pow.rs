// use std::{hash::{Hash,Hasher}, collections::hash_map::DefaultHasher};
// use rand::{thread_rng, Rng};

// #[derive(Default)]
// pub struct Diffculty{
//     pub difcculty: u64
// }

// pub fn init() {
//     get_state().difcculty = 0;
// }

// pub fn increace_diffculty() {
//     get_state().difcculty += 1;
// }

// static mut STATE :Option<Diffculty> = None;

// pub fn get_state() -> &'static mut Diffculty {
//     unsafe {
//         match STATE.as_mut() {
//             Some(s) => s,
//             None => {
//                 STATE = Some(Diffculty::default());
//                 get_state()
//             }
//         }
//     }
// }
