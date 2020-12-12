mod graphics;
use std::error::Error;

use crate::info;

pub struct Engine {
    glfw_instance: glfw::Glfw,
    graphic_engine: graphics::Graphic,
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

    pub fn do_engine_tick(&mut self) {
        self.graphic_engine.do_render_tick();
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        info!("Terminating the Barbaria Engine!");
        drop(&self.glfw_instance);
    }
}