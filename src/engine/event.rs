use std::{cell::{RefCell, RefMut}, rc::Rc, sync::mpsc::Receiver};

pub struct EventHandler {
    window_event_handler: Receiver<(f64, glfw::WindowEvent)>,
    glfw_instance: glfw::Glfw,
    event_functions: Vec<fn()>,
    event_objects: Vec<Rc<RefCell<dyn Listener>>>
}

impl EventHandler {
    pub fn new(window_event_handler: Receiver<(f64, glfw::WindowEvent)>, glfw_instance: glfw::Glfw) -> EventHandler{
        EventHandler {
            window_event_handler,
            glfw_instance,
            event_functions: Vec::new(),
            event_objects: Vec::new()
        }
    }

    pub fn register_event_function(&mut self, listener_function: fn()) {
        self.event_functions.push(listener_function);
    }

    pub fn register_event_object(&mut self, object: Rc<RefCell<dyn Listener>>) {
        self.event_objects.push(object);
    }

    pub fn trigger_event_listeners(&mut self) {
        /* Process Window events */
        self.glfw_instance.poll_events();
        for(_, event) in glfw::flush_messages(&self.window_event_handler) {
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    /* Assumes the registered event is always existing!!!!!!! */
                    /* Later return slice to unregister event */
                    self.event_objects[0].borrow_mut().key_pressed();
                },
                _ => {},
            }
        }
    }
}

pub trait Listener {
    fn key_pressed(&mut self);
    fn mouse_clicked(&mut self);
}