use super::data::{ArcanaCombination, PersonaData, PERSONAS, SPECIAL_PERSONAS};
use itertools::Itertools;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FusionCalculatorError {
    #[error("A Persona with the name {name:?} could not be found")]
    PersonaNotFound { name: String },

    #[error("No valid fusion between {first:?} and {second:?}")]
    InvalidFusion { first: String, second: String },
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct PersonaEntry {
    pub name: &'static str,
    pub data: PersonaData,
}

#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct PersonaCombination {
    pub first: PersonaEntry,
    pub second: PersonaEntry,
}

/// Returns the data of a Persona, if it exists
pub fn get_persona(name: &str) -> Result<&PersonaData, FusionCalculatorError> {
    if let Some(persona_data) = super::data::PERSONAS.get(name) {
        Ok(persona_data)
    } else {
        Err(FusionCalculatorError::PersonaNotFound {
            name: name.to_string(),
        })
    }
}

/// Returns the Arcana of two fused Personas, if possible
pub fn get_fused_arcana<'a>(
    first: &'a str,
    second: &'a str,
) -> Result<&'a str, FusionCalculatorError> {
    // Two personas fused together will always equal each other
    if first == second {
        return Ok(first);
    }
    let result = super::data::ARCANA_COMBINATIONS_MAP
        .get(first)
        .and_then(|v| v.get(second));
    if let Some(combination) = result {
        Ok(combination)
    } else {
        Err(FusionCalculatorError::InvalidFusion {
            first: first.to_string(),
            second: second.to_string(),
        })
    }
}

/// Returns a map of Persona names grouped by Arcana
fn get_personas_grouped_by_arcana() -> HashMap<&'static str, Vec<&'static str>> {
    super::data::PERSONAS
        .into_iter()
        .map(|(k, v)| (v.arcana, *k))
        .into_group_map()
}

/// Returns each possible combination required to fuse a persona. Will return an unordered set of tuples containing the names of Personas required.
fn get_possible_combinations_to_fuse_persona(
    arcana_combinations: Vec<ArcanaCombination>,
    persona_name: String,
) -> HashSet<(&'static str, &'static str)> {
    let personas_grouped_by_arcana = get_personas_grouped_by_arcana();
    let mut possible_combinations = HashSet::new();

    for arcana_combination in arcana_combinations {
        for persona_0 in personas_grouped_by_arcana
            .get(&arcana_combination.first)
            .unwrap_or_else(|| {
                panic!(
                    "Somehow we didn't get a list of Personas for this Arcana {:?}",
                    arcana_combination.first
                )
            })
        {
            for persona_1 in personas_grouped_by_arcana
                .get(&arcana_combination.second)
                .unwrap_or_else(|| {
                    panic!(
                        "Somehow we didn't get a list of Personas for this Arcana {:?}",
                        arcana_combination.second
                    )
                })
            {
                if possible_combinations.contains(&(*persona_0, *persona_1)) {
                    continue;
                }
                if let Ok(fusion_result) = get_fusion_of(persona_0, persona_1) {
                    if fusion_result.name == persona_name {
                        possible_combinations.insert((*persona_0, *persona_1));
                    }
                }
            }
        }
    }
    possible_combinations
}

/// Returns the fusion of two Personas, if possible
pub fn get_fusion_of(first: &str, second: &str) -> Result<PersonaEntry, FusionCalculatorError> {
    let p1 = get_persona(first)?;
    let p2 = get_persona(second)?;
    let fused_arcana = get_fused_arcana(p1.arcana, p2.arcana)?;
    let avg_level = 1 + ((p1.level + p2.level) / 2);
    let mut fusions = super::data::PERSONAS
        .into_iter()
        .filter(|(_, persona_data)| {
            persona_data.arcana == fused_arcana && persona_data.level <= avg_level
        })
        .map(|(k, v)| PersonaEntry { name: k, data: *v })
        .collect::<Vec<PersonaEntry>>();
    // Sort the available personas in descending order of level, that will be the one closest to the average level
    fusions.sort_by(|a, b| b.data.level.cmp(&a.data.level));
    if let Some(persona) = fusions.first() {
        Ok(persona.clone())
    } else {
        Err(FusionCalculatorError::InvalidFusion {
            first: first.to_string(),
            second: second.to_string(),
        })
    }
}

/// Returns a list of Personas that can be used to fuse the given Persona, if possible
pub fn get_fusions_to(name: String) -> Result<Vec<PersonaCombination>, FusionCalculatorError> {
    let persona_data = get_persona(&name)?;
    let arcana_combinations = super::data::ARCANA_COMBINATIONS_MAP
        .get(persona_data.arcana)
        .map(|combinations| {
            combinations
                .entries()
                .map(|(second, result)| ArcanaCombination {
                    first: persona_data.arcana,
                    second,
                    result,
                })
                .collect_vec()
        })
        .unwrap_or_default();

    let mut sorted_combinations =
        get_possible_combinations_to_fuse_persona(arcana_combinations, name)
            .into_iter()
            .map(|(p0, p1)| PersonaCombination {
                first: PersonaEntry {
                    name: p0,
                    data: *get_persona(p0)
                        .unwrap_or_else(|_| panic!("Somehow we didn't get data for Persona {p0}")),
                },
                second: PersonaEntry {
                    name: p1,
                    data: *get_persona(p1)
                        .unwrap_or_else(|_| panic!("Somehow we didn't get data for Persona {p1}")),
                },
            })
            .collect::<Vec<PersonaCombination>>();
    sorted_combinations.sort_unstable_by(|a, b| {
        a.first
            .data
            .level
            // Sort by the first persona's level
            .cmp(&b.first.data.level)
            // Then by the first persona's arcana so we group by arcana
            .then(a.first.data.arcana.cmp(b.first.data.arcana))
            // Finally by the second persona's level, so the second group's arcana is also sorted by ascending level
            .then(a.second.data.level.cmp(&b.second.data.level))
    });
    Ok(sorted_combinations)
}

/// Returns the possible fusions if the given persona is used
pub fn get_possible_fusions_from_persona(
    name: &str,
) -> Result<Vec<PersonaCombination>, FusionCalculatorError> {
    let mut fusions = Vec::new();
    for (persona_name, persona_data) in super::data::PERSONAS
        .into_iter()
        // Iterated through entries sorted by ascending level so we can a cleaner output
        .sorted_by(|a, b| a.1.level.cmp(&b.1.level))
    {
        if let Ok(fused) = get_fusion_of(name, persona_name) {
            let persona_combination = PersonaCombination {
                first: PersonaEntry {
                    name: persona_name,
                    data: *persona_data,
                },
                second: PersonaEntry {
                    name: fused.name,
                    data: fused.data,
                },
            };
            fusions.push(persona_combination);
        }
    }
    Ok(fusions)
}

/// Returns the result of a special fusion between two personas, if applicable
pub fn get_special_fusion(first: &str, second: &str) -> Option<PersonaEntry> {
    if let Some((persona_name, _)) = SPECIAL_PERSONAS
        .entries()
        .find(|(_, value)| value.len() == 2 && value.contains(&first) && value.contains(&second))
    {
        let persona_data = get_persona(persona_name)
            .unwrap_or_else(|_| panic!("Should have gotten data for {persona_name}"));
        return Some(PersonaEntry {
            name: persona_name,
            data: *persona_data,
        });
    }
    None
}

/// Returns the collection of personas required to fuse a special persona, if applicable
pub fn get_special_fusions_to(name: &str) -> Option<Vec<PersonaEntry>> {
    if let Some(fusions) = SPECIAL_PERSONAS.get(name) {
        let x = fusions
            .iter()
            .map(|persona| {
                let persona_data = get_persona(persona)
                    .unwrap_or_else(|_| panic!("Should have gotten data for {persona}"));
                (*persona, persona_data)
            })
            .map(|(name, data)| PersonaEntry { name, data: *data })
            .collect::<Vec<PersonaEntry>>();
        return Some(x);
    }
    None
}

/// Returns all Personas
pub fn get_all_personas(
    arcanas: &Vec<String>,
    min_level: &Option<usize>,
    max_level: &Option<usize>,
) -> Vec<PersonaEntry> {
    let personas: Vec<PersonaEntry> =
        if arcanas.is_empty() && min_level.is_none() && max_level.is_none() {
            let mut first_pass = PERSONAS
                .entries()
                .map(|(name, data)| PersonaEntry { name, data: *data })
                .collect_vec();
            // Sort by name, ascending by default
            first_pass.sort_by(|a, b| a.name.cmp(b.name));
            first_pass
        } else {
            let mut personas_by_filters = PERSONAS
                .entries()
                .filter(|(_, data)| {
                    if !arcanas.is_empty() {
                        arcanas.contains(&data.arcana.to_string())
                    } else {
                        true
                    }
                })
                .filter(|(_, data)| {
                    if let Some(min) = min_level {
                        data.level as usize >= *min
                    } else {
                        true
                    }
                })
                .filter(|(_, data)| {
                    if let Some(max) = max_level {
                        data.level as usize <= *max
                    } else {
                        true
                    }
                })
                .map(|(name, data)| PersonaEntry { name, data: *data })
                .collect_vec();
            personas_by_filters.sort_unstable_by(|a, b| {
                a.data
                    .arcana
                    .cmp(b.data.arcana)
                    .then(a.data.level.cmp(&b.data.level))
            });
            personas_by_filters
        };

    personas
}
