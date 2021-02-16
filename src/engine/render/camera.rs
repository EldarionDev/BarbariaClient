use super::{Bindable, Pushable, shader::{self, Shader}};

#[derive(Clone)]
pub struct Camera {}

impl Camera {
    
}

impl Bindable for Camera {
    fn new(path: String) -> Camera {
        Camera {}
    }

    fn load(&mut self) {
        
    }

    fn bind(&self) {}
}

impl Pushable for Camera {
    fn push(&mut self, shader: u32) {
        
    }
}