use std::{convert::TryInto, ffi::CString, fs, u32};

use serde_json::Value;

use super::{Bindable, Pushable, shader::Shader};

#[derive(Clone)]
pub struct Projection {
    projection_matrix: glm::Mat4
}

impl Bindable for Projection {
    fn new(path: String) -> Self {
        let projection_file_content: String = fs::read_to_string(path).unwrap().parse().unwrap();
        let projection_file_content = &projection_file_content[..];
        let json_content: Value = serde_json::from_str(projection_file_content).unwrap();

        if json_content["type"].as_str().unwrap() == "orthogonal" {
            let n = json_content["near_plane"].as_f64().unwrap() as f32;
            let f = json_content["far_plane"].as_f64().unwrap() as f32;
            let r = 1.0;
            let l = -1.0;
            let t = 1.0;
            let b = -1.0;

            let orthogonal = glm::mat4(
                2.0 / (r - l), 0.0, 0.0, -((r + l) / (r - l)),
                0.0, 2.0 / (t - b), 0.0, - ((t + b) / (t - b)),
                0.0, 0.0, -2.0 / (f - n), - ((f + n) / (f - n)),
                0.0, 0.0, 0.0, 1.0
            );

            return Projection {
                projection_matrix: orthogonal
            }
        } else if json_content["type"].as_str().unwrap() == "perspective" {
            return Projection {
                projection_matrix: glm::ext::perspective(glm::builtin::radians(json_content["fov"].as_f64().unwrap() as f32), 1.0, json_content["near_plane"].as_f64().unwrap() as f32, json_content["far_plane"].as_f64().unwrap() as f32)
            }
        } else {
            panic!("Unknown projection in construction!");
        }
    }

    fn load(&mut self) {

    }

    fn bind(&self) {
        
    }
}

impl Pushable for Projection {
    fn push(&mut self, shader: u32) {
        let string = CString::new("projection").unwrap();
        unsafe {
            let shader_location = gl::GetUniformLocation(shader, string.as_bytes().as_ptr() as *const i8);
            if shader_location == -1 {
                panic!("Shader location for projection matrix is not existing.");
            }
            gl::UniformMatrix4fv(shader_location, 1, 0, self.projection_matrix[0].as_array().as_ptr());
        }
    }
}