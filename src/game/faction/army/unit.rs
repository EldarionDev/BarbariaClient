use serde::{Serialize, Deserialize};

#[derive (Serialize, Deserialize, Clone)]
pub struct Unit {

}

impl Unit {
    pub fn new() -> Self {
        Unit {
            
        }
    }
}