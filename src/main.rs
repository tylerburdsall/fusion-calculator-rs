use cli::{
    commands::{fuse, list},
    fusion_cli::{FusionCalculatorCli, FusionCommand, OutputType},
};

mod calculator;
mod cli;
mod persona_data;
mod terminal;

fn main() {
    let args = FusionCalculatorCli::parse_arguments();
    // Default to Table output
    let output_type = match args.output_type {
        Some(output) => output,
        None => OutputType::Table,
    };
    match args.command {
        FusionCommand::Fuse { operation } => fuse::fuse_command(&operation, &output_type),
        FusionCommand::List { operation } => list::list_command(&operation, &output_type),
    }
}
