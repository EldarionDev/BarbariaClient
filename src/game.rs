use crate::{info};

mod world;

pub fn start_client () {
    info!("Starting the game client.");
}

pub fn start_server () {
    info!("Starting the game server.");
}

pub fn end_client () {
    info!("Terminating the game client.");
}

pub fn end_server () {
    info!("Terminating the game server.");
}