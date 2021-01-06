use serde::{Serialize, Deserialize};

mod building;

#[derive(Serialize, Deserialize)]
pub struct Settlement {
    buildings: Option<Vec<building::Building>>
}

impl Settlement {
    pub fn new() -> Self {
        Settlement {
            buildings: None
        }
    }
}