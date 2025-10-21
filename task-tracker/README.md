# Task Tracker

Sample solution for the [task-tracker](https://roadmap.sh/projects/task-tracker) challenge from [roadmap.sh](https://roadmap.sh/).

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

### List Tasks

The list command now supports filtering by status:

```bash
# List all tasks
task-tracker list

# List only Todo tasks
task-tracker list --todo

# List only In-progress tasks
task-tracker list --in-progress

# List only Done tasks
task-tracker list --done
```

**Note**: You can specify one of the filter options (`--todo`, `--in-progress`, or `--done`) when using the list command.

### Update a Task

You can update either the description, status, or both:

```bash
# Update description only
task-tracker update 1 --description "Updated task description"

# Update status only
task-tracker update 1 --status "in-progress"

# Update both description and status
task-tracker update 1 --description "New description" --status "done"
```

#### Available Status Values

- `todo` - Task is pending (default for new tasks)
- `in-progress` - Task is currently being worked on
- `done` - Task has been completed

### Delete a Task

```bash
task-tracker delete 1
```

Replace `1` with the actual task ID you want to delete.

### Quickly update task status

```bash
# Quickly update status to in-progress
task-tracker mark-in-progress 1

# Quickly update status to done
task-tracker mark-done 1
```

Replace `1` with the actual task ID you want to update.


## Examples

Here's a typical workflow:

```bash
# Add some tasks
task-tracker add "Buy groceries"
task-tracker add "Write unit tests"
task-tracker add "Deploy to production"

# Start working on a task
task-tracker update 2 --status "in-progress"

# Complete a task
task-tracker update 1 --status "done"

# Update task description
task-tracker update 3 --description "Deploy v1.0 to production"

# List all tasks
task-tracker list
# Output:
# 1 Buy groceries todo
# 2 Write unit tests todo  
# 3 Deploy to production todo

# List tasks by status
task-tracker list --done
# Output:
# 1 Buy groceries done

task-tracker list --in-progress
# Output:
# 2 Write unit tests in-progress

task-tracker list --todo
# Output:
# 3 Deploy v1.0 to production todo

# Delete completed task
task-tracker delete 1

# Quickly update status to in-progress
task-tracker mark-in-progress 1

# Quickly update status to done
task-tracker mark-done 1
```

## Data Storage

Tasks are automatically saved to a `datastore.json` file in the current working directory. This file is created automatically when you add your first task.

The JSON structure looks like this:
```json
[
  {
    "id": 1,
    "description": "Your task description",
    "status": "todo",
    "created_at": "2025-10-15T14:21:25.946645+07:00",
    "updated_at": "2025-10-15T14:22:20.076831+07:00"
  }
]
```

## Command Reference

| Command | Description | Example |
|---------|-------------|---------|
| `add <description>` | Add a new task | `task-tracker add "New task"` |
| `list [filter]` | Show tasks with status filter | `task-tracker list` |
| `update <id> [options]` | Update existing task | `task-tracker update 1 --status "done"` |
| `delete <id>` | Delete a task | `task-tracker delete 1` |
| `mark-in-progress <id>` | Update status to in-progress | `task-tracker mark-in-progress 1` |
| `mark-done <id>` | Update status to done | `task-tracker mark-done 1` |

### List Command Options

- `-t, --todo` - Show only todo tasks
- `-i, --in-progress` - Show only in-progress tasks
- `-d, --done` - Show only done tasks

### Update Command Options

- `-d, --description <DESCRIPTION>` - Update task description
- `-s, --status <STATUS>` - Update task status (todo, in-progress, done)
