
mod time_scheduler;
use time_scheduler::{Task,Project};
use std::env;

fn main() {
    // println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let mut project = Project::new();
    // project.add_task(Task::new(1.0,2.0,String::from("Long Tail..\nThe end"),String::from("Pew")));
    // project.add_task(Task::new(2.0,4.0,String::from("Long Tail..\nThe end"),String::from("Pew")));
    // project.add_task(Task::new(3.0,8.0,String::from("Long Tail..\nThe end"),String::from("Pew")));
    // project.add_task(Task::new(4.0,16.0,String::from("Long Tail..\nThe end"),String::from("Pew")));
    // project.display_tasks();

    time_scheduler::project_menu(args)

}
