use std::{collections::HashMap, ffi::c_void, thread::current};

use freetype::ffi::{FT_LOAD_RENDER, FT_Renderer};

pub struct Font {
    loaded: bool,
    font: freetype::Face,
    font_cache: HashMap<char, Character>
}

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
            font_cache: HashMap::new()
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

        self.loaded = true;
    }
}