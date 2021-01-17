use fs::ReadDir;
use std::fs;

/* Asset path contains all assets. Config path contains the game config while data path contains
the object information. */
#[derive(Clone)]
pub struct ResourceManager {
    asset_path: String,
    config_path: String,
    data_path: String,
    world_path: Option<String>,
}

impl ResourceManager {
    pub fn new(asset_path: &str, config_path: &str, data_path: &str) -> Self {
        ResourceManager {
            asset_path: asset_path.to_owned(),
            config_path: config_path.to_owned(),
            data_path: data_path.to_owned(),
            world_path: None,
        }
    }

    pub fn set_world(&mut self, world_path: &str) {
        self.world_path = Some(world_path.to_owned());
    }

    pub fn get_world(&self) -> String {
        let world_path = match self.world_path.to_owned() {
            Some(i) => i,
            None => panic!("World path could not be converted to string."),
        };
        return world_path.clone();
    }

    /* Functions to return the asset paths */
    pub fn get_assets(&self, assets_name: &str) -> Vec<String> {
        let shader_path = self.asset_path.to_string() + assets_name + "/";
        return self.return_files(&shader_path);
    }

    /* Functions to return data */
    pub fn get_map(&self) -> String {
        return self.data_path.to_string() + "/map.json";
    }

    pub fn get_data(&self, data_name: &str) -> Vec<String> {
        let unit_path = self.data_path.to_string() + data_name + "/";
        return self.return_files(&unit_path);
    }

    /* Functions to return world data */
    pub fn get_world_data(&self, data_name: &str) -> Vec<String> {
        let world_data_path = match &self.world_path {
            None => {
                panic!("World has not yet been initialized, but attempted to load data from it")
            }
            Some(i) => i,
        }
        .to_string()
            + data_name
            + "/";

        return self.return_files(&world_data_path);
    }

    /* Function to return a config file */
    pub fn get_config(&self, config_name: &str) -> String {
        let path = self.config_path.clone() + config_name;
        return path;
    }

    fn return_files<'b>(&self, path: &str) -> Vec<String> {
        let mut files = match fs::read_dir(path) {
            Err(_) => panic!("Directory: {} could not be opened!", path),
            Ok(i) => i,
        };
        let mut file_names: Vec<String> = Vec::new();
        self.get_files(&mut files, &mut file_names);
        return file_names;
    }

    fn get_files<'b>(&self, dir: &mut ReadDir, file_names: &mut Vec<String>) {
        for file in dir {
            let path = match file {
                Ok(f) => f,
                Err(e) => panic!("Could not get files of with error: {}", e),
            }
            .path();

            if path.is_dir() {
                let mut recursive_path = match fs::read_dir(path) {
                    Ok(r) => r,
                    Err(e) => panic!("Directory: {} could not be opened!", e),
                };
                self.get_files(&mut recursive_path, file_names);
                continue;
            }

            let final_path = match path.to_str() {
                Some(s) => s,
                None => panic!("Could not convert path to string"),
            }
            .to_string();

            file_names.push(final_path);
        }
    }
}
