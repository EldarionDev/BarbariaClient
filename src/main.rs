use std::{clone, env};

mod resource_manager;
mod game;
mod engine;
mod maths;

pub struct Config {
    resource_manager: resource_manager::ResourceManager,
}

fn main() {
    let cmd_args: Vec<String> = env::args().collect();

    /* In final executable 
    let asset_path: &str = &cmd_args[1];
    let config_path: &str = &cmd_args[2];
    let data_path: &str = &cmd_args[3]; */

    /* Testing */
    let asset_path: &str = "assets/";
    let config_path: &str = "config/";
    let data_path: &str = "data/";

    let mut program_config = Config {
        resource_manager: resource_manager::ResourceManager::new(asset_path, config_path, data_path)
    };
    program_config.resource_manager.set_world("world/");

    let mut game_engine = engine::Engine::new(&program_config);
    game_engine.open_title_screen();
    let game = game::Game::new(&program_config);
    game.load_world();
    game.save_world();

    loop {
        unsafe {
            gl::ClearColor(0.6, 0.3, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        
        game_engine.render_tick();
    }
}