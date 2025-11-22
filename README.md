# Todo App

A simple command-line todo list application written in Rust. Manage your tasks from the terminal with persistent storage.

## Building

Make sure you have Rust 1.70 or later installed. If not, install it from [rustup.rs](https://rustup.rs/).

```bash
cargo build --release
```
## Running

### Add a task
```bash
cargo run -- add "Your task description here"
```

Example:
```bash
cargo run -- add "Buy groceries"
```

### List all tasks
```bash
cargo run -- list
```

Output:
```
1. [ ] Buy groceries
2. [x] Complete assignment
3. [ ] Call dentist
```

### Mark a task as complete
```bash
cargo run -- complete <id>
```

Example:
```bash
cargo run -- complete 1
```

### Remove a task
```bash
cargo run -- remove <id>
```

Example:
```bash
cargo run -- remove 2
```

### Clear all tasks
```bash
cargo run -- clear
```

You'll be prompted to confirm:
```
Are you sure? This cannot be undone. (yes/no)
> yes
All tasks cleared.
```

## Data Storage

All tasks are automatically saved to `todos.json` in your current working directory. This file is created automatically the first time you add a task.

## Project Structure

```
src/
├── main.rs          
```

## Running Tests

```bash
cargo test
```

## Notes

- Task IDs are unique and never reused, even if a task is deleted
- The application handles missing or corrupted `todos.json` files gracefully
- All commands provide clear error messages if something goes wrong
