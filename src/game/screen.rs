use serde::{Deserialize, Serialize};
use crate::engine::{self, game_object::gui};

use super::listener::{self, Listener};

#[derive(Deserialize, Serialize, Clone)]
pub struct Screen {
    pub name: String,
    position: (f32, f32),
    scale: (f32, f32),
    background: String,
    text_elements: Vec<TextElement>,
    texture_elements: Vec<TextureElement>,
    gui: Option<gui::Gui>
}

impl Screen {
    pub fn open(&mut self, engine: &mut engine::Engine) {
        let mut gui = gui::Gui::new(self.scale, self.position);
        
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

    pub fn mouse_clicked(&self, listener: &mut Listener, cursor_pos: (f64, f64), screen_size: (f32, f32)) {
        /* TODO: Fix calculation to new orthogonal matrix */
        let cursor_pos = (cursor_pos.0 as f32, cursor_pos.1 as f32);
        let x = (1000.0 / screen_size.0) * cursor_pos.0;
        let y = (1000.0 / screen_size.1) * (screen_size.1 - cursor_pos.1); 
        let gui_position = self.gui.clone().unwrap().position.clone();
        let gui_size = self.gui.clone().unwrap().size.clone();

        if self.position.0 < x && x < ((self.position.0 + self.scale.0)) {
            if self.position.1 < y && y < ((self.position.1 + self.scale.1)) {
                let x = (x - gui_position.0) *  (gui_size.0 / 1000.0) + gui_position.0;
                let y = (y - gui_position.1) *  (gui_size.1 / 1000.0) + gui_position.1;

                for element in (&self.texture_elements).iter().rev() {
                    let element_x = (element.position.0 - gui_position.0) *  (gui_size.0 / 1000.0) + gui_position.0;
                    let element_x_end = (element.position.0 - gui_position.0) *  (gui_size.0 / 1000.0) + gui_position.0 + (element.size.0 * (gui_size.0 / 1000.0));
                    let element_y = (element.position.1 - gui_position.1) *  (gui_size.1 / 1000.0) + gui_position.1;
                    let element_y_end = (element.position.1 - gui_position.1) *  (gui_size.1 / 1000.0) + gui_position.1 + (element.size.1 * (gui_size.1 / 1000.0));

                    if element_x < x && x < element_x_end {
                        if element_y < y && y < element_y_end {
                            for e in element.event_codes.iter() {
                                if e == "" {
                                    continue;
                                }
                                listener.event_codes.push(e.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TextElement {
    event_name: String,
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