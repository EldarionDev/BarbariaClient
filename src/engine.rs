use std::fs;

use crate::{game::Game, maths::Vec3};

use super::Config;
use rand::Rng;
use serde_json::Value;

mod events;
mod graphic;
mod physics;
mod window;

pub struct Engine<'b> {
    paths: &'b Config,
    game_window: window::Window,
    graphic: graphic::Graphic,
    objects: Option<Vec<graphic::Object>>
}

impl<'b> Engine<'b> {
    pub fn new(paths: &'b Config) -> Engine<'b> {
        //let mut graphic = graphic::Graphic::new(&paths);
        //let object = graphic::Object::new(&mut graphic, "mines", graphic::ObjectType::Dimension2, graphic::ObjectClass::GUI);
        let config_file = paths.resource_manager.get_config("graphics.json");
        let config_file_content: String = fs::read_to_string(config_file).unwrap().parse().unwrap();
        let config_file_content: &str = &config_file_content[..];
        let json_content: Value = serde_json::from_str(config_file_content).unwrap();

        Engine {
            paths,
            game_window: window::Window::new(
                json_content["screenWidth"].as_u64().unwrap() as u32,
                json_content["screenHeight"].as_u64().unwrap() as u32,
            ),
            graphic: graphic::Graphic::new(paths),
            objects: None,
        }
    }

    pub fn register_object(&mut self, obj: graphic::Object) {
        let object_reference = &mut self.objects;
        match object_reference {
            Some(i) => i.push(obj),
            None => self.objects = Some(vec![obj])
        }
    }

    pub fn open_title_screen(&mut self) {
        let title_screen_files = self.paths.resource_manager.get_assets("textures");
        let title_screen_count = title_screen_files.len();
        let random_title_screen = rand::thread_rng().gen_range(0..(title_screen_count - 1));
        let random_title_screen = &title_screen_files[random_title_screen];
        let random_title_screen = random_title_screen.split('.').next().unwrap();
        self.register_object(self.graphic.create_object(random_title_screen, graphic::ObjectType::Dimension2, graphic::ObjectClass::GUI))
    }

    pub fn render_tick(&mut self) {
        let object_reference = &mut self.objects;
        let object_reference = match object_reference {
            Some(i) => i,
            None => {
                println!("Issued draw call though no objects exist.");
                return;
            }
        };

        let graphic_reference = &mut self.graphic;

        for i in object_reference.iter() {
            graphic_reference.draw(i);
        }

        let window_reference = &mut self.game_window;
        window_reference.update();
    }
}

impl<'a> Drop for Engine<'a> {
    fn drop(&mut self) {}
}