use std::fs;

use super::Config;
use rand::Rng;
use serde_json::Value;

mod graphic;
mod physics;
mod events;
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
            game_window: window::Window::new(json_content["screenWidth"].as_u64().unwrap() as u32, json_content["screenHeight"].as_u64().unwrap() as u32),
            graphic: graphic::Graphic::new(paths),
            objects: None
        }
    }

    pub fn open_title_screen(&mut self) {
        let title_screen_files = self.paths.resource_manager.get_assets("textures");
        let title_screen_count = title_screen_files.len();
        let random_title_screen = rand::thread_rng().gen_range(0..(title_screen_count-1));
        let random_title_screen = &title_screen_files[random_title_screen];
        let random_title_screen = random_title_screen.split('.').next().unwrap();
        self.objects = Some(vec![self.graphic.create_object(random_title_screen, graphic::ObjectType::Dimension2, graphic::ObjectClass::GUI)]);
    }

    pub fn render_tick(&mut self) {
        for i in self.objects.clone().unwrap() {
            self.graphic.clone().draw(i);
        }
        self.game_window.update();
    }
}

impl<'a> Drop for Engine<'a> {
    fn drop(&mut self) {
        
    }
}