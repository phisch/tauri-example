use dotenv::dotenv;

use clap::{Parser, Subcommand};

extern crate pretty_env_logger;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(about = "run the pile webserver")]
    Server {
        #[clap(short, long, value_parser)]
        port: Option<u16>,
    },
    #[clap(about = "run the pile user interface as a windowed client")]
    Client {},
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let args = Cli::parse();

    match args.command {
        Commands::Client {  } => {
            client::run();
        }
        Commands::Server { port } => {
            web_server::run(port).await;
        }
    }
}