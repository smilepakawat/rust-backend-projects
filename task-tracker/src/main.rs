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
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { description: String },
    Delete { id: i32 },
    List {},
}

#[derive(Serialize, Deserialize, Debug)]
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

    match cli.command {
        Some(Commands::Add { description }) => {
            let task = Tasks {
                id: 1,
                description,
                status: String::from("in-progress"),
            };
            list_of_task.push(task);
            if let Err(e) = write_json_to_file(path, list_of_task) {
                eprintln!("Error: {}", e)
            }
        }
        Some(Commands::Delete { id }) => {
            println!("deleting task: {:}", id)
        }
        Some(Commands::List {}) => {
            let data = match read_json_file(path) {
                Ok(tasks) => tasks,
                Err(_) => Vec::new(),
            };
            println!("List:\n{:?}", data)
        }
        None => {}
    }
}

fn write_json_to_file(path: &Path, task: Vec<Tasks>) -> Result<()> {
    let json_string = serde_json::to_string_pretty(&task)?;
    OpenOptions::new()
        .mode(0o644)
        .create(true)
        .write(true)
        .open(path)
        .unwrap()
        .write(json_string.as_bytes())?;
    Ok(())
}

fn read_json_file(path: &Path) -> Result<Vec<Tasks>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: Vec<Tasks> = serde_json::from_reader(reader)?;
    Ok(data)
}
