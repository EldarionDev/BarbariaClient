use std::fs;

use crate::{game::Game, maths::Vec3};

use super::Config;
use rand::Rng;
use serde_json::Value;

mod events;
mod graphic;
mod physics;
mod window;
mod game_object;

pub struct Engine<'b> {
    paths: &'b Config,
    game_window: window::Window,
    graphic: graphic::Graphic,
    objects: Vec<graphic::Object>
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
            objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, obj: impl game_object::GameObject) {
        let mut create= true;
        let mut index = 0;

        for i in self.objects.iter() {
            if i.object_name == obj.get_name() {
                create = false;
                break;
            } else {
                index += 1;
            }
        }

        if create {
            index = 0;
            self.register_object(self.graphic.create_object(obj.get_name(), obj.get_type(), obj.get_class()));
            for i in self.objects.iter() {
                if i.object_name == obj.get_name() {
                    break;
                } else {
                    index += 1;
                }
            }
        }

        let object_reference =  self.objects.get_mut(index).unwrap();
        object_reference.add(obj.get_position());
    }

    pub fn remove_object(&mut self, name: &str, pos: Vec3) {
        let mut object_index = 0;
        let mut found = false;
        
        for i in self.objects.iter() {
            if i.object_name == name {
                found = true;
                break;
            } else {
                object_index += 1;
            }
        }

        if found == false {panic!("Object could not be removed because it was not found: {}", name)};
        let object_reference = self.objects.get_mut(object_index).unwrap();
        object_reference.remove(pos);
    }


    pub fn open_title_screen(&mut self) {
        let title_screen_files = self.paths.resource_manager.get_assets("textures");
        let title_screen_count = title_screen_files.len();
        let random_title_screen = rand::thread_rng().gen_range(0..(title_screen_count - 1));
        let random_title_screen = &title_screen_files[random_title_screen];
        let random_title_screen = random_title_screen.split('.').next().unwrap();
        self.register_object(self.graphic.create_object(random_title_screen, &graphic::ObjectType::Dimension2, &graphic::ObjectClass::GUI))
    }

    pub fn render_tick(&mut self) {
        let object_reference = &mut self.objects;

        let graphic_reference = &mut self.graphic;

        for i in object_reference.iter() {
            graphic_reference.draw(i);
        }

        let window_reference = &mut self.game_window;
        window_reference.update();
    }

    fn register_object(&mut self, obj: graphic::Object) {
        self.objects.push(obj);
    }

    fn unregister_object(&mut self, name: &str) {
        let object_reference = &mut self.objects;

        let mut index = 0;
        let mut found = false;
        for i in object_reference.iter() {
            if i.object_name == name {
                found = true;
                break;
            } else {
                index += 1;
            }
        }

        if found {
            object_reference.remove(index);
        } else {
            panic!("Could not remove specified object: {}", name);
        }
    }
}

impl<'a> Drop for Engine<'a> {
    fn drop(&mut self) {}
}