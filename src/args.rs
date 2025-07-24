use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct MainSubcommands {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    /// Add an url
    Add {
        url: String,

        #[arg(short, long)]
        tag: Option<String>,
    },
    /// List all added urls
    List,
    /// Check added urls
    Check,
}
