use std::ffi::c_void;

use freetype::{Library, RenderMode};

pub struct Font {
    font: freetype::Face,
    characters: Vec<Character>,
}

struct Character {
    letter: char,
    texture_id: Option<u32>,
    size: (i32, i32)
}

impl Font {
    pub fn new(font_engine: &mut Library, path: &str) -> Self {
        let mut face = match font_engine.new_face(path, 0) {
            Ok(i) => i,
            Err(e) => panic!("Could not load font: {} with error: {}", path, e)
        };

        match face.set_char_size(1000, 1000, 1000, 1000) {
            Ok(_) => (),
            Err(e) => panic!("Could not set font size because of: {}", e)
        };

        Font {
            font: face,
            characters: Vec::new()
        }
    }

    pub fn load(&mut self) {
        unsafe {
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        for c in 0..127 {
            match self.font.load_char(c, freetype::face::LoadFlag::RENDER) {
                Ok(_) => (),
                Err(e) => panic!("Could not load font: {} because of: {}", c, e)
            }

            let glyph = self.font.glyph();

            unsafe {
                let mut texture: u32 = 0;
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as i32, glyph.bitmap().width(), glyph.bitmap().rows(), 0, gl::RED, gl::UNSIGNED_BYTE, glyph.bitmap().buffer().as_ptr() as *const c_void,);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                self.characters.push(Character {
                    letter: c as u8 as char,
                    texture_id: Some(texture),
                    size: (glyph.bitmap().width(), glyph.bitmap().rows())
                });
                gl::PixelStorei(gl::UNPACK_ALIGNMENT, 0);
            }
        }
    }

    pub fn bind(&self, character: char) {
        let character = match self.characters.iter().find(|c| c.letter == character) {
            Some(i) => {i},
            None => {return}
        };

        let texture_id = match character.texture_id {
            Some(i) => i,
            None => panic!("Attempted to use unitialized Texture font!")
        };

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
        }
    }
}