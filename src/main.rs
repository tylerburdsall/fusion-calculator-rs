mod calculator;
mod persona_data;
mod terminal;

fn main() {
    calculator::fusion_calculator::fuse_personas("Thor", "Incubus");
    calculator::fusion_calculator::fuse_possible_personas_for("Thor");
    calculator::fusion_calculator::reverse_fuse_personas_to("Thor")
}
