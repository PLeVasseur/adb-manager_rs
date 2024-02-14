#![allow(non_snake_case)]
mod cli;
mod client;

fn main() -> anyhow::Result<()>{
    cli::run()
}