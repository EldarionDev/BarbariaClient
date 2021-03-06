use serde::{Deserialize, Serialize};

mod unit;

#[derive(Serialize, Deserialize, Clone)]
pub struct Army {
    units: Option<Vec<unit::Unit>>,
}

impl Army {
    pub fn new() -> Self {
        Army { units: None }
    }
}
