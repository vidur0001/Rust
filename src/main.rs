use std::fs::{read_to_string, write};
use std::io::{self, Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: i32,
    name: String,
    status: TaskStatus,
    description: String,
    deadline: String,
    priority: String,
}

fn main() {
    let file_path = "tasks.json";
    let mut all_tasks = load_tasks_from_file(file_path);

    println!("📋 Welcome to RustyTasks: A Simple CLI Task Manager");

    loop {
        println!("\n1. Create a new task");
        println!("2. View all tasks");
        println!("3. Edit a task");
        println!("4. Delete a task");
        println!("5. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                let task = create_task();
                all_tasks.push(task);
                println!("✅ Task added!");
            }
            "2" => {
                println!("\n🗂️  All Tasks:");
                for task in &all_tasks {
                    println!("{:#?}", task);
                }
            }
            "3" => {
                let id_input = read_input("Enter Task ID to edit: ");
                let id = id_input.parse::<i32>().unwrap_or(-1);
                edit_task(&mut all_tasks, id);
            }
            "4" => {
                let id_input = read_input("Enter Task ID to delete: ");
                let id = id_input.parse::<i32>().unwrap_or(-1);
                delete_task(&mut all_tasks, id);
            }
            "5" => {
                save_tasks_to_file(file_path, &all_tasks);
                println!("💾 Tasks saved to '{}'. Goodbye!", file_path);
                break;
            }
            _ => {
                println!("❌ Invalid choice. Please try again.");
            }
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn create_task() -> Task {
    let id = read_input("Enter task ID: ").parse::<i32>().expect("ID must be a number");
    let name = read_input("Enter task name: ");
    let status_input = read_input("Enter task status (Pending/Completed): ");
    let status = match status_input.to_lowercase().as_str() {
        "pending" => TaskStatus::Pending,
        "completed" => TaskStatus::Completed,
        _ => {
            println!("Invalid status. Defaulting to Pending.");
            TaskStatus::Pending
        }
    };
    let description = read_input("Enter task description: ");
    let deadline = read_input("Enter task deadline: ");
    let priority = read_input("Enter task priority: ");

    Task {
        id,
        name,
        status,
        description,
        deadline,
        priority,
    }
}

fn edit_task(tasks: &mut Vec<Task>, id: i32) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        println!("\n✏️ Editing Task ID {}:", id);
        task.name = read_input("Enter new name: ");
        let status_input = read_input("Enter new status (Pending/Completed): ");
        task.status = match status_input.to_lowercase().as_str() {
            "pending" => TaskStatus::Pending,
            "completed" => TaskStatus::Completed,
            _ => {
                println!("Invalid status. Keeping existing.");
                task.status.clone()
            }
        };
        task.description = read_input("Enter new description: ");
        task.deadline = read_input("Enter new deadline: ");
        task.priority = read_input("Enter new priority: ");
        println!("✅ Task updated!");
    } else {
        println!("❌ Task with ID {} not found.", id);
    }
}

fn delete_task(tasks: &mut Vec<Task>, id: i32) {
    let initial_len = tasks.len();
    tasks.retain(|task| task.id != id);
    if tasks.len() < initial_len {
        println!("🗑️ Task ID {} deleted.", id);
    } else {
        println!("❌ Task with ID {} not found.", id);
    }
}

fn load_tasks_from_file(path: &str) -> Vec<Task> {
    match read_to_string(path) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_else(|_| Vec::new()),
        Err(_) => Vec::new(), // file doesn't exist yet
    }
}

fn save_tasks_to_file(path: &str, tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    write(path, data).expect("Failed to write tasks to file");
}
