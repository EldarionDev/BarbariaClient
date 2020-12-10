mod graphics;


use crate::info;

struct Engine {
    glfw_instance: glfw::Glfw,
}

impl Drop for Engine {
    fn drop(&mut self) {
        info!("Terminating the Barbaria Engine!");
        drop(self.glfw_instance);
    }
}

impl Engine {
    pub fn new() -> Result<Self, glfw::InitError> {
        info!("Starting the Barbaria Engine!");
        Ok(Engine {
            glfw_instance: glfw::init(glfw::FAIL_ON_ERRORS)?,
        })
    }
}