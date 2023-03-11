use std::vec;

use fusion_calculator_rs::FusionCalculatorError;

use crate::persona_data::data::Arcana;
use crate::persona_data::utils;
use crate::terminal::table::render_table;
use crate::terminal::term::{output, output_err};

fn get_fusion<'a>(
    first: &'a str,
    second: &'a str,
) -> Result<Vec<Vec<String>>, FusionCalculatorError> {
    let persona_result = if let Some(special_result) = utils::get_special_fusion(first, second) {
        special_result
    } else {
        utils::get_fusion_of(first, second)?
    };
    let persona_data = utils::get_persona(persona_result)?;
    Ok(vec![vec![
        persona_result.to_string(),
        persona_data.level.to_string(),
        persona_data.arcana.to_string(),
    ]])
}

pub fn fuse_personas(first: &str, second: &str) {
    match get_fusion(first, second) {
        Ok(persona_data) => {
            let table = render_table(
                vec![
                    "Result".to_string(),
                    "Level".to_string(),
                    "Arcana".to_string(),
                ],
                persona_data,
            );
            output!("{}", table);
        }
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

pub fn fusions_to(persona: &str) {
    if let Some(reverse_fusions) = utils::get_special_fusions_to(persona) {
        let table = render_table(
            vec![
                "Fused With".to_string(),
                "Level".to_string(),
                "Arcana".to_string(),
            ],
            reverse_fusions,
        );
        output!("{table}");
        return;
    }
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

pub fn list_all_personas(
    arcanas: &Vec<Arcana>,
    min_level: &Option<usize>,
    max_level: &Option<usize>,
) {
    let personas = utils::get_all_personas(arcanas, min_level, max_level);
    let table = render_table(
        vec![
            "Name".to_string(),
            "Level".to_string(),
            "Arcana".to_string(),
        ],
        personas
            .into_iter()
            .map(|(name, data)| {
                vec![
                    name.to_string(),
                    data.level.to_string(),
                    data.arcana.to_string(),
                ]
            })
            .collect(),
    );
    output!("{table}")
}
