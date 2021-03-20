use std::{convert::TryInto, ffi::CString, fs, u32};

use serde_json::Value;

use super::shader::Shader;

#[derive(Clone)]
pub struct Projection {
    projection_matrix: glm::Mat4,
}

impl Projection {
    pub fn new(path: &str, screen_size: (f32, f32)) -> Projection {
        let projection_file_content: String = fs::read_to_string(path).unwrap().parse().unwrap();
        let projection_file_content = &projection_file_content[..];
        let json_content: Value = serde_json::from_str(projection_file_content).unwrap();

        if json_content["type"].as_str().unwrap() == "orthogonal" {
            let n = json_content["near_plane"].as_f64().unwrap() as f32;
            let f = json_content["far_plane"].as_f64().unwrap() as f32;
            let r = screen_size.0;
            let l = 0.0;
            let t = screen_size.1;
            let b = 0.0;

            let orthogonal = glm::ortho(l, r, b, t, n, f);

            return Projection {
                projection_matrix: orthogonal
            }
        } else if json_content["type"].as_str().unwrap() == "perspective" {
            return Projection {
                projection_matrix: glm::perspective(screen_size.0 / screen_size.1, json_content["fov"].as_f64().unwrap() as f32 * 0.01745329, json_content["near_plane"].as_f64().unwrap() as f32, json_content["far_plane"].as_f64().unwrap() as f32)
            }
        } else {
            panic!("Unknown projection in construction!");
        }
    }

    pub fn bind(&self, shader: &Shader) {
        let string = CString::new("projection").unwrap();
        unsafe {
            let shader_location = gl::GetUniformLocation(shader.get_id(), string.as_bytes().as_ptr() as *const i8);
            if shader_location == -1 {
                panic!("Shader location for projection matrix is not existing.");
            }
            gl::UniformMatrix4fv(shader_location, 1, 0, self.projection_matrix.as_ptr());
        }
    }
}