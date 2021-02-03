pub struct Listener {
    pub event_codes: Vec<String>
}

impl Listener {
    pub fn new() -> Self{
        Listener {
            event_codes: Vec::new()
        }
    }
}