use serde::{Serialize, Deserialize};

mod unit;

#[derive (Serialize, Deserialize)]
pub struct Army {
    units: Option<Vec<unit::Unit>>
}

impl Army {
    pub fn new() -> Self {
        Army {
            units: None
        }
    }
}