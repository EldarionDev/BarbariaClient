use std::{collections::HashMap, ffi::c_void, mem, thread::current};

use freetype::ffi::{FT_LOAD_RENDER, FT_Renderer};

#[derive(Clone)]
pub struct Font {
    loaded: bool,
    font: freetype::Face,
    font_cache: HashMap<char, Character>,
    vao: u32,
    vbo: u32
}

#[derive(Clone)]
struct Character {
    texture_id: u32,
    size: glm::Vec2,
    bearing: glm::Vec2,
    advance: i64
}

impl Font {
    pub fn new(font_library: &mut freetype::Library, path: String) -> Self {
        Font {
            loaded: false,
            font: match font_library.new_face(path, 0) {
                Ok(i) => i,
                Err(e) => panic!("Could not load font.")
            },
            font_cache: HashMap::new(),
            vao: 0,
            vbo: 0
        }
    }

    pub fn load(&mut self) {
        match self.font.set_pixel_sizes(0, 48) {
            Ok(_) => {},   
            Err(_) => panic!("Could not set font size.")
        }

        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        let iterator = (0u8..128).map(|b| b as char as usize);
        for i in iterator {
            match self.font.load_char(i, freetype::face::LoadFlag::RENDER) {
                Ok(_) => {},
                Err(_) => panic!("Could not load font.") 
            }

            let mut texture_id: u32 = 0;
            unsafe {
                gl::GenTextures(1, &mut texture_id);
                gl::BindTexture(gl::TEXTURE_2D, texture_id);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexImage2D(
                    gl::TEXTURE_2D,
                    0,
                    gl::RED as i32,
                    self.font.glyph().bitmap().width(),
                    self.font.glyph().bitmap().rows(),
                    0,
                    gl::RED,
                    gl::UNSIGNED_BYTE,
                    &(self.font.glyph().bitmap().buffer()[0]) as *const u8 as *const c_void
                )
            }

            self.font_cache.insert(i as u8 as char, Character{
                texture_id,
                size: glm::vec2(self.font.glyph().bitmap().width() as f32, self.font.glyph().bitmap().rows() as f32),
                bearing: glm::vec2(self.font.glyph().bitmap_left() as f32, self.font.glyph().bitmap_top() as f32),
                advance: self.font.glyph().advance().x 
            });
        }

        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::BufferData(gl::ARRAY_BUFFER, 
                (mem::size_of::<gl::types::GLfloat>() as u32 * 6 * 4) as gl::types::GLsizeiptr,
                std::ptr::null(), gl::DYNAMIC_DRAW);
            gl::EnableVertexAttribArray(0);
            
            let stride = 4 * mem::size_of::<gl::types::GLfloat>() as gl::types::GLsizei;
            gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        self.loaded = true;
    }

    pub fn render_text(&self, color: glm::Vec3, text: String, shader: &super::shader::Shader, mut position: (f32, f32), scale: f32) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        
            let uniform_location = gl::GetUniformLocation(shader.get_id(), "text_color".as_bytes().as_ptr() as *const i8);
            gl::Uniform3f(uniform_location, color.x, color.y, color.z);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindVertexArray(self.vao);

            for c in text.chars() {
                let current_char = match self.font_cache.get(&c) {
                    Some(i) => i,
                    None => panic!("Could not find char in character map.")
                };

                let x_position = position.0 + current_char.bearing.x * scale;
                let y_position = position.1 - (current_char.size.y - current_char.bearing.y) * scale;
                let width = current_char.size.x * scale;
                let height = current_char.size.y * scale;

                let vertices: [[f32; 4]; 6] = 
                    [[x_position, y_position + height, 0.0, 0.0],
                    [x_position, y_position, 0.0, 1.0],
                    [x_position + width, y_position, 1.0, 1.0],
                    
                    [x_position, y_position + height, 0.0, 0.0],
                    [x_position + width, y_position, 1.0, 1.0],
                    [x_position + width, y_position + height, 1.0, 0.0]];

                gl::BindTexture(gl::TEXTURE_2D, current_char.texture_id);

                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                let size = mem::size_of::<f32>() as isize * 4 * 6;
                gl::BufferSubData(gl::ARRAY_BUFFER, 0, size, (&vertices).as_ptr() as *const i32 as *const c_void);
                gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                gl::DrawArrays(gl::TRIANGLES, 0, 6);
                position.0 += ((current_char.advance >> 6) * scale as i64) as f32;
            }

            gl::BindVertexArray(0);
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}