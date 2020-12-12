mod graphics;
use std::error::Error;

use crate::info;

pub struct Engine {
    glfw_instance: glfw::Glfw,
    graphic_engine: graphics::Graphic,
}

impl Drop for Engine {
    fn drop(&mut self) {
        info!("Terminating the Barbaria Engine!");
        drop(&self.glfw_instance);
    }
}

impl Engine {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        info!("Starting the Barbaria Engine!");
        let mut gli = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let gre = graphics::Graphic::new(&mut gli); 
        Ok(Engine {
            glfw_instance: gli,
            graphic_engine: gre?,
        })
    }
}