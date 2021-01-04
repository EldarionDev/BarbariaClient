mod army;
mod settlement;

pub struct Faction {
    armies: Option<Vec<army::Army>>,
    settlements: Option<Vec<settlement::Settlement>>
}

impl Faction {
    pub fn new()-> Self {
        Faction {
            armies: None,
            settlements: None,
        }
    }
}