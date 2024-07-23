use crate::task::Task;
use std::fs::File;
use std::path::Path;

const TASKS_FILE: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if Path::new(TASKS_FILE).exists() {
        let file = File::open(TASKS_FILE).unwrap();
        let tasks: Vec<Task> = serde_json::from_reader(file).unwrap();
        tasks
    } else {
        Vec::new()
    }
}

pub fn save_tasks(tasks: &[Task]) {
    let file = File::create(TASKS_FILE).expect("Unable to create tasks file");
    serde_json::to_writer_pretty(file, tasks).expect("Unable to write tasks to file");
}
