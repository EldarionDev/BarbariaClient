use std::{fs, path::PathBuf};
use std::io;

/* Asset path contains all assets. Config path contains the game config while data path contains
the object information. */
pub struct ResourceManager {
    asset_path: String,
    config_path: String,
    data_path: String,
    world_path: Option<&'static str>
}

impl ResourceManager {
    pub fn new(asset_path: &str, config_path: &str, data_path: &str) -> Self {
        ResourceManager {
            asset_path: asset_path.to_owned(),
            config_path: config_path.to_owned(),
            data_path: data_path.to_owned(),
            world_path: None
        }
    }

    pub fn set_world(mut self, world_path: &'static str) {
        self.world_path = Some(world_path);
    }

    /* Functions to return the asset paths */
    pub fn get_assets(self, assets_name: &str) -> Vec<String> {
        let shader_path =  self.asset_path.to_string() + "/" + assets_name + "/";
        return self.get_dir_files(&shader_path);
    }

    /* Functions to return data */
    pub fn get_map(self) -> String {
        return self.data_path + "/map.json";
    }

    pub fn get_data(self, data_name: &str) -> Vec<String> {
        let unit_path = self.data_path.to_string() + "/" + data_name + "/";
        return self.get_dir_files(&unit_path);
    }

    /* Functions to return world data */
    pub fn get_world_data(self, data_name: &str) -> Vec<String> {
        let world_data_path = match self.world_path {
            None => panic!("World has not yet been initialized, but attempted to load data from it"),
            Some(i) => i
        }.to_string() + "/" + data_name + "/";

        return self.get_dir_files(&world_data_path);
    }

    fn get_dir_files<'a>(self, path: &str) -> Vec<String>{
        
        let files = match fs::read_dir(path) {
            Err(_) => panic!("Shader directory could not be opened!"),
            Ok(i) => i
        };

        let mut file_names: Vec<String> = vec![];

        for file in files {
            file_names.push(file.unwrap().path().to_str().unwrap().to_string());
        }

        let mut return_value: Vec<String> = vec![];
        for file_name in file_names {
            let full_path = path.to_string() + &file_name;
            return_value.push(full_path);
        }

        return return_value;
    }
}