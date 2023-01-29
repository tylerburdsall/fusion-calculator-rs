enum Arcana {
    Fool,
    Magician,
    Priestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Justice,
    Hermit,
    Fortune,
    Strength,
    Hanged,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    Faith,
    Councillor,
    World
}

struct ArcanaCombination {
    first: &'a Arcana,
    second: &'a Arcana,
    result: &'a Arcana,
}

struct PersonaData {
    // name: &'a str,
    level: &'a u8,
    arcana: &'a Arcana,
    special: &'a bool,
    max: &'a bool,
}