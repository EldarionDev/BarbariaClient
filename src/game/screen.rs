use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Screen {
    name: String,
    position: (f32, f32),
    scale: (f32, f32),
    background: String,
    text_elements: Vec<TextElement>,
    texture_elements: Vec<TextureElement>
}

impl Screen {
    pub fn load() {

    }

    pub fn close() {

    }

    pub fn key_pressed() {

    }

    pub fn window_closed() {

    }

    pub fn mouse_clicked() {
        
    }
}

#[derive(Deserialize, Serialize)]
pub struct TextElement {
    position: (f32, f32),
    fontsize: (f32, f32),
    font: String,
    text: String
}

#[derive(Deserialize, Serialize)]
pub struct TextureElement {
    position: (f32, f32),
    size: (f32, f32),
    name: String
}