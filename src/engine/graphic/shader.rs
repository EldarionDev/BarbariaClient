use std::fs;

pub struct Shader {
    shader_program: usize
}

impl Shader {
    pub fn new(shader_base_path: String) -> Shader {
        let vertex_shader_path = shader_base_path.clone() + ".vs";
        let fragment_shader_path = shader_base_path.clone() + ".fs";

        let vertex_shader_content = fs::read_to_string(vertex_shader_path).unwrap();
        let fragment_shader_content = fs::read_to_string(fragment_shader_path).unwrap();

        

        Shader {
            shader_program: 1
        }
    }

    pub fn bind(&self) {

    }
}