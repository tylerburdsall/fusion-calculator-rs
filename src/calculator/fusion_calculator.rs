use std::vec;
use anyhow::Result;

use itertools::Itertools;

use crate::cli::fusion_cli::OutputType;
use crate::persona_data::data::Arcana;
use crate::persona_data::utils::{self, PersonaCombination, PersonaEntry};
use crate::terminal::table::render_table;
use crate::terminal::term::{output, output_err};

fn get_fusion<'a>(
    first: &'a str,
    second: &'a str,
) -> Result<Vec<PersonaEntry>> {
    let persona_result = if let Some(special_result) = utils::get_special_fusion(first, second) {
        special_result
    } else {
        utils::get_fusion_of(first, second)?
    };
    Ok(vec![persona_result])
}

pub fn fuse_personas(first: &str, second: &str, output_type: &OutputType) {
    match get_fusion(first, second) {
        Ok(persona_entry) => match output_type {
            OutputType::Table => {
                let table = render_table(
                    vec![
                        "Result".to_string(),
                        "Level".to_string(),
                        "Arcana".to_string(),
                    ],
                    persona_entry.to_table_output(),
                );
                output!("{table}");
            }
            OutputType::Json => {
                let json_string = persona_entry.to_json_output();
                println!("{json_string}")
            }
        },
        Err(e) => {
            output_err!("{e:?}")
        }
    }
}

pub fn fuse_possible_personas_for(persona: &str, output_type: &OutputType) {
    match utils::get_possible_fusions_from_persona(persona) {
        Ok(fusions) => match output_type {
            OutputType::Table => {
                let table = render_table(
                    vec![
                        "Fused With".to_string(),
                        "Level".to_string(),
                        "Arcana".to_string(),
                        "Result".to_string(),
                        "Level".to_string(),
                        "Arcana".to_string(),
                    ],
                    fusions.to_table_output(),
                );
                output!("{table}");
            }
            OutputType::Json => {
                let json_string = fusions.to_json_output();
                println!("{json_string}")
            }
        },
        Err(e) => {
            output_err!("{e:?}")
        }
    }
}

pub fn fusions_to(persona: &str, output_type: &OutputType) {
    if let Some(reverse_fusions) = utils::get_special_fusions_to(persona) {
        match output_type {
            OutputType::Table => {
                let table = render_table(
                    vec![
                        "Fused With".to_string(),
                        "Level".to_string(),
                        "Arcana".to_string(),
                    ],
                    reverse_fusions.to_table_output(),
                );
                output!("{table}");
            }
            OutputType::Json => {
                let json_string = reverse_fusions.to_json_output();
                println!("{json_string}")
            }
        }
        return;
    }
    match utils::get_fusions_to(persona) {
        Ok(fusions) => match output_type {
            OutputType::Table => {
                let table = render_table(
                    vec![
                        "First".to_string(),
                        "Level".to_string(),
                        "Arcana".to_string(),
                        "Second".to_string(),
                        "Level".to_string(),
                        "Aracana".to_string(),
                    ],
                    fusions.to_table_output(),
                );
                output!("{table}");
            }
            OutputType::Json => {
                let json_string = fusions.to_json_output();
                println!("{json_string}")
            }
        },
        Err(e) => {
            output_err!("{e:?}")
        }
    }
}

pub fn list_all_personas(
    arcanas: &Vec<Arcana>,
    min_level: &Option<usize>,
    max_level: &Option<usize>,
    output_type: &OutputType,
) {
    let personas = utils::get_all_personas(arcanas, min_level, max_level);
    match output_type {
        OutputType::Table => {
            let table = render_table(
                vec![
                    "Name".to_string(),
                    "Level".to_string(),
                    "Arcana".to_string(),
                ],
                personas.to_table_output(),
            );
            output!("{table}")
        }
        OutputType::Json => {
            let json_string = personas.to_json_output();
            println!("{json_string}")
        }
    }
}

trait RenderOutputType {
    fn to_table_output(&self) -> Vec<Vec<String>>;
    fn to_json_output(&self) -> String;
}

impl RenderOutputType for Vec<PersonaEntry> {
    fn to_table_output(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|entry| {
                vec![
                    entry.name.clone(),
                    entry.data.level.to_string(),
                    entry.data.arcana.to_string(),
                ]
            })
            .collect_vec()
    }

    fn to_json_output(&self) -> String {
        serde_json::to_string(&self).expect("failed to serialize Persona(s) to JSON output")
    }
}

impl RenderOutputType for Vec<PersonaCombination> {
    fn to_table_output(&self) -> Vec<Vec<String>> {
        self.iter()
            .map(|entry| {
                vec![
                    entry.first.name.clone(),
                    entry.first.data.level.to_string(),
                    entry.first.data.arcana.to_string(),
                    entry.second.name.clone(),
                    entry.second.data.level.to_string(),
                    entry.second.data.arcana.to_string(),
                ]
            })
            .collect_vec()
    }

    fn to_json_output(&self) -> String {
        serde_json::to_string(&self).expect("failed to serialize Persona(s) to JSON output")
    }
}
