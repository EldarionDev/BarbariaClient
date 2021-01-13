mod landscape;
mod region;

#[derive(Clone)]
pub struct Map {
    landscapes: Option<Vec<landscape::Landscape>>,
    regions: Option<Vec<region::Region>>
}

impl Map {
    pub fn new() -> Self {
        Map {
            landscapes: None,
            regions: None
        }
    }
}