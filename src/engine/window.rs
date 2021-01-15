use glfw::{Context};

use crate::Config;

pub struct Window {
    window: glfw::Window
}

impl Window {
    pub fn new(paths: &Config) -> Window{
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(1920, 1070, "Third Age Reforged", glfw::WindowMode::Windowed).expect("Could notcreate window.");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window {
            window
        }
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
    }
}