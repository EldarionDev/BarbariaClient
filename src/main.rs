mod logger;
mod game;
mod engine;

fn main() {
    info!("Entering Middle-earth...");
    game::start_client();
    let mut barbaria_engine = engine::Engine::new().unwrap();
    loop {
        barbaria_engine.do_engine_tick();
        unsafe{
            gl::ClearColor(0.5, 0.3, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); 
        }
    }
    game::end_client();
    info!("Sailing to Valinor...");
}