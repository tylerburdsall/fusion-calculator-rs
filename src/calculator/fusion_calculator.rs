use std::vec;

use crate::persona_data::utils;
use crate::terminal::table::render_table;
use crate::terminal::term::{output_err, output};

pub fn fuse_personas(first: &str, second: &str) {
    match utils::get_fusion_of(first, second) {
        Ok(persona_result) => match utils::get_persona(persona_result) {
            Ok(persona_data) => {
                let table = render_table(
                    vec![
                        "Result".to_string(),
                        "Level".to_string(),
                        "Arcana".to_string(),
                    ],
                    vec![vec![
                        persona_result.to_string(),
                        persona_data.level.to_string(),
                        persona_data.arcana.to_string(),
                    ]],
                );
                output!("{}", table);
            }
            Err(e) => {
                output_err!("{}", e)
            }
        },
        Err(e) => {
            output_err!("{}", e)
        }
    }
}

pub fn fuse_possible_personas_for(persona: &str) {
    match utils::get_possible_fusions_from_persona(persona) {
        Ok(fusions) => {
            let table = render_table(
                vec![
                    "Fused With".to_string(),
                    "Level".to_string(),
                    "Arcana".to_string(),
                    "Result".to_string(),
                    "Level".to_string(),
                    "Arcana".to_string(),
                ],
                // TODO: Implement a better way to transform a PersonaCombination to a Vec<&str> to avoid
                // extra memory copies
                fusions
                    .into_iter()
                    .map(|persona_combination| {
                        vec![
                            persona_combination.first.name.to_string(),
                            persona_combination.first.data.level.to_string(),
                            persona_combination.first.data.arcana.to_string(),
                            persona_combination.second.name.to_string(),
                            persona_combination.second.data.level.to_string(),
                            persona_combination.second.data.arcana.to_string(),
                        ]
                    })
                    .collect(),
            );
            output!("{table}");
        }
        Err(e) => {
            output_err!("{e}")
        }
    }
}

pub fn reverse_fuse_personas_to(persona: &str) {
    match utils::get_fusions_to(persona) {
        Ok(fusions) => {
            let table = render_table(
                vec![
                    "First".to_string(),
                    "Level".to_string(),
                    "Arcana".to_string(),
                    "Second".to_string(),
                    "Level".to_string(),
                    "Aracana".to_string(),
                ],
                // TODO: Implement a better way to transform a PersonaCombination to a Vec<&str> to avoid
                // extra memory copies
                fusions
                    .into_iter()
                    .map(|persona_combination| {
                        vec![
                            persona_combination.first.name.to_string(),
                            persona_combination.first.data.level.to_string(),
                            persona_combination.first.data.arcana.to_string(),
                            persona_combination.second.name.to_string(),
                            persona_combination.second.data.level.to_string(),
                            persona_combination.second.data.arcana.to_string(),
                        ]
                    })
                    .collect(),
            );
            output!("{table}");
        }
        Err(e) => {
            output_err!("{e}")
        }
    }
}
