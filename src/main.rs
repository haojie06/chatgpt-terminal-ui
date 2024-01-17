mod client;
mod openai;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ask {},
    Test {
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let arg = Args::parse();

    match arg.command {
        Some(Commands::Ask {}) => {
            println!("ask");
        }
        Some(Commands::Test { list }) => {
            println!("test {}", list);
        }
        None => {
            println!("none");
        }
    }
}
