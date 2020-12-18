use std::error::Error;
pub struct Army {
    name: String,
    unit_names: Vec<String>,
    general_unit_name: String,
}

impl Army {
    pub fn new(name: String, unit_names: Vec<String>, general_unit_name: String) -> Result<Self, Box<dyn Error>> {
        Ok( Army {
            name,
            unit_names,
            general_unit_name
        }
        )
    }
}