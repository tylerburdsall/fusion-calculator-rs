use cli::{fusion_cli::{FusionCalculatorCli, FusionCommand}, commands};

mod calculator;
mod persona_data;
mod terminal;
mod cli;


fn main() {
    let args = FusionCalculatorCli::parse_arguments();
}
