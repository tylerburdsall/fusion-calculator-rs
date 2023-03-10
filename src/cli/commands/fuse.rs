use crate::calculator::fusion_calculator;

#[derive(clap::Parser)]
pub enum FuseCommand {
    #[command(about = "Fuse two personas together")]
    Calculate {
        #[arg(long = "first")]
        first: String,
        #[arg(long = "second")]
        second: String,
    },
    #[command(about = "Show possible fusions with this persona")]
    From {
        #[arg(long = "name")]
        name: String,
    },
    #[command(about = "Show possible fusions resulting in this persona")]
    To {
        #[arg(long = "name")]
        name: String,
    },
}

pub fn fuse_command(command: &FuseCommand) {
    match command {
        FuseCommand::Calculate { first, second } => fusion_calculator::fuse_personas(first, second),
        FuseCommand::From { name } => fusion_calculator::fuse_possible_personas_for(name),
        FuseCommand::To { name } => fusion_calculator::fusions_to(name),
    }
}
