mod file_handler;
mod task;
mod task_manager;

use structopt::StructOpt;
use task_manager::TaskManager;

#[derive(StructOpt)]
#[structopt(name = "task_manager", about = "A simple task manager")]
enum Cli {
    #[structopt(about = "Add a new task")]
    Add { title: String, description: String },
    #[structopt(about = "List all tasks")]
    List,
    #[structopt(about = "Mark a task as completed")]
    Complete { id: u32 },
    #[structopt(about = "Remove a task")]
    Remove { id: u32 },
}

fn main() {
    let args = Cli::from_args();
    let mut task_manager = TaskManager::new();

    match args {
        Cli::Add { title, description } => task_manager.add_task(title, description),
        Cli::List => task_manager.list_tasks(),
        Cli::Complete { id } => task_manager.complete_task(id),
        Cli::Remove { id } => task_manager.remove_task(id),
    }
}
