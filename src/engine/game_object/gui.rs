use crate::engine;
use super::{GameObject, coordinate_transform::to_gl_space, gui_element};
use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct Gui {
    pub size: (f32, f32),
    pub position: (f32, f32),
}

impl Gui {
    pub fn new(size: (f32, f32), position: (f32, f32)) -> Gui{
        Gui {
            size,
            position
        }
    }

    pub fn add_background(&mut self, engine: &mut engine::Engine, element_name: &str, position: (f32, f32), size: (f32, f32)) {
        let aspect_x = 1920.0 / 1000.0;
        let aspect_y = 1080.0 / 1000.0;

        let screen_scale_x = engine.game_window.size_x as f32 / 1920.0;
        let screen_scale_y = engine.game_window.size_y as f32 / 1080.0;

        let position = (position.0 * aspect_x * screen_scale_x, position.1 * aspect_y * screen_scale_y);
        let size = (size.0 * aspect_x * screen_scale_x, size.1 * aspect_y * screen_scale_y);
        engine.register_render_object(element_name.to_string(), glm::vec3(position.0, position.1, 0.0), 
        glm::vec3(0.0, 0.0, 0.0), 0.0, glm::vec3(size.0, size.1, 1.0));
    }

    pub fn add_element(&mut self, engine: &mut engine::Engine, element_name: &str, position: (f32, f32), size: (f32, f32)) {
        let aspect_x = 1920.0 / 1000.0;
        let aspect_y = 1080.0 / 1000.0;

        let screen_scale_x = engine.game_window.size_x as f32 / 1920.0;
        let screen_scale_y = engine.game_window.size_y as f32 / 1080.0;

        /* self.size.0 / 1000.0 is a factor how often the GUI size fits into the screen size. */
        let pos_x = ((self.size.0 / 1000.0) * position.0  - 500.0) * aspect_x * screen_scale_x;
        let pos_y = ((self.size.1 / 1000.0) * position.1 - 500.0) * aspect_y * screen_scale_y;

        let size_x = (self.size.0 / 1000.0) * size.0 * screen_scale_x;
        let size_y = (self.size.1 / 1000.0) * size.1 * screen_scale_y; 
        
        engine.register_render_object(element_name.to_string(), glm::vec3(pos_x, pos_y, 0.0), 
        glm::vec3(0.0, 0.0, 0.0), 0.0, glm::vec3(size_x, size_y, 1.0));
    }

    pub fn add_text(&mut self, engine: &mut engine::Engine, position: (f32, f32), font_size: f32, font_name: &str, text: &str, color: (f32, f32, f32)) {
        let aspect_x = engine.game_window.size_x as f32 / 1000.0;
        let aspect_y = engine.game_window.size_y as f32 / 1000.0;

        /* self.size.0 / 1000.0 is a factor how often the GUI size fits into the screen size. */
        let pos_x = ((self.size.0 / 1000.0) * position.0  - 500.0) * aspect_x;
        let pos_y = ((self.size.1 / 1000.0) * position.1 - 500.0) * aspect_y;
        
        engine.register_render_text(font_name.to_string(), text.to_string(), color, (pos_x, pos_y), font_size);
    }
}