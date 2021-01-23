use std::sync::mpsc::Receiver;

pub struct EventHandler {
    window_event_handler: Receiver<(f64, glfw::WindowEvent)>,
    glfw_instance: glfw::Glfw
}

impl EventHandler {
    pub fn new(window_event_handler: Receiver<(f64, glfw::WindowEvent)>, glfw_instance: glfw::Glfw) -> EventHandler{
        EventHandler {
            window_event_handler,
            glfw_instance
        }
    }

    pub fn trigger_event_listeners(&mut self) {
        /* Process Window events */
        self.glfw_instance.poll_events();
        for(_, event) in glfw::flush_messages(&self.window_event_handler) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    println!("Stopping game!")
                },
                _ => {},
            }
        }
    }
}