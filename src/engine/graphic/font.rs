pub struct Font {
    loaded: bool
}

impl Font {
    pub fn new(path: String) -> Self {
        Font {
            loaded: false
        }
    }
}