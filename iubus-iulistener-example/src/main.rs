#![allow(non_snake_case)]
mod cli;
mod server;
mod client_rust;
mod client_java;

fn main() -> anyhow::Result<()>{
    cli::run()
}