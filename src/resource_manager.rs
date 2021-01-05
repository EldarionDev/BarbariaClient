use std::{fs, path::PathBuf};
use std::io;

pub struct ResourceManager {
    asset_path: String,
    config_path: String,
}

impl ResourceManager {
    pub fn new(asset_path: &str, config_path: &str) -> Self {
        ResourceManager {
            asset_path: asset_path.to_owned(),
            config_path: config_path.to_owned(),
        }
    }

    pub fn set_world(world_path: &str) {

    }

    pub fn get_shaders(self) -> Vec<String>{
        let shader_path =  self.asset_path.to_string() + "/shaders/";

        return self.get_dir_files(&shader_path);
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