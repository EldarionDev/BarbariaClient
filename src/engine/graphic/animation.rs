#[derive(Clone)]
pub struct Animation {
    loaded: bool
}

impl Animation {
    pub fn new(path: &str) -> Animation {
        Animation {
            loaded: false
        }
    }
}