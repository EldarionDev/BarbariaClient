use std::error::Error;

pub struct Faction {
    name: String,
    gold: u32,
}

impl Faction {
    pub fn new(faction_name: &str, gold: u32) -> Result<Self, Box<Error>> {
        Ok(Faction {
            name: faction_name.to_string(),
            gold,
        })
    }
}