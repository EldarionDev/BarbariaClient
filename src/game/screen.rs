use serde::{Deserialize, Serialize};
use crate::{Config, engine::{self, game_object::gui}};

use super::listener::{self, Listener};

#[derive(Deserialize, Serialize, Clone)]
pub struct Screen {
    pub name: String,
    position: (f32, f32),
    scale: (f32, f32),
    background: String,
    text_elements: Vec<TextElement>,
    texture_elements: Vec<TextureElement>,
    event_text_elements: Vec<TextElement>,
    event_texture_elements: Vec<TextureElement>,
    gui: Option<gui::Gui>
}

impl Screen {
    pub fn open(&mut self, engine: &mut engine::Engine, paths: Config) {
        let mut gui = gui::Gui::new(self.scale, self.position);
        
        if self.background != "" {
            gui.add_background(engine, &self.background[..], (0.0, 0.0),(1000.0, 1000.0));
        }

        for e in &self.texture_elements {
            gui.add_element(engine, &e.name[..], e.position, e.size);
        }

        for e in &self.text_elements {
            for i in paths.resource_manager.get_assets("texts") {
                let name = i.split('/').last().unwrap().to_string();
                let name = name.split('.').next().unwrap();
                if name == e.text {
                    let file_c = std::fs::read_to_string(i).expect("Could not read text file.");
                    gui.add_text(engine, e.position, e.fontsize, &e.font[..], &file_c, e.color);
                    break;
                }
            }
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
        /* y-Coordinates are upside down */
        let cursor_pos = (cursor_pos.0, screen_size.1 as f64 - cursor_pos.1);

        let aspect_x = (screen_size.0 / 1000.0) as f64;
        let aspect_y = (screen_size.1 / 1000.0) as f64;

        /* Transform mouse coordinates to screen coordinates */
        let cursor_pos = ((cursor_pos.0 / aspect_x) as f32 , (cursor_pos.1 / aspect_y) as f32);

        /* Check if mouse is in range of GUI */
        if cursor_pos.0 >= self.position.0 && cursor_pos.0 <= self.position.0 + self.scale.0 && cursor_pos.1 >= self.position.1 && cursor_pos.1 <= self.position.1 + self.scale.1 {
            
            for element in (&self.texture_elements).iter().rev() {
                let element_x = (element.position.0 - self.position.0) *  (self.scale.0 / 1000.0) + self.position.0;
                let element_x_end = (element.position.0 - self.position.0) *  (self.scale.0 / 1000.0) + self.position.0 + (element.size.0 * (self.scale.0 / 1000.0));
                let element_y = (element.position.1 - self.position.1) *  (self.scale.1 / 1000.0) + self.position.1;
                let element_y_end = (element.position.1 - self.position.1) *  (self.scale.1 / 1000.0) + self.position.1 + (element.size.1 * (self.scale.1 / 1000.0));

                if element_x < cursor_pos.0 && cursor_pos.0 < element_x_end {
                    if element_y < cursor_pos.1 && cursor_pos.1 < element_y_end {
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