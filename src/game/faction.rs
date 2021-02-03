use serde::{Deserialize, Serialize};

mod army;
mod settlement;

#[derive(Serialize, Deserialize, Clone)]
pub struct Faction {
    name: String,
    armies: Option<Vec<army::Army>>,
    settlements: Option<Vec<settlement::Settlement>>,
}

impl Faction {
    pub fn new(name: String) -> Self {
        Faction {
            name,
            armies: None,
            settlements: None,
        }
    }

    pub fn retreive_event_code(&mut self, code: &str) {

    }
}
