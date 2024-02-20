#![allow(non_snake_case)]
mod cli;
mod client;
mod server;

fn main() -> anyhow::Result<()> {
    cli::run()
}
