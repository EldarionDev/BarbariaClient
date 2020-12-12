use glfw::Context;

pub struct GameWindow {
    resolution: (u32, u32),
    title: String,
    glfw_window: glfw::Window,
}

impl GameWindow {
    pub fn new(res_y: u32, res_x: u32, window_title: &str, glfw_instance: &mut glfw::Glfw) -> Result<Self, glfw::Error> {
        glfw_instance.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw_instance.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        let (mut window, _) = glfw_instance.create_window(res_x, res_y, window_title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        
        Ok(GameWindow {
            resolution: (res_x, res_y),
            title: window_title.to_string(),
            glfw_window: window,
        })
    }

    pub fn update_window(&mut self) {
        self.glfw_window.swap_buffers();
    }
}

impl Drop for GameWindow {
    fn drop(&mut self) {
        drop(&self.glfw_window);
        drop(&self.title);
    }
}