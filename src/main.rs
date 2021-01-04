use std::env;

mod resource_manager;
mod game;
mod engine;

struct Config {
    resource_manager: resource_manager::ResourceManager,
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    let asset_path: &str = &cmd_args[1];
    let config_path: &str = &cmd_args[2];

    let program_config = Config {
        resource_manager: resource_manager::ResourceManager::new(asset_path, config_path)
    };
}