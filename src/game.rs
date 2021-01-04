mod faction;
mod map;

struct Game {
    /* Remove Option when JSON loading is implemented */
    factions: Option<Vec<faction::Faction>>,
    map: Option<map::Map>
}

impl Game {
    pub fn new() -> Self{
        /* Temporary till JSON loading */
        Game {
            factions: None,
            map: None
        }
    }

    pub fn load_world() {

    }

    pub fn save_world() {

    }
}
