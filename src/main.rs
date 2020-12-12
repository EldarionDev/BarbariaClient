mod logger;
mod game;
mod engine;

fn main() {
    info!("Entering Middle-earth...");
    game::start_client();
    let barbaria_engine = engine::Engine::new();
    loop {
        unsafe {
            /*gl::ClearColor(0.5, 0.3, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); */
        }
    }
    info!("Sailing to Valinor...");
}