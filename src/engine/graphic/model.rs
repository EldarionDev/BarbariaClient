use std::{ffi::c_void, fs::File, io::{BufRead, BufReader}, mem::size_of, ptr};

use crate::maths::Vec3;


enum ReadMode {
    Vertices,
    Indices
}

struct ModelVertex {
    x: f64,
    y: f64,
    z: f64,
    normalX: f64,
    normalY: f64,
    normalZ: f64,
    textureX: f64,
    textureY: f64
}

pub struct Model {
    vertex_array_object: u32,
    vertex_buffer_object: u32,
    element_buffer_object: u32,
    num_indices: u32
}

impl Model {
    pub fn new(model_path: String) -> Model {
        /* Read Model file */
        let mut vertices: Vec<ModelVertex> = Vec::with_capacity(50);
        let mut indices: Vec<u32> = Vec::with_capacity(50);
        let mut num_indices: u32 = 0;
        let mut num_vertices: u32 = 0;

        let model_file = File::open(model_path).unwrap();
        let reader = BufReader::new(model_file);
        let mut mode = ReadMode::Vertices;

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if line.contains("Vertices") {
                mode = ReadMode::Vertices;
            } else if line.contains("Indices") {
                mode = ReadMode::Indices; 
            } else {
                match mode {
                    ReadMode::Vertices => {
                        let mut split_string = line.split_whitespace();
                        let x: f64 = split_string.next().unwrap().parse().unwrap();
                        let y: f64 = split_string.next().unwrap().parse().unwrap();
                        let z: f64 = split_string.next().unwrap().parse().unwrap();
                        let normalX: f64 = split_string.next().unwrap().parse().unwrap();
                        let normalY: f64 = split_string.next().unwrap().parse().unwrap();
                        let normalZ: f64 = split_string.next().unwrap().parse().unwrap();
                        let textureX: f64 = split_string.next().unwrap().parse().unwrap();
                        let textureY: f64 = split_string.next().unwrap().parse().unwrap();
                        vertices.push(ModelVertex {x, y, z, normalX, normalY, normalZ, textureX, textureY});
                        num_vertices += 1;
                    }
                    ReadMode::Indices => {
                        indices.push(line.split_whitespace().next().unwrap().lines().next().unwrap().parse().unwrap());
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
            gl::BufferData(gl::ARRAY_BUFFER, (num_vertices * 64) as isize, (&vertices).as_ptr() as *const c_void, gl::DYNAMIC_DRAW);

            /* Describe data */
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 64, ptr::null());
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 64, 24 as *mut c_void);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, 64, 48 as *mut c_void);
            gl::EnableVertexAttribArray(2);

            /* Bind EBO and data to it */
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (num_indices * 4) as isize, (&indices).as_ptr() as *const c_void, gl::DYNAMIC_DRAW);
        }

        /* Assign values and return */
        Model {
            vertex_array_object: vao,
            vertex_buffer_object: vbo,
            element_buffer_object: ebo,
            num_indices
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vertex_array_object);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, (self.num_indices) as i32, gl::UNSIGNED_INT, ptr::null());
        }
    }
}