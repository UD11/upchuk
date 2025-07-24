use clap::Parser;

use crate::{
    args::Commands,
    urls::{add_urls, print_all_urls},
};

mod args;
mod urls;

fn main() {
    let subcommands = args::MainSubcommands::parse();

    match subcommands.command {
        Commands::Add { url, tag } => {
            add_urls(&url, tag.as_deref());
        }

        Commands::Check => println!("check"),
        Commands::List => print_all_urls(),
    }
}
