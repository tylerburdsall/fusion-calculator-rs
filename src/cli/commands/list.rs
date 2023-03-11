use crate::{calculator::fusion_calculator, persona_data::data::Arcana};

#[derive(clap::Parser)]
pub enum ListCommand {
    #[command(about = "List all Personas")]
    Personas {
        #[arg(long = "arcana", help = "List of Arcanas to filter by")]
        arcana: Vec<Arcana>,
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

pub fn list_command(command: &ListCommand) {
    match command {
        ListCommand::Personas {
            arcana,
            min_level,
            max_level,
        } => fusion_calculator::list_all_personas(arcana, min_level, max_level),
    }
}
