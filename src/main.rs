use cli::{
    commands::{fuse, list},
    fusion_cli::{FusionCalculatorCli, FusionCommand},
};

mod calculator;
mod cli;
mod persona_data;
mod terminal;

fn main() {
    let args = FusionCalculatorCli::parse_arguments();
    match args.command {
        FusionCommand::Fuse { operation } => fuse::fuse_command(&operation),
        FusionCommand::List { operation } => list::list_command(&operation),
    }
}
