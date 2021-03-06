use std::{cell::{RefCell, RefMut}, rc::Rc, sync::mpsc::Receiver};

pub struct EventHandler {
    window_event_handler: Receiver<(f64, glfw::WindowEvent)>,
    glfw_instance: glfw::Glfw,
    event_functions: Vec<fn()>,
    event_objects: Vec<Rc<RefCell<dyn Listener>>>,
    current_cursor_pos: (f64, f64),
    mouse_clicked: bool
}

impl EventHandler {
    pub fn new(window_event_handler: Receiver<(f64, glfw::WindowEvent)>, glfw_instance: glfw::Glfw) -> EventHandler{
        EventHandler {
            window_event_handler,
            glfw_instance,
            event_functions: Vec::new(),
            event_objects: Vec::new(),
            current_cursor_pos: (0.0, 0.0),
            mouse_clicked: false
        }
    }

    pub fn register_event_function(&mut self, listener_function: fn()) {
        self.event_functions.push(listener_function);
    }

    pub fn register_event_object(&mut self, object: Rc<RefCell<dyn Listener>>) {
        self.event_objects.push(object);
    }

    pub fn process_events(&mut self) {
        /* Process Window events */
        self.glfw_instance.poll_events();
        for(_, event) in glfw::flush_messages(&self.window_event_handler) {
            match event {
                glfw::WindowEvent::Close => {
                    for o in self.event_objects.iter() {
                        o.borrow_mut().window_closed();
                    }
                }

                glfw::WindowEvent::MouseButton(_, a, _) => {
                    match a {
                        glfw::Action::Press => {if self.mouse_clicked == true {continue;} else {self.mouse_clicked = true}},
                        glfw::Action::Release => {self.mouse_clicked = false; continue;},
                        glfw::Action::Repeat => {continue;}
                    }

                    for o in self.event_objects.iter() {
                        o.borrow_mut().mouse_clicked(self.current_cursor_pos);
                    }
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    self.current_cursor_pos = (x, y);
                }

                glfw::WindowEvent::Key(_, _, _, _) => {
                    for o in self.event_objects.iter() {
                        o.borrow_mut().key_pressed();
                    }
                }
                _ => {}
            }
        }
    }

    pub fn do_game_tick(&mut self) {
        
    }
}

pub trait Listener {
    fn key_pressed(&mut self);
    fn mouse_clicked(&mut self, cursor_pos: (f64, f64));
    fn window_closed(&mut self);
}