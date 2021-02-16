use std::fs;

use crate::{game::Game};
use glm::{Vec3, Vec4};

use super::Config;
use rand::Rng;
use serde_json::Value;
use std::sync::mpsc::Receiver;

pub(crate) mod event;
mod render;
mod physics;
mod window;

pub struct Engine {
    paths: Config,
}

impl Engine {
    pub fn new(paths: Config) -> Engine {
        Engine {
            paths,
        }
    }

    pub fn render_tick(&mut self, send: &mut Vec<f32>, receive: Vec<f32>) {
        unsafe {
            gl::ClearColor(0.6, 0.3, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

impl Drop for Engine {
    fn drop(&mut self) {}
}
