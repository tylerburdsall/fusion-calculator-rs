use crate::{calculator::fusion_calculator, cli::fusion_cli::OutputType};

#[derive(clap::Parser)]
pub enum ListCommand {
    #[command(about = "List all Personas")]
    Personas {
        #[arg(long = "arcanas", help = "List of Arcanas to filter by")]
        arcanas: Vec<String>,
        #[arg(
            long = "min-level",
            help = "Only display Peronas at or above the minimum level given"
        )]
        min_level: Option<usize>,
        #[arg(
            long = "max-level",
            help = "Only display Personas at or below the maximum level given"
        )]
        max_level: Option<usize>,
    },
}

pub fn list_command(command: &ListCommand, output_type: &OutputType) {
    match command {
        ListCommand::Personas {
            arcanas,
            min_level,
            max_level,
        } => fusion_calculator::list_all_personas(arcanas, min_level, max_level, output_type),
    }
}
