use std::{cell::{RefCell, RefMut}, rc::Rc, time::{Instant, SystemTime}};

mod engine;
mod game;
mod resource_manager;

extern crate nalgebra_glm as glm;

#[derive(Clone)]
pub struct Config {
    resource_manager: resource_manager::ResourceManager,
}

fn main() {
    /* In final executable
    let cmd_args: Vec<String> = env::args().collect();
    let asset_path: &str = &cmd_args[1];
    let config_path: &str = &cmd_args[2];
    let data_path: &str = &cmd_args[3]; */

    /* Testing */
    let asset_path: &str = "assets/";
    let config_path: &str = "config/";
    let data_path: &str = "data/";
    
    let mut start = Instant::now();

    let mut program_config = Config {
        resource_manager: resource_manager::ResourceManager::new(
            asset_path,
            config_path,
            data_path,
        ),
    };
    program_config.resource_manager.set_world("world/");

    let mut game_engine = engine::Engine::new(&program_config);
    
    let game = game::Game::new(program_config.clone(), (game_engine.game_window.size_x as f32, game_engine.game_window.size_y as f32));
    let game = Rc::new(RefCell::new(game));

    //game_engine.open_title_screen();

    game.borrow_mut().load_world();

    game_engine.event_handler.register_event_object(game.clone());
    //game_engine.register_render_text("prince_valiant".to_string(), "testtesttesttesttesttest".to_string(), (0.9, 0.9, 0.9), (250.0, 250.0), 1.0);

    game.borrow_mut().open_screen("main_menu", &mut game_engine);

    while !game.borrow().close {
        let elapsed = start.elapsed().as_millis();
        if elapsed < 16 {
            continue;
        }
        start = Instant::now();

        unsafe {
            gl::ClearColor(0.5, 0.5, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        game_engine.render_tick();
        game.borrow_mut().game_tick(&mut game_engine, &program_config);
    }

    game.borrow().save_world();
    return;
}