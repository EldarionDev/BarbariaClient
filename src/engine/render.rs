mod animation;
mod camera;
mod corpus;
mod projection;
mod shader;
mod texture;

pub trait Bindable {
    fn new(path: String) -> Self;
    fn load(&mut self);
    fn bind(&self);
}

pub trait Renderable {
    fn render(&self);
}

pub trait RetShader {
    fn get_shader(&self) -> u32;
}

pub trait Pushable {
    fn push(&mut self, shader: u32);
}