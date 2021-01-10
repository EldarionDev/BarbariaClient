use std::{ffi::CString, fs, ptr};

pub struct Shader {
    shader_program: u32
}

impl Shader {
    pub fn new(shader_base_path: String) -> Shader {
        /* Parse shader files to CStrings */
        let vertex_shader_path = shader_base_path.clone() + ".vs";
        let fragment_shader_path = shader_base_path.clone() + ".fs";

        let vertex_shader_content = fs::read_to_string(vertex_shader_path).unwrap();
        let fragment_shader_content = fs::read_to_string(fragment_shader_path).unwrap();

        let vertex_shader_content = CString::new(vertex_shader_content.as_bytes()).unwrap();
        let fragment_shader_content = CString::new(fragment_shader_content.as_bytes()).unwrap();

        /* Create OpenGL shaders */
        let vertex_shader: u32 = unsafe {
            gl::CreateShader(gl::VERTEX_SHADER)
        };

        let fragment_shader: u32 = unsafe {
            gl::CreateShader(gl::FRAGMENT_SHADER)
        };

        /* Assign shader source codes to OpenGL shaders */
        unsafe {
            gl::ShaderSource(vertex_shader, 1, &vertex_shader_content.as_ptr(), ptr::null());
            gl::ShaderSource(fragment_shader, 1, &fragment_shader_content.as_ptr(), ptr::null());
        }

        /* Compile the shaders and check for errors */
        let mut infoLog: Vec<char> = Vec::with_capacity(512);
        let mut success: i32 = 0;
        let mut success_ptr: *mut i32 = &mut success as *mut i32;

        unsafe {
            gl::CompileShader(vertex_shader);
            gl::CompileShader(fragment_shader);
            
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, success_ptr);
            if success < 1 {
                gl::GetShaderInfoLog(vertex_shader, 512, 0 as *mut i32, infoLog.as_ptr() as *mut i8);
                println!("Fatal error compiling vertex shader: {:?}", infoLog);
            }

            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, success_ptr);
            if success < 1 {
                gl::GetShaderInfoLog(fragment_shader, 512, 0 as *mut i32, infoLog.as_ptr() as *mut i8);
                println!("Fatal error compiling fragment shader: {:?}", infoLog);
            }
        }

        /* Create shader program and assign the compiled shaders to it, link shader program */
        let shader_program: u32;
        let mut infoLog: Vec<char> = Vec::with_capacity(512);
        let mut success: i32 = 0;
        let mut success_ptr: *mut i32 = &mut success as *mut i32;

        unsafe {
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            /* Check for errors */
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, success_ptr);
            if success < 1 {
                gl::GetProgramInfoLog(shader_program, 512, 0 as *mut i32, infoLog.as_ptr() as *mut i8);
                println!("Error while linking shader program: {:?}", infoLog);
            }
        }

        /* Assign values and return */
        Shader {
            shader_program
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.shader_program);
        }
    }
}