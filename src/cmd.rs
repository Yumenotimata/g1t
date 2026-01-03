use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: SubCommand,
}

#[derive(Subcommand, Debug)]
pub enum SubCommand {
    Init,
    Touch { path: String },
}
