use super::shader::Shader;

#[derive(Clone)]
pub struct Camera {}

impl Camera {
    pub fn new(path: &str) -> Camera {
        Camera {}
    }

    pub fn bind(&self, shader: &Shader) {}
}
