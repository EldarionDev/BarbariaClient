use super::Config;
use rand::Rng;

mod graphic;
mod physics;
mod events;
mod window;

pub struct Engine<'a> {
    paths: &'a Config,
    game_window: window::Window
}

impl<'a> Engine<'a> {
    pub fn new(paths: &'a Config) -> Engine {
        Engine {
            paths,
            game_window: window::Window::new()
        }
    }

    pub fn open_title_screen(&self) {
        let title_screen_files = self.paths.resource_manager.get_assets("title_screens");
        let title_screen_count = title_screen_files.len();
        let random_title_screen = rand::thread_rng().gen_range(0..(title_screen_count-1));
        let random_title_screen = &title_screen_files[random_title_screen];
    }

    pub fn render_tick(&mut self) {
        self.game_window.update();
    }
}

impl<'a> Drop for Engine<'a> {
    fn drop(&mut self) {
        
    }
}