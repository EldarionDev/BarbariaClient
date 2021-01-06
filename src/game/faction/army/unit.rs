use serde::{Serialize, Deserialize};

#[derive (Serialize, Deserialize)]
pub struct Unit {

}

impl Unit {
    pub fn new() -> Self {
        Unit {
            
        }
    }
}