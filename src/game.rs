use std::{fs, fs::File, io::Write};

use crate::engine::event::Listener;

use super::Config;

mod faction;
mod map;

#[derive(Clone)]
pub struct Game {
    /* Remove Option when JSON loading is implemented */
    factions: Option<Vec<faction::Faction>>,
    map: Option<map::Map>,
    paths: Config,
    pub close: bool
}

impl Game {
    pub fn new(paths: Config) -> Self {
        /* Temporary till JSON loading */
        Game {
            factions: None,
            map: None,
            paths: paths,
            close: false
        }
    }

    pub fn load_world(&self) {
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
}


impl Listener for Game {
    fn key_pressed(&mut self) {
        self.close = true;
    }

    fn mouse_clicked(&mut self) {
        
    }
}