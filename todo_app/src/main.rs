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
                println!("Error: 'add' required a task description");
                return;
            }
            let description = args[2..].join(" ");
            add_task(&description);
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
    match load_tasks() {}
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
