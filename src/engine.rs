use crate::info;
mod graphics;

pub fn init_engine () {
    info!("Starting the Barbaria Engine!");
    graphics::setup_graphics();
}

pub fn uninit_engine () {
    info!("Terminating the Barbaria Engine!");
}