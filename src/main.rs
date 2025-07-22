use clap::Parser;

use crate::{args::Commands, urls::add_urls};

mod args;
mod urls;

fn main() {
    let subcommands = args::MainSubcommands::parse();

    match subcommands.command {
        Commands::Add => {
            add_urls("hard_Coded_test.com");
        }

        Commands::Check => println!("check"),
        Commands::List => println!("list"),
    }
}
