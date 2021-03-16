use std::{fs, time::{SystemTime, UNIX_EPOCH}};

use crate::{game::Game};
use game_object::{gui, gui_element, GameObject};
use glm::{Vec3, Vec4};

use self::graphic::RenderObject;

use super::Config;
use rand::Rng;
use serde_json::Value;
use std::sync::mpsc::Receiver;

pub(crate) mod event;
pub(crate) mod game_object;
mod graphic;
mod physics;
mod window;

pub struct Engine<'b> {
    paths: &'b Config,
    pub game_window: window::Window,
    graphic: graphic::Graphic<'b>,
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
            event_handler: event::EventHandler::new(receiver.unwrap(), glfw.unwrap())
        }
    }

    pub fn register_render_object(&mut self, render_name: String, position: Vec3, rotation: Vec3, rotation_angle: f32, scale: Vec3) -> String{
        /* Later generate name with current system time and return the string */
        let name = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().to_string();
        let return_name = name.clone();

        self.graphic.add_object(render_name, RenderObject::new(name, position, rotation, rotation_angle, scale));

        return_name
    }

    pub fn unregister_render_object() {

    }

    pub fn render_tick(&mut self) {
        self.graphic.render();
        self.game_window.update();
        self.event_handler.process_events();
    }
}

impl<'a> Drop for Engine<'a> {
    fn drop(&mut self) {}
}