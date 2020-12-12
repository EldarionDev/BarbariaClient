pub struct GameWindow {
    resolution: (f32, f32),
    title: String,
    glfw_window: glfw::Window,
}

impl GameWindow {
    pub fn new(resX: f32, resY: f32, window_title: &str, glfw_instance: &mut glfw::Glfw) -> Result<Self, glfw::Error> {
        let tmp: String =  window_title.to_string();
        
        glfw_instance.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw_instance.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        let (mut window, _) = glfw_instance.create_window(1920, 1080, "The Fourth Age", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window");
        window.set_key_polling(true);
        window.set_framebuffer_size_polling(true);
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
        
        Ok(GameWindow {
            resolution: (resX, resY),
            title: String::clone(&tmp),
            glfw_window: window,
        })
    }
}

impl Drop for GameWindow {
    fn drop(&mut self) {
        drop(&self.glfw_window);
        drop(&self.title);
    }
}