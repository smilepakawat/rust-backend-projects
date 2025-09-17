use serde_json;
use std::io::{BufReader, Result, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::{
    fs::{File, OpenOptions},
    path::Path,
};

use clap::{Parser, Subcommand};
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
        id: i32,
    },
    Update {
        id: i32,
        #[arg(short, long, required = false)]
        description: Option<String>,
        #[arg(short, long, required = false)]
        status: Option<String>,
    },
    List {},
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Tasks {
    id: i32,
    description: String,
    status: String,
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
            let task = Tasks {
                id: incrementor,
                description,
                status: String::from("todo"),
            };
            list_of_task.push(task.clone());
            if let Err(e) = write_json_to_file(path, list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(task)
        }
        Some(Commands::Delete { id }) => {
            println!("deleting task: {:}", id);
            let index = list_of_task.binary_search_by(|t| t.id.cmp(&id));
            let task = list_of_task.remove(index.unwrap());
            if let Err(e) = write_json_to_file(path, list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(task)
        }
        Some(Commands::Update {
            id,
            description,
            status,
        }) => {
            let des = description.unwrap_or("".to_string());
            let sta = status.unwrap_or("".to_string());
            let index = list_of_task.binary_search_by(|t| t.id.cmp(&id)).unwrap();
            if des != "" {
                list_of_task[index].description = des.to_string();
            }
            if sta != "" {
                list_of_task[index].status = sta.to_string();
            }
            let task = list_of_task[index].clone();
            if let Err(e) = write_json_to_file(path, list_of_task) {
                eprintln!("Error: {}", e)
            }
            print_task(task);
        }
        Some(Commands::List {}) => {
            let data = match read_json_file(path) {
                Ok(tasks) => tasks,
                Err(_) => Vec::new(),
            };
            print_list_of_task(data)
        }
        None => {}
    }
}

fn write_json_to_file(path: &Path, task: Vec<Tasks>) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&task)?;
    println!("test {}", json_string);
    OpenOptions::new()
        .mode(0o644)
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?
        .write(json_string.as_bytes())?;
    Ok(())
}

fn read_json_file(path: &Path) -> Result<Vec<Tasks>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: Vec<Tasks> = serde_json::from_reader(reader)?;
    Ok(data)
}

fn print_list_of_task(lot: Vec<Tasks>) {
    for t in lot {
        print_task(t)
    }
}

fn print_task(t: Tasks) {
    println!("{} {} {}", t.id, t.description, t.status)
}
