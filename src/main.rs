mod client;
mod openai;

use std::{fs::File, io::Read, path::PathBuf};

use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    openai_key: String,
    openai_base_url: Option<String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.toml")]
    config: PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Ask {
        #[arg()]
        content: String,
    },
    Test {
        #[arg(short, long)]
        list: bool,
    },
}

#[tokio::main]
async fn main() {
    let arg = Args::parse();
    println!("use config file: {:?}", arg.config);
    let mut config_file = File::open(arg.config).expect("open config file");
    let mut contents = String::new();
    config_file
        .read_to_string(&mut contents)
        .expect("read config file");
    let config: Config = toml::from_str(&contents).expect("parse config file");
    let completion_client = client::Client::new(openai::OpenAIClientConfig {
        api_key: config.openai_key,
        base_url: config.openai_base_url,
    });

    match arg.command {
        Some(Commands::Ask { content }) => {
            println!("ask {}", content);
            let mut result_reader = completion_client
                .ask(content, openai::CompletionModel::GPT3_5Turbo)
                .await
                .unwrap();
            while let Some(content) = result_reader.next_content().await.unwrap() {
                println!("{content}");
            }
        }
        Some(Commands::Test { list }) => {
            println!("test {}", list);
        }
        None => {
            println!("none");
        }
    }
}
