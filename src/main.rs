use std::{cell::{RefCell, RefMut}, rc::Rc, thread, sync::{mpsc, Mutex, Arc}};

mod engine;
mod game;
mod resource_manager;
mod networking;
mod protocol;

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

    let mut program_config = Config {
        resource_manager: resource_manager::ResourceManager::new(
            asset_path,
            config_path,
            data_path,
        ),
    };
    program_config.resource_manager.set_world("world/");

    let program_config_engine_clone = program_config.clone();
    let program_config_game_clone = program_config.clone();

    let (engine_sender, engine_receiver): (mpsc::Sender<Vec<f32>>, mpsc::Receiver<Vec<f32>>) = mpsc::channel();
    let (engine_sender, engine_receiver) = (Arc::new(Mutex::new(engine_sender)), Arc::new(Mutex::new(engine_receiver)));
    let (game_sender, game_receiver): (mpsc::Sender<Vec<f32>>, mpsc::Receiver<Vec<f32>>) = mpsc::channel();
    let (game_sender, game_receiver) = (Arc::new(Mutex::new(game_sender)), Arc::new(Mutex::new(game_receiver)));
    let (network_sender, network_receiver): (mpsc::Sender<Vec<f32>>, mpsc::Receiver<Vec<f32>>) = mpsc::channel();
    let (network_sender, network_receiver) = (Arc::new(Mutex::new(network_sender)), Arc::new(Mutex::new(network_receiver)));

    /* The Engine SENDS user input to the game such as keyboard input,
    it RECEIVES Game events such as what to draw */
    let game_sender_c = Arc::clone(&game_sender);

    let engine_thread = thread::spawn(move || {
        let mut game_engine = engine::Engine::new(program_config_engine_clone);

        loop {
            let mut send = Vec::new();
            let receive = match engine_receiver.lock().unwrap().recv() {
                Ok(i) => i,
                Err(e) => panic!("Could not fetch events for engine loop to execute: {}", e)
            };

            game_engine.render_tick(&mut send, receive);

            match game_sender_c.lock().unwrap().send(send) {
                Ok(i) => i,
                Err(e) => panic!("Could not send engine events to the game: {}", e)
            }
        }
    });

    /* The Game SENDS data to the engine such as what to draw,
    the Game SENDS data to the Network to inform the other players client what this player has done
    the Game RECEIVES user input by the engine such as keyboard input,
    the Game RECEIVES networking events to perform changes done by the other player */
    let engine_sender_c = Arc::clone(&engine_sender);
    let network_sender_c = Arc::clone(&network_sender);

    let game_thread = thread::spawn(move || {
        /* Later let game load screen size itself */
        let mut game = game::Game::new(program_config_game_clone);
        
        loop {
            let mut send = Vec::new();
            let receive = match game_receiver.lock().unwrap().recv() {
                Ok(i) => i,
                Err(e) => panic!("Could not fetch events for engine loop to execute: {}", e)
            };

            game.game_tick(&mut send, receive);

            match engine_sender_c.lock().unwrap().send(send.clone()) {
                Ok(i) => i,
                Err(e) => panic!("Could not send game events to the engine: {}", e)
            }

            match network_sender_c.lock().unwrap().send(send.clone()) {
                Ok(i) => i,
                Err(e) => panic!("Could not send game events to the network: {}", e)
            }
        }
    });

    /* The Network SENDS data to the game, what the other player has done,
    the Network RECEIVES data from the engine, what this player has done */
    let game_sender_c = Arc::clone(&game_sender);

    let network_thread = thread::spawn(move || {
        let mut network = networking::Network::new();

        loop {
            let mut send = Vec::new();
            let receive = match network_receiver.lock().unwrap().recv() {
                Ok(i) => i,
                Err(e) => panic!("Could not fetch events for networking loop to execute: {}", e)
            };

            network.update(&mut send, receive);

            match game_sender_c.lock().unwrap().send(send) {
                Ok(i) => i,
                Err(e) => panic!("Could not send networking events to the game: {}", e)
            }
        }
    });

    engine_thread.join().unwrap();
    game_thread.join().unwrap();
    network_thread.join().unwrap();

    return;
}