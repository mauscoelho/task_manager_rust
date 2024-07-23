use crate::file_handler::{load_tasks, save_tasks};
use crate::task::Task;

pub struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        TaskManager {
            tasks: load_tasks(),
        }
    }

    pub fn add_task(&mut self, title: String, description: String) {
        let id = self.tasks.len() as u32;
        let task = Task {
            id,
            title,
            description,
            completed: false,
        };
        self.tasks.push(task);
        save_tasks(&self.tasks);
        println!("Task added successfully");
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks found");
        } else {
            for task in &self.tasks {
                println!(
                    "[{}] {}. {} - {}",
                    if task.completed { "x" } else { " " },
                    task.id,
                    task.title,
                    task.description
                );
            }
        }
    }

    pub fn complete_task(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            save_tasks(&self.tasks);
            println!("Task completed successfully");
        } else {
            println!("Task not found");
        }
    }

    pub fn remove_task(&mut self, id: u32) {
        if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
            self.tasks.remove(pos);
            save_tasks(&self.tasks);
            println!("Task removed successfully");
        } else {
            println!("Task not found");
        }
    }
}
