use std::fs;
use std::io::{ ErrorKind };

const UNDONE_TASKS_FILE_PATH: &'static str = "undone.txt";
const FINISHED_TASKS_FILE_PATH: &'static str = "finished.txt";
const COUNTER_FILE_PATH: &'static str = "counter.txt";

fn write_to_file(file_name: &str, content: Option<&str>) {
    fs::write(file_name, String::from(content.unwrap_or(""))).unwrap_or_else(|error| {
        println!("Error creating file: {}", error);
    });
}

fn read_or_create(file_name: &str) -> String {
    fs::read_to_string(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            write_to_file(file_name, None);
        } else {
            eprintln!("Error reading file {}: {}", file_name, error);
        }

        String::from("")
    })
}

fn get_list(file_name: &str) -> Vec<String> {
    read_or_create(file_name).lines().map(String::from).collect()
}

pub fn get_undone_tasks() -> Vec<String> {
    get_list(UNDONE_TASKS_FILE_PATH)
}

pub fn get_finished_tasks() -> Vec<String> {
    get_list(FINISHED_TASKS_FILE_PATH)
}

pub fn get_all_tasks_list() -> Vec<String> {
    let mut all_tasks = vec![];

    let undone_tasks = get_undone_tasks();
    let finished_tasks = get_finished_tasks();

    if !undone_tasks.is_empty() {
        all_tasks.push(String::from("[Scheduled]"));
        all_tasks.extend(undone_tasks);
    }

    if !finished_tasks.is_empty() {
        all_tasks.push(String::from("[Finished]"));
        all_tasks.extend(finished_tasks);
    }

    if all_tasks.is_empty() {
        all_tasks.push(String::from("No tasks found"));
    }

    all_tasks
}

fn update_counter(value: usize) -> usize {
    write_to_file(COUNTER_FILE_PATH, Some(&value.to_string()));
    value
}

fn add_new_line_to_file(file_name: &str, content: String) {
    let current_content = read_or_create(file_name);
    let updated_content = if current_content.is_empty() {
        content
    } else {
        format!("{}\r\n{}", current_content, content)
    };

    write_to_file(file_name, Some(&updated_content));
}

pub fn add_new_task(text: &str, counter: usize) {
    let task_text = format!("[{}] {}", counter, text);
    add_new_line_to_file(UNDONE_TASKS_FILE_PATH, task_text);
}

pub fn remove_task(file_name: &str, task_id: usize) -> Result<String, ()> {
    let file_content = read_or_create(file_name);
    let task_to_remove = file_content.lines().find(|line| line.contains(&task_id.to_string()));

    if let Some(task) = task_to_remove {
        let filtered_content = file_content
            .lines()
            .filter(|line| line != &task)
            .collect::<Vec<_>>()
            .join("\r\n");

        write_to_file(file_name, Some(&filtered_content));
        Ok(String::from(task))
    } else {
        println!("Error deleting task [{}], probably it was removed before", task_id);
        Err(())
    }
}

pub fn finish_task(task_id: usize) {
    if let Ok(task) = remove_task(UNDONE_TASKS_FILE_PATH, task_id) {
        add_new_line_to_file(FINISHED_TASKS_FILE_PATH, task);
        println!("Task [{}] marked as finished.", task_id);
    }
}


fn get_counter() -> usize {
    read_or_create(COUNTER_FILE_PATH)
        .trim()
        .parse::<usize>()
        .unwrap_or_else(|_| {
            update_counter(0)
        })
}

pub fn increment_counter() -> usize {
    let counter = get_counter();
    update_counter(counter + 1)
}

pub fn clear_all_tasks() {
    write_to_file(UNDONE_TASKS_FILE_PATH, None);
    write_to_file(FINISHED_TASKS_FILE_PATH, None);
    write_to_file(COUNTER_FILE_PATH, None);
}
