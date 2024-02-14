use anyhow::Result;
use clap::Parser;

use crate::{client};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Client,
}

pub fn run() -> Result<()> {
    let cli = Args::parse();

    let result = match cli.command {
        Commands::Client => client::run(),
    };

    if let Err(e) = &result {
        println!("Error: {}", e);
    }

    result
}
