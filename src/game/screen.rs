use serde::{Deserialize, Serialize};
use crate::engine::{self, game_object::gui};

use super::listener::{self, Listener};

#[derive(Deserialize, Serialize, Clone)]
pub struct Screen {
    pub name: String,
    position: (f32, f32),
    scale: (f32, f32),
    background: String,
    adjust_to_screen: bool,
    text_elements: Vec<TextElement>,
    texture_elements: Vec<TextureElement>,
    event_text_elements: Vec<TextElement>,
    event_texture_elements: Vec<TextureElement>,
    gui: Option<gui::Gui>
}

impl Screen {
    pub fn open(&mut self, engine: &mut engine::Engine) {
        let mut gui = gui::Gui::new(self.scale, self.position, self.adjust_to_screen);
        
        if self.background != "" {
            gui.add_background(engine, &self.background[..], (0.0, 0.0),(1000.0, 1000.0));
        }

        for e in &self.texture_elements {
            gui.add_element(engine, &e.name[..], e.position, e.size);
        }

        for e in &self.text_elements {
            gui.add_text(engine, e.position, e.fontsize, &e.font[..], &e.text[..], e.color);
        }

        self.gui = Some(gui);
    }

    pub fn close(&self) {

    }

    pub fn key_pressed(&self) {

    }

    pub fn window_closed(&self) {

    }

    pub fn render_event_texture(&mut self, engine: &mut engine::Engine, element_name: &str) {
        let index: usize = element_name.parse().expect("Not a valid index");
        let texture_element = self.event_texture_elements.get(index).unwrap();
        self.gui.as_mut().unwrap().add_element(engine, &texture_element.name, texture_element.position, texture_element.size)
    }

    pub fn render_event_text(&mut self, engine: &mut engine::Engine, element_name: &str) {
        let index: usize = element_name.parse().expect("Not a valid index");
        let text_element = self.event_text_elements.get(index).unwrap();
        self.gui.as_mut().unwrap().add_text(engine, text_element.position, text_element.fontsize, &text_element.font, &text_element.text, text_element.color);
    }

    pub fn mouse_clicked(&self, listener: &mut Listener, cursor_pos: (f64, f64), screen_size: (f32, f32)) {
        /* TODO: Fix calculation to new orthogonal matrix */
        let cursor_pos = (cursor_pos.0 as f32, screen_size.1 - cursor_pos.1 as f32);
        let gui_position = self.gui.clone().unwrap().position.clone();
        let gui_size = self.gui.clone().unwrap().size.clone();


        /* Check if mouse-click is within screen */
        let x = (1000.0 / screen_size.0) * cursor_pos.0;
        let y = (1000.0 / screen_size.1) * cursor_pos.1;
        if self.position.0 > x || x > (self.position.0 + self.scale.0) || self.position.1 > y || y > (self.position.1 + self.scale.1) {
            return;
        }
        
        
        for element in (&self.texture_elements).iter().rev() {
            let mut x = 0;
            let mut y = 0;
            
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TextElement {
    position: (f32, f32),
    color: (f32, f32, f32),
    fontsize: f32,
    font: String,
    text: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TextureElement {
    position: (f32, f32),
    size: (f32, f32),
    name: String,
    pub event_codes: Vec<String>
}