use std::fs;

use crate::{game::Game, maths::Vec3};
use game_object::{gui_element, GameObject};

use super::Config;
use rand::Rng;
use serde_json::Value;
use std::sync::mpsc::Receiver;

pub(crate) mod event;
mod game_object;
mod graphic;
mod physics;
mod window;

pub struct Engine<'b> {
    paths: &'b Config,
    game_window: window::Window,
    graphic: graphic::Graphic,
    objects: Vec<graphic::Object>,
    pub event_handler: event::EventHandler
}

impl<'b> Engine<'b> {
    pub fn new(paths: &'b Config) -> Engine<'b> {
        //let mut graphic = graphic::Graphic::new(&paths);
        //let object = graphic::Object::new(&mut graphic, "mines", graphic::ObjectType::Dimension2, graphic::ObjectClass::GUI);
        let config_file = paths.resource_manager.get_config("graphics.json");
        let config_file_content: String = fs::read_to_string(config_file).unwrap().parse().unwrap();
        let config_file_content: &str = &config_file_content[..];
        let json_content: Value = serde_json::from_str(config_file_content).unwrap();

        let mut receiver: Option<Receiver<(f64, glfw::WindowEvent)>> = None;
        let mut glfw: Option<glfw::Glfw> = None;

        Engine {
            paths,
            game_window: window::Window::new(
                json_content["screenWidth"].as_u64().unwrap() as u32,
                json_content["screenHeight"].as_u64().unwrap() as u32,
                &mut receiver,
                &mut glfw
            ),
            graphic: graphic::Graphic::new(paths),
            objects: Vec::new(),
            event_handler: event::EventHandler::new(receiver.unwrap(), glfw.unwrap())
        }
    }

    pub fn add_object(&mut self, obj: impl game_object::GameObject) {
        match self
            .objects
            .iter_mut()
            .find(|o| obj.get_name() == o.object_name)
        {
            Some(o) => o,
            None => {
                self.register_object(self.graphic.create_object(
                    obj.get_name(),
                    obj.get_type(),
                    obj.get_class(),
                ));
                self.objects.last_mut().unwrap() 
            }
        }
        .add(obj.get_position());
    }

    pub fn remove_object(&mut self, name: &str, pos: Vec3) {
        match self.objects.iter_mut().find(|o| o.object_name == name) {
            Some(o) => o,
            None => return
        }.remove(&pos);
    }

    pub fn open_title_screen(&mut self) {
        let element = gui_element::GuiElement::new(
            "mines",
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
        self.add_object(element);
    }

    pub fn render_tick(&mut self) {
        let object_reference = &mut self.objects;

        let graphic_reference = &mut self.graphic;

        for i in object_reference.iter() {
            graphic_reference.draw(i);
        }

        let window_reference = &mut self.game_window;
        window_reference.update();
        self.event_handler.trigger_event_listeners();
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
