use std::{fs, fs::File, io::Write};

use super::Config;

mod faction;
mod map;

#[derive(Clone)]
pub struct Game<'a> {
    /* Remove Option when JSON loading is implemented */
    factions: Option<Vec<faction::Faction>>,
    map: Option<map::Map>,
    paths: &'a Config
}

impl<'a> Game<'a> {
    pub fn new(paths: &'a Config) -> Self{
        /* Temporary till JSON loading */
        Game {
            factions: None,
            map: None,
            paths: paths
        }
    }

    pub fn load_world(&self) {
        for faction_file in self.paths.resource_manager.get_world_data("factions") {
            let faction_file_content: String = fs::read_to_string(faction_file).unwrap().parse().unwrap();
            let faction_file_content: &str = &faction_file_content[..];
            let faction: faction::Faction = serde_json::from_str(faction_file_content).unwrap();
        }
    }

    pub fn save_world(&self) {
        let file = File::create(self.paths.resource_manager.get_world().to_owned() + "test_faction.json");
        let f = faction::Faction::new("Hello there".to_string());
        let json_data = serde_json::to_string(&f);
        file.unwrap().write_all(json_data.unwrap().as_bytes());
    }
}
