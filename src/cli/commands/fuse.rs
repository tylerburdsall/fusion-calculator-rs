use crate::{calculator::fusion_calculator, cli::fusion_cli::OutputType};

#[derive(clap::Parser)]
pub enum FuseCommand {
    #[command(about = "Fuse two Personas together")]
    Calculate {
        #[arg(long = "first")]
        first: String,
        #[arg(long = "second")]
        second: String,
    },
    #[command(about = "Show possible fusions with this Persona")]
    From {
        #[arg(long = "name")]
        name: String,
    },
    #[command(about = "Show possible fusions resulting in this Persona")]
    To {
        #[arg(long = "name")]
        name: String,
    },
}

pub fn fuse_command(command: &FuseCommand, output_type: &OutputType) {
    match command {
        FuseCommand::Calculate { first, second } => {
            fusion_calculator::fuse_personas(first, second, output_type)
        }
        FuseCommand::From { name } => {
            fusion_calculator::fuse_possible_personas_for(name, output_type)
        }
        FuseCommand::To { name } => fusion_calculator::fusions_to(name, output_type),
    }
}
