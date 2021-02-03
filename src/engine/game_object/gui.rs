use crate::engine;

use super::{GameObject, coordinate_transform::to_gl_space, gui_element};


pub struct Gui {
    size: (f32, f32),
    position: (f32, f32),
}

impl Gui {
    pub fn new(size: (f32, f32), position: (f32, f32)) -> Gui{
        Gui {
            size,
            position,
        }
    }

    pub fn add_background(&mut self, engine: &mut engine::Engine, element_name: &str, position: (f32, f32), size: (f32, f32)) {
        let pos_x = to_gl_space((position.0 / self.size.0) * self.size.0 + self.position.0);
        let pos_y = to_gl_space((position.1 / self.size.1) * self.size.1 + self.position.1);
        let size_x = (1.0 / (1000.0 / self.size.0) * (size.0 / self.size.0)) * 2.0;
        let size_y = (1.0 / (1000.0 / self.size.1) * (size.1 / self.size.1)) * 2.0;
        engine.add_object(gui_element::GuiElement::new(element_name, glm::vec3(pos_x, pos_y, 0.0), 
                                                            glm::vec3(size_x, size_y, 1.0), glm::vec3(0.0, 0.0, 0.0), 0.0));
    }

    pub fn add_element(&mut self, engine: &mut engine::Engine, element_name: &str, position: (f32, f32), size: (f32, f32)) {
        let position = self.aspect_coordinates(position, engine);   
        let size = self.aspect_coordinates(size, engine);
        let pos_x = to_gl_space((position.0 / self.size.0) * self.size.0 + self.position.0);
        let pos_y = to_gl_space((position.1 / self.size.1) * self.size.1 + self.position.1);
        let size_x = (1.0 / (1000.0 / self.size.0) * (size.0 / self.size.0)) * 2.0;
        let size_y = (1.0 / (1000.0 / self.size.1) * (size.1 / self.size.1)) * 2.0;
        engine.add_object(gui_element::GuiElement::new(element_name, glm::vec3(pos_x, pos_y, 0.0), 
                                                            glm::vec3(size_x, size_y, 1.0), glm::vec3(0.0, 0.0, 0.0), 0.0));
    }

    fn aspect_coordinates(&self, coordinates: (f32, f32), engine: &engine::Engine) -> (f32, f32) {
        let aspect_multiplier_x = 1000.0 / (engine.game_window.size_x as f32);
        let aspect_multiplier_y = 1000.0 / (engine.game_window.size_y as f32);
        (aspect_multiplier_x * coordinates.0, aspect_multiplier_y * coordinates.1)
    }
}