use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Building {}

impl Building {
    pub fn new() -> Self {
        Building {}
    }
}
