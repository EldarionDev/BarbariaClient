use super::shader::Shader;

#[derive(Clone)]
pub struct Projection {}

impl Projection {
    pub fn new(path: &str) -> Projection {
        Projection {}
    }

    pub fn bind(&self, shader: &Shader) {}
}
