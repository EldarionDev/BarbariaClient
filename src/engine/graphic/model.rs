use std::{
    ffi::c_void,
    fs::File,
    io::{BufRead, BufReader},
    mem, ptr,
    str::SplitWhitespace,
};

use glm::{Vec3};

extern crate gl;

enum ReadMode {
    Vertices,
    Indices,
}

struct ModelVertex {
    x: f32,
    y: f32,
    z: f32,
    normal_x: f32,
    normal_y: f32,
    normal_z: f32,
    texture_x: f32,
    texture_y: f32,
}

#[derive(Clone)]
pub struct Model {
    model_path: String,
    vertex_array_object: Option<u32>,
    vertex_buffer_object: Option<u32>,
    element_buffer_object: Option<u32>,
    num_indices: Option<u32>,
    loaded: bool
}

impl Model {
    pub fn new(model_path: String) -> Model {
        Model {
            model_path,
            vertex_array_object: None,
            vertex_buffer_object: None,
            element_buffer_object: None,
            num_indices: None,
            loaded: false
        }
    }

    pub fn load(&mut self) {
        if self.loaded == true {return;}

        /* Read Model file */
        let mut vertices: Vec<ModelVertex> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut num_indices: u32 = 0;
        let mut num_vertices: u32 = 0;

        let model_file = match File::open(self.model_path.clone()) {
            Ok(f) => f,
            Err(e) => panic!("Could not read model file: {}", e),
        };
        let reader = BufReader::new(model_file);
        let mut mode = ReadMode::Vertices;

        let split_convert_string = |s: &mut SplitWhitespace<'_>| -> f32 {
            let s = match s.next() {
                Some(s) => s,
                None => panic!("Could not split by Whitespaces in model file"),
            };
            match s.parse() {
                Ok(f) => f,
                Err(e) => panic!("Could not convert model file string to float: {}", e),
            }
        };

        for (_, line) in reader.lines().enumerate() {
            let line = match line {
                Ok(l) => l,
                Err(e) => panic!("Could not read line of model file: {}", e),
            };

            if line.contains("Vertices") {
                mode = ReadMode::Vertices;
                continue;
            } else if line.contains("Indices") {
                mode = ReadMode::Indices;
                continue;
            } else {
                match mode {
                    ReadMode::Vertices => {
                        let mut split_string = line.split_whitespace();
                        let x: f32 = split_convert_string(&mut split_string);
                        let y: f32 = split_convert_string(&mut split_string);
                        let z: f32 = split_convert_string(&mut split_string);
                        let normal_x: f32 = split_convert_string(&mut split_string);
                        let normal_y: f32 = split_convert_string(&mut split_string);
                        let normal_z: f32 = split_convert_string(&mut split_string);
                        let texture_x: f32 = split_convert_string(&mut split_string);
                        let texture_y: f32 = split_convert_string(&mut split_string);
                        vertices.push(ModelVertex {
                            x,
                            y,
                            z,
                            normal_x,
                            normal_y,
                            normal_z,
                            texture_x,
                            texture_y,
                        });
                        num_vertices += 1;
                    }
                    ReadMode::Indices => {
                        let index = match line.split_whitespace().next() {
                            Some(i) => i,
                            None => {
                                panic!("Error extracting whitespaces from index of model file!")
                            }
                        };

                        let index = match index.lines().next() {
                            Some(i) => i,
                            None => panic!("Error extracting newlines from index of model file!"),
                        };

                        let index: u32 = match index.parse() {
                            Ok(i) => i,
                            Err(e) => {
                                panic!("Failed converting index of model file to u32 type: {}", e)
                            }
                        };

                        indices.push(index);
                        num_indices += 1;
                    }
                }
            }
        }

        /* Construct OpenGL data and pass model data */
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut ebo: u32 = 0;

        unsafe {
            /* Generate buffers */
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            /* Bind buffers */
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            /* Pass vertex data */
            /* Try vertices.as_ptr() */
            println!("test: {}", num_vertices * 64);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (num_vertices * 8 * mem::size_of::<gl::types::GLfloat>() as u32)
                    as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            let stride = 8 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;
            /* Describe data */
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (6 * mem::size_of::<gl::types::GLfloat>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);

            /* Bind EBO and data to it */
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (num_indices * mem::size_of::<gl::types::GLfloat>() as u32)
                    as gl::types::GLsizeiptr,
                (&indices).as_ptr() as *const i32 as *const c_void,
                gl::DYNAMIC_DRAW,
            );
        }

        self.vertex_array_object = Some(vao);
        self.vertex_buffer_object = Some(vbo);
        self.element_buffer_object = Some(ebo);
        self.num_indices = Some(num_indices);

        self.loaded = true;
    }

    pub fn bind(&self) {
        let vao = match self.vertex_array_object {
            Some(i) => i,
            None => panic!("Attempted to bind unitialized Model: {}", self.model_path)
        };

        unsafe {
            gl::BindVertexArray(vao);
        }
    }

    pub fn draw(&self) {
        let num_indices = match self.num_indices  {
            Some(i) => i,
            None => panic!("Attempted to draw unitialized Model: {}", self.model_path)
        };

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                (num_indices) as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}