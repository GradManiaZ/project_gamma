

use std::env;

mod time_scheduler;

#[allow(unused_variables)]
fn main() {
    let mut args: Vec<String> = env::args().skip(1).rev().collect();
    time_scheduler::wrapper(&mut args);
}
