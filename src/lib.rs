use std::env::Args;

pub mod file_manager;
pub mod commander;

const UNDONE_LIST_COMMAND: &str = "undone";
const FINISHED_LIST_COMMAND: &str = "finished";
const SHOW_LIST_COMMAND: &str = "list";
const ADD_TASK_COMMAND: &str = "add";
const FINISH_TASK_COMMAND: &str = "finish";
const CLEAR_TASKS_COMMAND: &str = "clear";

pub fn run(mut args: Args) {
    let command = args.next();

    if let Some(command_text) = command {
        match command_text.as_str() {
            SHOW_LIST_COMMAND => commander::print_list(&mut args),
            ADD_TASK_COMMAND => commander::add_task(&mut args),
            FINISH_TASK_COMMAND => commander::mark_task_as_finished(&mut args),
            CLEAR_TASKS_COMMAND => file_manager::clear_all_tasks(),
            _ => println!("Command {} was not found", command_text),
        }
    } else {
        println!("Type any command to proceed");
    }
}