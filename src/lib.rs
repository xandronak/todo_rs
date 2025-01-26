use std::env::Args;

pub mod file_manager;
pub mod commander;

const UNDONE_LIST_COMMAND: &str = "undone";
const FINISHED_LIST_COMMAND: &str = "finished";

pub fn run(mut args: Args) {
    let command = args.next();

    if let Some(command_text) = command {
        match command_text.as_str() {
            "list" => commander::print_list(&mut args),
            "add" => commander::add_task(&mut args),
            "finish" => commander::mark_task_as_finished(&mut args),
            "clear" => file_manager::clear_all_tasks(),
            _ => println!("Command {} was not found", command_text),
        }
    } else {
        println!("Type any command to proceed");
    }
}