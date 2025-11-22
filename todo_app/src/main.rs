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
}

fn show_help() {
    println!("Todo App - Usage:");
    println!("  cargo run -- add <task>      Add a new task");
    println!("  cargo run -- list            List all tasks");
    println!("  cargo run -- complete <id>   Mark task as complete");
    println!("  cargo run -- remove <id>     Remove a task");
    println!("  cargo run -- clear           Clear all tasks");
}
