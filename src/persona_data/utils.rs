use super::data::{Arcana, ArcanaCombination, PersonaData, SPECIAL_PERSONAS};
use fusion_calculator_rs::FusionCalculatorError;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PersonaEntry<'a> {
    pub name: &'a str,
    pub data: &'a PersonaData,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PersonaCombination<'a> {
    pub first: PersonaEntry<'a>,
    pub second: PersonaEntry<'a>,
}

/// Returns the data of a Persona, if it exists
pub fn get_persona(name: &str) -> Result<&PersonaData, FusionCalculatorError> {
    if let Some(persona_data) = super::data::PERSONAS.get(name) {
        Ok(persona_data)
    } else {
        Err(FusionCalculatorError::PersonaNotFound(format!(
            "{name} is not a valid persona"
        )))
    }
}

/// Returns the Arcana of two fused Personas, if possible
fn get_fused_arcana<'a>(
    first: &'a Arcana,
    second: &'a Arcana,
) -> Result<&'a Arcana, FusionCalculatorError> {
    // Two Arcanas fused together will always be the same
    if first == second {
        return Ok(first);
    }
    let result = super::data::ARCANA_COMBINATIONS
        .iter()
        .find(|ac| ac.first == *first && ac.second == *second);
    if let Some(combination) = result {
        Ok(&combination.result)
    } else {
        Err(FusionCalculatorError::InvalidFusion(format!(
            "There is no valid fusion for Arcanas {first:?} and {second:?}"
        )))
    }
}

/// Returns a map of Persona names grouped by Arcana
fn get_personas_grouped_by_arcana() -> HashMap<&'static Arcana, Vec<&'static str>> {
    let flattened_personas = super::data::PERSONAS
        .into_iter()
        .map(|(k, v)| (&v.arcana, *k))
        .collect::<Vec<(&Arcana, &str)>>();
    let mut arcana_map: HashMap<&Arcana, Vec<&str>> = HashMap::new();
    for (arcana, persona) in flattened_personas {
        arcana_map
            .entry(arcana)
            .and_modify(|e| e.push(persona))
            .or_insert(vec![persona]);
    }
    arcana_map
}

/// Returns each possible combination required to fuse a persona. Will return an unordered set of tuples containing the names of Personas required.
fn get_possible_combinations_to_fuse_persona(
    arcana_combinations: Vec<&ArcanaCombination>,
    persona_name: String,
) -> HashSet<(&str, &str)> {
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
                    if fusion_result == persona_name {
                        possible_combinations.insert((*persona_0, *persona_1));
                    }
                }
            }
        }
    }
    possible_combinations
}

/// Returns the fusion of two Personas, if possible
pub fn get_fusion_of<'a>(
    first: &'a str,
    second: &'a str,
) -> Result<&'a str, FusionCalculatorError> {
    let p1 = get_persona(first)?;
    let p2 = get_persona(second)?;
    let fused_arcana = get_fused_arcana(&p1.arcana, &p2.arcana)?;
    let avg_level = 1 + ((p1.level + p2.level) / 2);
    let mut fusions = super::data::PERSONAS
        .into_iter()
        .filter(|(_, persona_data)| {
            persona_data.arcana == *fused_arcana && persona_data.level <= avg_level
        })
        .map(|(k, v)| (*k, v.level))
        .collect::<Vec<(&str, u8)>>();
    // Sort the available personas in descending order of level, that will be the one closest to the average level
    fusions.sort_by(|a, b| b.1.cmp(&a.1));
    if let Some((persona, _)) = fusions.first() {
        Ok(persona)
    } else {
        Err(FusionCalculatorError::InvalidFusion(format!(
            "There is no valid fusion for Personas {first} and {second}"
        )))
    }
}

/// Returns a list of Personas that can be used to fuse the given Persona, if possible
pub fn get_fusions_to(name: &str) -> Result<Vec<PersonaCombination>, FusionCalculatorError> {
    let persona_data = get_persona(name)?;
    let arcana_combinations = super::data::ARCANA_COMBINATIONS
        .iter()
        .filter(|combination| combination.result == persona_data.arcana)
        .collect::<Vec<&ArcanaCombination>>();

    let mut sorted_combinations =
        get_possible_combinations_to_fuse_persona(arcana_combinations, name.to_string())
            .into_iter()
            .map(|(p0, p1)| PersonaCombination {
                first: PersonaEntry {
                    name: p0,
                    data: get_persona(p0)
                        .unwrap_or_else(|_| panic!("Somehow we didn't get data for Persona {p0}")),
                },
                second: PersonaEntry {
                    name: p1,
                    data: get_persona(p1)
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
            .then(a.first.data.arcana.cmp(&b.first.data.arcana))
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
            let fused_data = get_persona(fused).unwrap_or_else(|e| {
                panic!("Somehow we didn't get Persona data for {fused}: {e:?}")
            });
            let persona_combination = PersonaCombination {
                first: PersonaEntry {
                    name: persona_name,
                    data: persona_data,
                },
                second: PersonaEntry {
                    name: fused,
                    data: fused_data,
                },
            };
            fusions.push(persona_combination);
        }
    }
    Ok(fusions)
}

/// Returns the result of a special fusion between two personas, if applicable
pub fn get_special_fusion<'a>(first: &'a str, second: &'a str) -> Option<&'a str> {
    if let Some((persona_name, _)) = SPECIAL_PERSONAS
        .entries()
        .find(|(_, value)| value.len() == 2 && value.contains(first) && value.contains(second))
    {
        return Some(persona_name);
    }
    None
}

/// Returns the collection of personas required to fuse a special persona, if applicable
pub fn get_special_fusions_to(name: &str) -> Option<Vec<Vec<String>>> {
    if let Some(fusions) = SPECIAL_PERSONAS.get(name) {
        let x = fusions
            .iter()
            .map(|persona| {
                let persona_data = get_persona(persona)
                    .unwrap_or_else(|_| panic!("Should have gotten data for {persona}"));
                (*persona, persona_data)
            })
            .map(|(name, data)| {
                vec![
                    name.to_string(),
                    data.level.to_string(),
                    data.arcana.to_string(),
                ]
            })
            .collect::<Vec<Vec<String>>>();
        return Some(x);
    }
    None
}
