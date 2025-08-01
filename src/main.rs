use clap::Parser;

use crate::{
    args::Commands,
    urls::{add_urls, check_all_urls, print_all_urls},
};

mod args;
mod urls;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subcommands = args::MainSubcommands::parse();

    match subcommands.command {
        Commands::Add { url, tag } => {
            add_urls(&url, tag.as_deref())?;
        }

        Commands::Check => check_all_urls()?,
        Commands::List => print_all_urls()?,
    }
    Ok(())
}
