use std::process::Output;

use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
pub enum OutputType {
    Table,
    Json,
}

#[derive(clap::Parser)]
#[command(
    version,
    bin_name = "fusion-calculator",
    name = "fusion-calculator",
    about = "Simple CLI fusion calculator for Persona 5 Royal"
)]
pub struct FusionCalculatorCli {
    #[clap(subcommand)]
    pub command: FusionCommand,

    #[arg(long, global = true, required = false)]
    pub output_type: Option<OutputType>,
}

impl FusionCalculatorCli {
    pub fn parse_arguments() -> Self {
        FusionCalculatorCli::parse()
    }
}

#[derive(clap::Parser)]
pub enum FusionCommand {
    #[command(about = "Perform fusion-related commands")]
    Fuse {
        #[command(subcommand)]
        operation: super::commands::fuse::FuseCommand,
    },
    #[command(about = "List info- and data-related commands")]
    List {
        #[command(subcommand)]
        operation: super::commands::list::ListCommand,
    },
}
