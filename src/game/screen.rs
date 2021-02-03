use serde::{Deserialize, Serialize};
use crate::engine::{self, game_object::gui};

#[derive(Deserialize, Serialize, Clone)]
pub struct Screen {
    pub name: String,
    position: (f32, f32),
    scale: (f32, f32),
    background: String,
    text_elements: Vec<TextElement>,
    texture_elements: Vec<TextureElement>
}

impl Screen {
    pub fn open(&self, engine: &mut engine::Engine) {
        let mut gui = gui::Gui::new(self.scale, self.position);
        
        if self.background != "" {
            gui.add_background(engine, &self.background[..], (0.0, 0.0),(1000.0, 1000.0));
        }

        for e in &self.texture_elements {
            gui.add_element(engine, &e.name[..], e.position, e.size);
        }
    }

    pub fn close(&self) {

    }

    pub fn key_pressed(&self) {

    }

    pub fn window_closed(&self) {

    }

    pub fn mouse_clicked(&self, cursor_pos: (f64, f64)) {
        println!("pos: {:?}", cursor_pos);
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TextElement {
    event_name: String,
    position: (f32, f32),
    fontsize: (f32, f32),
    font: String,
    text: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct TextureElement {
    position: (f32, f32),
    size: (f32, f32),
    name: String
}