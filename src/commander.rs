use std::env::Args;
use crate::{file_manager, FINISHED_LIST_COMMAND, UNDONE_LIST_COMMAND};

pub fn print_list(args: &mut Args) {
    let tasks_to_display = match args.next().as_deref() {
        Some(UNDONE_LIST_COMMAND) => file_manager::get_undone_tasks(),
        Some(FINISHED_LIST_COMMAND) => file_manager::get_finished_tasks(),
        _ => file_manager::get_all_tasks_list()
    };

    for task in tasks_to_display {
        println!("{}", task);
    }
}

pub fn add_task(args: &mut Args) {
    if let Some(content) = args.next() {
        let task_id = file_manager::increment_counter();
        file_manager::add_new_task(&content, task_id);
    } else {
        println!("Failed to add new task, please type text");
    }
}

pub fn mark_task_as_finished(args: &mut Args) {
    if let Some(id_of_task) = args.next() {
        match id_of_task.parse::<usize>() {
            Ok(task_id) => file_manager::finish_task(task_id),
            Err(_) => println!("Invalid task ID: {}", id_of_task),
        }
    } else {
        println!("Failed to mark task as finished");
    }
}