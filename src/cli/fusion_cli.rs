use clap::Parser;

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
}