use chrono::{DateTime, Local};
use serde_json;
use std::io::{BufReader, Result, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(arg_required_else_help = true, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        description: String,
    },
    Delete {
        id: u32,
    },
    Update {
        id: u32,
        #[arg(short, long, required = false)]
        description: Option<String>,
        #[arg(short, long, required = false)]
        status: Option<String>,
    },
    MarkInProgress {
        id: u32,
    },
    MarkDone {
        id: u32,
    },
    List(FilterArgs),
}

#[derive(Args, Debug)]
struct FilterArgs {
    #[arg(short, long, required = false)]
    todo: bool,
    #[arg(short, long, required = false)]
    done: bool,
    #[arg(short, long, required = false)]
    in_progress: bool,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn get(&self) -> String {
        match self {
            Self::Todo => String::from("todo"),
            Self::InProgress => String::from("in-progress"),
            Self::Done => String::from("done"),
        }
    }

    fn verify(status: &String) -> bool {
        match status.as_str() {
            "todo" => true,
            "in-progress" => true,
            "done" => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: u32,
    description: String,
    status: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl Task {
    fn new(id: u32, description: String) -> Self {
        Self {
            id,
            description,
            status: Status::Todo.get(),
            created_at: Local::now(),
            updated_at: Local::now(),
        }
    }

    fn update(&mut self, description: &String, status: &String) -> Self {
        Self {
            id: self.id,
            description: if description.is_empty() {
                self.description.clone()
            } else {
                description.to_string()
            },
            status: if status.is_empty() {
                self.status.clone()
            } else {
                status.to_string()
            },
            created_at: self.created_at,
            updated_at: Local::now(),
        }
    }

    fn update_status(&mut self, status: &String) -> Self {
        Self {
            id: self.id,
            description: self.description.clone(),
            status: status.to_string(),
            created_at: self.created_at,
            updated_at: Local::now(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new("datastore.json");

    let mut list_of_task = match read_json_file(path) {
        Ok(lot) => lot,
        Err(_) => Vec::new(),
    };
    let incrementor = match list_of_task.is_empty() {
        true => 1,
        false => list_of_task.last().unwrap().id + 1,
    };

    match cli.command {
        Some(Commands::Add { description }) => {
            let task = Task::new(incrementor, description);
            list_of_task.push(task.clone());
            if let Err(e) = write_json_to_file(path, &list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(&task)
        }
        Some(Commands::Delete { id }) => {
            let index = get_index(&list_of_task, id);
            let task = list_of_task.remove(index);
            if let Err(e) = write_json_to_file(path, &list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(&task)
        }
        Some(Commands::Update {
            id,
            description,
            status,
        }) => {
            let unwrap_status = status.clone().unwrap_or_default();
            let unwrap_description = description.clone().unwrap_or_default();
            if unwrap_status != "" && !Status::verify(&unwrap_status) {
                eprintln!("Error: Status must be todo, in-progress or done");
                return;
            }
            let index = get_index(&list_of_task, id);
            list_of_task[index] = list_of_task[index].update(&unwrap_description, &unwrap_status);
            if let Err(e) = write_json_to_file(path, &list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(&list_of_task[index]);
        }
        Some(Commands::MarkInProgress { id }) => {
            let index = get_index(&list_of_task, id);
            list_of_task[index] = list_of_task[index].update_status(&Status::InProgress.get());
            if let Err(e) = write_json_to_file(path, &list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(&list_of_task[index]);
        }
        Some(Commands::MarkDone { id }) => {
            let index = get_index(&list_of_task, id);
            list_of_task[index] = list_of_task[index].update_status(&Status::Done.get());
            if let Err(e) = write_json_to_file(path, &list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(&list_of_task[index]);
        }
        Some(Commands::List(args)) => {
            if let Some(status) = match () {
                _ if args.todo => Some(Status::Todo),
                _ if args.in_progress => Some(Status::InProgress),
                _ if args.done => Some(Status::Done),
                _ => None,
            } {
                print_tasks_by_status(&list_of_task, status.get());
            } else {
                print_tasks(&list_of_task);
            }
        }
        None => {}
    }
}

fn write_json_to_file(path: &Path, task: &Vec<Task>) -> Result<()> {
    let json_string = serde_json::to_string_pretty(task)?;
    OpenOptions::new()
        .mode(0o644)
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?
        .write(json_string.as_bytes())?;
    Ok(())
}

fn read_json_file(path: &Path) -> Result<Vec<Task>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: Vec<Task> = serde_json::from_reader(reader)?;
    Ok(data)
}

fn print_tasks_by_status(list_of_task: &Vec<Task>, status: String) {
    for task in list_of_task {
        if task.status == status {
            print_task(&task)
        }
    }
}

fn print_tasks(list_of_task: &Vec<Task>) {
    for task in list_of_task {
        print_task(&task)
    }
}

fn print_task(task: &Task) {
    println!("{} {} {} {} {}", task.id, task.description, task.status, task.created_at, task.updated_at)
}

fn get_index(list_of_task: &Vec<Task>, id: u32) -> usize {
    list_of_task.binary_search_by(|t| t.id.cmp(&id)).unwrap()
}
