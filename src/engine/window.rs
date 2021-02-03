use std::sync::mpsc::Receiver;

use glfw::Context;

pub struct Window {
    window: glfw::Window,
    pub size_x: u32,
    pub size_y: u32
}

impl Window {
    pub fn new(screen_size_x: u32, screen_size_y: u32, window_event: &mut Option<Receiver<(f64, glfw::WindowEvent)>>, glfw_instance: &mut Option<glfw::Glfw>) -> Window {
        let mut glfw = match glfw::init(glfw::FAIL_ON_ERRORS) {
            Ok(g) => g,
            Err(e) => panic!("Could not initialize GLFW: {}", e),
        };

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 4));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, event) = glfw
            .create_window(
                screen_size_x,
                screen_size_y,
                "Third Age Reforged",
                glfw::WindowMode::Windowed,
            )
            .expect("Could not create window.");

        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        *window_event = Some(event);
        *glfw_instance = Some(glfw);

        Window { 
            window, 
            size_x: screen_size_x,
            size_y: screen_size_y
        }
    }

    pub fn update(&mut self) {
        self.window.swap_buffers();
    }
}
