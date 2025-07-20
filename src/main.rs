use clap::Parser;

use crate::args::Commands;

mod args;

fn main() {
    let subcommands = args::MainSubcommands::parse();

    match subcommands.command {
        Commands::Add => {
            println!("add");
        }

        Commands::Check => println!("check"),
        Commands::List => println!("list"),
    }
}
