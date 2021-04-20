use std::{borrow::BorrowMut, fs, fs::File, io::Write, ops::Deref};

use fs::read_to_string;
use screen::TextureElement;

use crate::engine::{self, event::Listener};

use super::Config;

mod faction;
mod map;
mod screen;
mod listener;

pub struct Game {
    /* Remove Option when JSON loading is implemented */
    factions: Option<Vec<faction::Faction>>,
    player_faction: Option<faction::Faction>,
    map: Option<map::Map>,
    paths: Config,
    pub close: bool,
    screens: Vec<screen::Screen>,
    open_screens: Vec<screen::Screen>,
    listener: listener::Listener,
    screen_size: (f32, f32),
    event_string: String
}

impl Game {
    pub fn new(paths: Config, screen_size: (f32, f32)) -> Self {
        let mut screens: Vec<screen::Screen> = Vec::new();
        
        for (_, screen_path) in paths.resource_manager.get_assets("screens").iter().enumerate() {
            let screen_path_content = &match fs::read_to_string(screen_path) {
                Ok(f) => f,
                Err(e) => panic!("Could not read JSON of faction file because: {}", e)
            }[..];

            let screen: screen::Screen = match serde_json::from_str(screen_path_content) {
                Ok(s) => s,
                Err(e) => panic!("Could not create Screen from json: {}", e)
            };

            screens.push(screen);
        }

        /* Temporary till JSON loading */
        Game {
            factions: None,
            player_faction: None,
            map: None,
            paths: paths,
            close: false,
            screens,
            open_screens: Vec::new(),
            listener: listener::Listener::new(),
            screen_size,
            event_string: "".to_string()
        }
    }

    pub fn open_screen(&mut self, name: &str, engine: &mut engine::Engine) {
        for s in &mut self.screens {
            if s.name == name {
                s.open(engine, self.paths.clone());
                self.open_screens.push((*s).clone());
            }
        }
    }

    pub fn load_world(&self, player_faction_name: &str) {
        println!("Starting game, player has chosen: {}", player_faction_name);
        for faction_file in self.paths.resource_manager.get_world_data("factions") {
            let faction_file_content = match fs::read_to_string(faction_file) {
                Ok(f) => f,
                Err(e) => panic!("Could not open faction file because of: {}", e),
            };

            let faction_file_content: &str = &faction_file_content[..];

            let _: faction::Faction = match serde_json::from_str(faction_file_content) {
                Ok(f) => f,
                Err(e) => panic!("Could not create faction from JSON: {}", e),
            };
        }
    }

    pub fn game_tick(&mut self, engine: &mut engine::Engine, paths: &Config) {
        let event_codes = self.listener.event_codes.clone();
        for s in &event_codes[..] {
            if s == "" {
                continue;
            }

            if s == "exit" {
                self.close = true;
                return;
            }

            if s == "close" {
                match self.screens.last() {
                    Some(i) => i,
                    None => panic!("Critical error occurred!")
                }.close();
            }

            if s.starts_with("open") {
                let split_string = s.split(" ");
                self.open_screen(split_string.last().expect("No screen to open specified"), engine);
            }

            if s.starts_with("render") {
                let mut split_string1 = s.split(" ");
                let mut split_string2 = s.split(" ");
                match split_string1.find(|&x| x == "texture") {
                    Some(_) => {self.screens.last_mut().unwrap().render_event_texture(engine, split_string1.last().unwrap()); continue;},
                    None => {}
                };
                match split_string2.find(|&x| x == "text") {
                    Some(_) => {self.screens.last_mut().unwrap().render_event_text(engine, split_string2.last().unwrap(), paths); continue;},
                    None => panic!("Neither texture nor text specified")
                }
            }

            if s.starts_with("set") {
                let split_string = s.split(" ");
                self.event_string = split_string.last().expect("set command used without parameters.").to_string();
            }

            if s.starts_with("play") {
                if self.event_string == "" {panic!("Used play command without setting the player faction.")}
                self.load_world(&self.event_string);
            }

            match &mut self.map {
                Some(i) => i,
                None => continue
            }.retreive_event_code(&s[..]);

            match &mut self.player_faction {
                Some(i) => i,
                None => continue
            }.retreive_event_code(&s[..]);
        }
        self.listener.event_codes.clear();
    }

    pub fn save_world(&self) {
        let file =
            File::create(self.paths.resource_manager.get_world().to_owned() + "test_faction.json");
        let f = faction::Faction::new("Hello there".to_string());
        let json_data = serde_json::to_string(&f);

        let mut save_file = match file {
            Ok(f) => f,
            Err(e) => panic!("Could not create save file while saving world: {}", e),
        };

        let save_data = match json_data {
            Ok(s) => s,
            Err(e) => panic!("Could not proceed JSON data while saving world: {}", e),
        };

        match save_file.write_all(save_data.as_bytes()) {
            Ok(_) => (),
            Err(e) => panic!("Error while saving world: {}", e),
        }
    }

    pub fn push_event_code(&mut self, code: &str) {
        self.listener.event_codes.push(code.to_string());
    }
}


impl Listener for Game {
    fn key_pressed(&mut self) {
    
    }

    fn mouse_clicked(&mut self, cursor_pos: (f64, f64)) {
        let screen = match self.open_screens.last() {
            Some(i) => i,
            None => return
        };
        screen.mouse_clicked(&mut self.listener, cursor_pos,  self.screen_size);
    }

    fn window_closed(&mut self) {
        self.close = true;
    }
}