use std::{ffi::c_void, path::Path};
use image::GenericImage;

#[derive(Clone)]
pub struct Texture {
    texture_id: u32
}

impl Texture {
    pub fn new(texture_path: String) -> Texture {
        /* Create OpenGL texture */
        let mut opengl_texture: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut opengl_texture);
            gl::BindTexture(gl::TEXTURE_2D, opengl_texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        }

        /* Load texture as image */
        let image = image::open(&Path::new(&texture_path)).expect("Failed to load texture");
        let image_data = image.raw_pixels();

        /* Attach image to OpenGL texture */
        unsafe {
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, image.width() as i32, image.height() as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, &image_data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        /* Assign values and return */
        Texture {
            texture_id: opengl_texture
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }   
    }
}