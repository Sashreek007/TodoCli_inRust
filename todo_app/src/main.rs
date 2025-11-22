use serde_json::{Value, json};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const TODO_FILE: &str = "todos.json";

#[derive(Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        show_help();
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => {
            if args.len() < 3 {
                println!("Error: 'add' requires a task description");
                return;
            }
            let description = args[2..].join(" ");
            add_task(&description);
        }
        "list" => list_tasks(),
        "complete" => {
            if args.len() < 3 {
                println!("Error: 'complete' requires a task ID");
                return;
            }
            if let Ok(id) = args[2].parse::<u32>() {
                complete_task(id);
            } else {
                println!("Error: Task ID must be a number");
            }
        }
        "remove" => {
            if args.len() < 3 {
                println!("Error: 'remove' requires a task ID");
                return;
            }
            if let Ok(id) = args[2].parse::<u32>() {
                remove_task(id);
            } else {
                println!("Error: Task ID must be a number");
            }
        }
        "clear" => clear_tasks(),
        _ => {
            println!("Unknown command: {}", command);
            show_help();
        }
    }
}

fn show_help() {
    println!("Todo App - Usage:");
    println!("  cargo run -- add <task>      Add a new task");
    println!("  cargo run -- list            List all tasks");
    println!("  cargo run -- complete <id>   Mark task as complete");
    println!("  cargo run -- remove <id>     Remove a task");
    println!("  cargo run -- clear           Clear all tasks");
}

fn add_task(description: &str) {
    match load_tasks() {
        Ok(mut tasks) => {
            let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

            let new_task = Task {
                id: next_id,
                description: description.to_string(),
                completed: false,
            };

            tasks.push(new_task);

            match save_tasks(&tasks) {
                Ok(_) => println!("Task added with ID: {}", next_id),
                Err(e) => println!("Error saving task: {}", e),
            }
        }
        Err(e) => println!("Error loading tasks: {}", e),
    }
}

fn list_tasks() {
    match load_tasks() {
        Ok(tasks) => {
            if tasks.is_empty() {
                println!("No tasks yet!");
            } else {
                for task in tasks {
                    let status = if task.completed { "[x]" } else { "[ ]" };
                    println!("{}. {} {}", task.id, status, task.description);
                }
            }
        }
        Err(e) => println!("Error loading tasks: {}", e),
    }
}

fn complete_task(id: u32) {
    match load_tasks() {
        Ok(mut tasks) => {
            // Find the task with this ID
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                if task.completed {
                    println!("Task {} is already complete!", id);
                } else {
                    task.completed = true;
                    match save_tasks(&tasks) {
                        Ok(_) => println!("Task {} marked as complete.", id),
                        Err(e) => println!("Error saving: {}", e),
                    }
                }
            } else {
                println!("Task {} not found.", id);
            }
        }
        Err(e) => println!("Error loading tasks: {}", e),
    }
}

fn remove_task(id: u32) {
    match load_tasks() {
        Ok(mut tasks) => {
            // Check if task exists
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                tasks.remove(pos);
                match save_tasks(&tasks) {
                    Ok(_) => println!("Task {} removed.", id),
                    Err(e) => println!("Error saving: {}", e),
                }
            } else {
                println!("Task {} not found.", id);
            }
        }
        Err(e) => println!("Error loading tasks: {}", e),
    }
}

fn clear_tasks() {
    print!("Are you sure? This cannot be undone. (yes/no): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "yes" {
        match save_tasks(&Vec::new()) {
            Ok(_) => println!("All tasks cleared."),
            Err(e) => println!("Error clearing tasks: {}", e),
        }
    } else {
        println!("Clear cancelled.");
    }
}

fn load_tasks() -> Result<Vec<Task>, String> {
    if !Path::new(TODO_FILE).exists() {
        return Ok(Vec::new());
    }

    let contents =
        fs::read_to_string(TODO_FILE).map_err(|e| format!("Failed to read file: {}", e))?;

    let json: Value =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let tasks_array = json.as_array().ok_or("JSON must be an array")?;

    let mut tasks = Vec::new();
    for item in tasks_array {
        let task = Task {
            id: item["id"].as_u64().ok_or("Missing or invalid id")? as u32,
            description: item["description"]
                .as_str()
                .ok_or("Missing or invalid description")?
                .to_string(),
            completed: item["completed"]
                .as_bool()
                .ok_or("Missing or invalid completed")?,
        };
        tasks.push(task);
    }

    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> Result<(), String> {
    let json_array: Vec<Value> = tasks
        .iter()
        .map(|task| {
            json!({
                "id":task.id,
                "description":task.description,
                "completed":task.completed,
            })
        })
        .collect();
    let json_string = serde_json::to_string(&json_array)
        .map_err(|e| format!("!Failed to serialise JSON: {}", e))?;

    fs::write(TODO_FILE, json_string).map_err(|e| format!("Failed to serialise JSON: {}", e))?;

    Ok(())
}
