use anyhow::Result;
use clap::Parser;

use crate::{client_rust, client_java, server};

#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand, Debug)]
enum Commands {
    Server,

    ClientRust,

    ClientJava,
}

pub fn run() -> Result<()> {
    let cli = Args::parse();

    let result = match cli.command {
        Commands::Server => server::run(),
        Commands::ClientRust => client_rust::run(),
        Commands::ClientJava => client_java::run()
    };

    if let Err(e) = &result {
        println!("Error: {}", e);
    }

    result
}
