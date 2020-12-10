mod logger;
mod game;
mod engine;

fn main() {
    info!("Entering Middle-earth...");
    game::start_client();
    engine::init_engine();
    loop {
        unsafe {
            /*gl::ClearColor(0.5, 0.3, 0.4, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT); */
        }
    }
    game::end_client();
    engine::uninit_engine();
    info!("Sailing to Valinor...");
}