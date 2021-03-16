use std::{ffi::CString, fs, ptr};

#[derive(Clone)]
pub struct Shader {
    shader_path: String,
    shader_program: Option<u32>,
    loaded: bool
}

impl Shader {
    pub fn new(shader_base_path: String) -> Shader {
        /* Assign values and return */
        Shader { 
            shader_path: shader_base_path,
            shader_program: None,
            loaded: false
        }
    }

    pub fn load(&mut self) {
        if self.loaded == true {
            return;
        } 

        /* Parse shader files to CStrings */
        let vertex_shader_path = self.shader_path.clone() + ".vs";
        let fragment_shader_path = self.shader_path.clone() + ".fs";

        let vertex_shader_content = match fs::read_to_string(vertex_shader_path) {
            Ok(c) => c,
            Err(e) => panic!(
                "Error while reading vertex shader of: {} because: {}",
                self.shader_path, e
            ),
        };

        let fragment_shader_content = match fs::read_to_string(fragment_shader_path) {
            Ok(c) => c,
            Err(e) => panic!(
                "Error while reading fragment shader of: {} because: {}",
                self.shader_path, e
            ),
        };

        let vertex_shader_content = match CString::new(vertex_shader_content.as_bytes()) {
            Ok(c) => c,
            Err(e) => panic!(
                "Error while converting vertex shader of: {} to a CString because: {}",
                self.shader_path, e
            ),
        };

        let fragment_shader_content = match CString::new(fragment_shader_content.as_bytes()) {
            Ok(c) => c,
            Err(e) => panic!(
                "Error while converting fragment shader of: {} to a CString because: {}",
                self.shader_path, e
            ),
        };

        /* Create OpenGL shaders */
        let vertex_shader: u32 = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

        let fragment_shader: u32 = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

        /* Assign shader source codes to OpenGL shaders */
        unsafe {
            gl::ShaderSource(
                vertex_shader,
                1,
                &vertex_shader_content.as_ptr(),
                ptr::null(),
            );
            gl::ShaderSource(
                fragment_shader,
                1,
                &fragment_shader_content.as_ptr(),
                ptr::null(),
            );
        }

        /* Compile the shaders and check for errors */
        let info_log: Vec<char> = Vec::with_capacity(512);
        let mut success: i32 = 0;
        let success_ptr: *mut i32 = &mut success as *mut i32;

        unsafe {
            gl::CompileShader(vertex_shader);
            gl::CompileShader(fragment_shader);

            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, success_ptr);
            if success < 1 {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    0 as *mut i32,
                    info_log.as_ptr() as *mut i8,
                );
                println!("Fatal error compiling vertex shader: {:?}", info_log);
            }

            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, success_ptr);
            if success < 1 {
                gl::GetShaderInfoLog(
                    fragment_shader,
                    512,
                    0 as *mut i32,
                    info_log.as_ptr() as *mut i8,
                );
                println!("Fatal error compiling fragment shader: {:?}", info_log);
            }
        }

        /* Create shader program and assign the compiled shaders to it, link shader program */
        let shader_program: u32;
        let info_log: Vec<char> = Vec::with_capacity(512);
        let mut success: i32 = 0;
        let success_ptr: *mut i32 = &mut success as *mut i32;

        unsafe {
            shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            /* Check for errors */
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, success_ptr);
            if success < 1 {
                gl::GetProgramInfoLog(
                    shader_program,
                    512,
                    0 as *mut i32,
                    info_log.as_ptr() as *mut i8,
                );
                println!("Error while linking shader program: {:?}", info_log);
            }
        }

        self.shader_program = Some(shader_program);
        self.loaded = true;
    }

    pub fn get_id(&self) -> u32{
        match self.shader_program {
            Some(i) => i,
            None => panic!("Attempte to retreive shader location without initalising shader!")
        }
    }

    pub fn bind(&self) {
        let shader_program = match self.shader_program {
            Some(i) => i,
            None => panic!("Attempted to use unitialized shader: {}", self.shader_path)
        };

        unsafe {
            gl::UseProgram(shader_program);
        }
    }
}