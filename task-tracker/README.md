# Task Tracker

A simple command-line task management application written in Rust that helps you organize and track your tasks with different statuses.

## Installation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Build from Source

1. Clone or download this repository
2. Navigate to the project directory:
   ```bash
   cd task-tracker
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. The executable will be available at `target/release/task-tracker`

### Run without Installation

You can also run the application directly with Cargo:
```bash
cargo run -- [COMMAND]
```

## Usage

The task tracker supports the following commands:

### Add a New Task

```bash
task-tracker add "Your task description"
```

Example:
```bash
task-tracker add "Complete the project documentation"
```

### List All Tasks

```bash
task-tracker list
```

This displays all tasks in the format: `ID DESCRIPTION STATUS`

### Update a Task

You can update either the description, status, or both:

```bash
# Update description only
task-tracker update 1 --description "Updated task description"

# Update status only
task-tracker update 1 --status "In-progress"

# Update both description and status
task-tracker update 1 --description "New description" --status "Success"
```

#### Available Status Values

- `Todo` - Task is pending (default for new tasks)
- `In-progress` - Task is currently being worked on
- `Success` - Task has been completed

### Delete a Task

```bash
task-tracker delete 1
```

Replace `1` with the actual task ID you want to delete.

## Examples

Here's a typical workflow:

```bash
# Add some tasks
task-tracker add "Buy groceries"
task-tracker add "Write unit tests"
task-tracker add "Deploy to production"

# List all tasks
task-tracker list
# Output:
# 1 Buy groceries Todo
# 2 Write unit tests Todo  
# 3 Deploy to production Todo

# Start working on a task
task-tracker update 2 --status "In-progress"

# Complete a task
task-tracker update 1 --status "Success"

# Update task description
task-tracker update 3 --description "Deploy v1.0 to production"

# List updated tasks
task-tracker list
# Output:
# 1 Buy groceries Success
# 2 Write unit tests In-progress
# 3 Deploy v1.0 to production Todo

# Delete completed task
task-tracker delete 1
```

## Data Storage

Tasks are automatically saved to a `datastore.json` file in the current working directory. This file is created automatically when you add your first task.

The JSON structure looks like this:
```json
[
  {
    "id": 1,
    "description": "Your task description",
    "status": "Todo"
  }
]
```

## Command Reference

| Command | Description | Example |
|---------|-------------|---------|
| `add <description>` | Add a new task | `task-tracker add "New task"` |
| `list` | Show all tasks | `task-tracker list` |
| `update <id> [options]` | Update existing task | `task-tracker update 1 --status "Success"` |
| `delete <id>` | Delete a task | `task-tracker delete 1` |

### Update Command Options

- `-d, --description <DESCRIPTION>` - Update task description
- `-s, --status <STATUS>` - Update task status (Todo, In-progress, Success)
