use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { task: String },
    List {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { task }) => {
            println!("adding task: {:}", task)
        }
        Some(Commands::List {}) => {
            println!("list task:")
        }
        None => {}
    }
}
