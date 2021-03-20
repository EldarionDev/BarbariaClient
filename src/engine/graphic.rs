use crate::{game::Game, Config};
use std::{collections::HashMap, ffi::CString, fs};
use glm::{Vec3, Vec4, Mat4};
use serde_json::Value;


extern crate freetype;

mod animation;
mod camera;
mod model;
mod projection;
mod shader;
mod texture;
mod font;



pub struct Graphic {
    render_queue: Vec<Box<dyn Render>>,
    render_units: Vec<RenderUnit>,
    render_texts: Vec<RenderText>,
    fonts: HashMap<String, font::Font>,
    animations: HashMap<String, animation::Animation>,
    cameras: HashMap<String, camera::Camera>,
    models: HashMap<String, model::Model>,
    projections: HashMap<String, projection::Projection>,
    shaders: HashMap<String, shader::Shader>,
    textures: HashMap<String, texture::Texture>,
}

impl Graphic {
    pub fn new(paths: &Config, screen_size: (f32, f32)) -> Self {
        let mut animations: HashMap<String, animation::Animation> = HashMap::new();
        let mut cameras: HashMap<String, camera::Camera> = HashMap::new();
        let mut models: HashMap<String, model::Model> = HashMap::new();
        let mut projections: HashMap<String, projection::Projection> = HashMap::new();
        let mut shaders: HashMap<String, shader::Shader> = HashMap::new();
        let mut textures: HashMap<String, texture::Texture> = HashMap::new();
        let mut fonts: HashMap<String, font::Font> = HashMap::new();
        let mut render_units: Vec<RenderUnit> = Vec::new();

        let split_string_last = |s: &str, split: char| -> String {
            let s = s.split(split);
            match s.last() {
                Some(s) => s,
                None => panic!("Could not split string!"),
            }
            .to_string()
        };

        let split_string_first = |s: &str, split: char| -> String {
            let mut s = s.split(split);
            match s.next() {
                Some(s) => s,
                None => panic!("Could not split string!"),
            }
            .to_string()
        };

        for (_, x) in paths
            .resource_manager
            .get_assets("animations")
            .iter()
            .enumerate()
        {
            animations.insert(
                split_string_first(&split_string_last(x, '/')[..], '.'),
                animation::Animation::new(x),
            );
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("cameras")
            .iter()
            .enumerate()
        {
            cameras.insert(
                split_string_first(&split_string_last(x, '/')[..], '.'),
                camera::Camera::new(x),
            );
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("models")
            .iter()
            .enumerate()
        {
            models.insert(
                split_string_first(&split_string_last(x, '/')[..], '.'),
                model::Model::new(x.to_string()),
            );
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("projections")
            .iter()
            .enumerate()
        {
            projections.insert(
                split_string_first(&split_string_last(x, '/')[..], '.'),
                projection::Projection::new(x, screen_size),
            );
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("shaders")
            .iter()
            .enumerate()
        {
            let string = split_string_last(x, '/');
            let string = match string.split('.').next() {
                Some(s) => s,
                None => panic!("Could not split shader string!"),
            };
            if shaders.contains_key(&string[..]) {
                continue;
            };
            let shader_path = match x.split('.').next() {
                Some(s) => s,
                None => panic!("Error spliting shader string!"),
            };
            shaders.insert(
                string.to_string(),
                shader::Shader::new(shader_path.to_string()),
            );
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("textures")
            .iter()
            .enumerate()
        {
            textures.insert(
                split_string_first(&split_string_last(x, '/')[..], '.'),
                texture::Texture::new(x.to_string()),
            );
        }

        let mut font_library = match freetype::Library::init() {
            Ok(i) => i,
            Err(e) => panic!("Could not initialize the Font library!")
        };

        for (_, x) in paths
            .resource_manager
            .get_assets("fonts")
            .iter()
            .enumerate()
        {
            let mut val = font::Font::new(&mut font_library, x.to_string());
            fonts.insert(
                split_string_first(&split_string_last(x, '/')[..], '.'),
                val
            );
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("render_units")
            .iter()
            .enumerate() 
        {
            let config_file_content: String = fs::read_to_string(x).unwrap().parse().unwrap();
            let config_file_content: &str = &config_file_content[..];
            let json_content: Value = serde_json::from_str(config_file_content).unwrap();
            render_units.push(
            RenderUnit{
                name: split_string_first(&split_string_last(x, '/')[..], '.'),
                animation: animations.get(json_content["animation"].as_str().expect("No animation specified.")).expect("Animation has not been loaded.").clone(),
                camera: cameras.get(json_content["camera"].as_str().expect("No camera specified.")).expect("Camera has not been loaded.").clone(),
                model: models.get(json_content["model"].as_str().expect("No model specified.")).expect("Model has not been loaded.").clone(),
                projection: projections.get(json_content["projection"].as_str().expect("No projection specified.")).expect("Projection has not been loaded.").clone(),
                shader: shaders.get(json_content["shader"].as_str().expect("No shader specified.")).expect("Shader has not been loaded.").clone(),
                texture: textures.get(json_content["texture"].as_str().expect("No texture specified.")).expect("Texture has not been loaded.").clone(),
                render_objects: Vec::new()
            });
        }

        Graphic {
            render_queue: Vec::new(),
            render_texts: Vec::new(),
            render_units,
            fonts,
            animations,
            cameras,
            models,
            projections,
            shaders,
            textures
        }
    }

    pub fn add_object(&mut self, render_unit: String, render_object: RenderObject) -> &str {
        let unit = &mut self.render_units;
        let unit = match unit.into_iter().filter(|i| i.name == render_unit).last() {
            Some(i) => i,
            None => panic!("Attempted to use render unit without having it loaded.")
        };
        
        if unit.render_objects.is_empty() {
            unit.model.load();
            unit.shader.load();
            unit.texture.load();
        }

        unit.render_objects.push(render_object);
        self.render_queue.push(Box::new(unit.clone()));

        &unit.render_objects.last().unwrap().name
    }

    pub fn remove_object(name: &str) {

    }

    pub fn add_text(&mut self, font: String, text: String, color: (f32, f32, f32), position: (f32, f32), scale: f32) {
        let mut shader = self.shaders.get("2d_text").expect("Could not load shader").clone();
        shader.load();
        let mut font = self.fonts.get(&font).expect("Could not load font.").clone();
        font.load();
        let projection = self.projections.get("2d_text").expect("COuld not load projection.").clone();
        let render_text = RenderText{
            font,
            shader,
            projection,
            text,
            color,
            position,
            scale
        };
        
        self.render_texts.push(render_text.clone());
        self.render_queue.push(Box::new(render_text.clone()));
    }

    pub fn remove_text() {

    }

    pub fn render(&mut self) {
        for i in self.render_queue.iter() {
            i.render();
        }
    }
}

#[derive(Clone)]
pub struct RenderUnit {
    name: String,
    animation: animation::Animation,
    camera: camera::Camera,
    model: model::Model,
    projection: projection::Projection,
    shader: shader::Shader,
    texture: texture::Texture,
    render_objects: Vec<RenderObject>
}

impl RenderUnit {

}

impl Render for RenderUnit {
    fn render(&self) {
        if self.render_objects.is_empty() {return;}
        self.shader.bind();
        self.projection.bind(&self.shader);
        self.camera.bind(&self.shader);
        self.texture.bind();

        for j in self.render_objects.iter() {
                j.calc_bind_model_matrix(&self.shader);
                self.model.bind();
                self.model.draw();
            }
    }
}

#[derive(Clone)]
pub struct RenderText {
    font: font::Font,
    shader: shader::Shader,
    projection: projection::Projection,
    text: String,
    color: (f32, f32, f32),
    position: (f32, f32),
    scale: (f32),
}

impl RenderText{

}

impl Render for RenderText {
    fn render(&self) {
        self.shader.bind();
        self.projection.bind(&self.shader);
        self.font.render_text(glm::vec3(self.color.0, self.color.1, self.color.2), self.text.clone(), &self.shader, self.position, self.scale)
    }
}

#[derive(Clone)]
pub struct RenderObject {
    name: String,
    position: Vec3,
    rotation: Vec3,
    rotation_angle: f32,
    scale: Vec3,
}

impl RenderObject {
    pub fn new(name: String, position: Vec3, rotation: Vec3, rotation_angle: f32, scale: Vec3) -> Self{
        RenderObject {
            name,
            position,
            rotation,
            rotation_angle,
            scale
        }
    }

    fn calc_bind_model_matrix(&self, shader: &shader::Shader) {
        let model_matrix = glm::mat4(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        );
        let model_matrix = glm::translate(&model_matrix, &self.position);
        let model_matrix = glm::scale(&model_matrix, &self.scale);
        //let model_matrix = glm::ext::rotate(&model_matrix, glm::builtin::radians(obj.rotation_angle), obj.rotation);
        
        let string = CString::new("model_matrix").unwrap();
        unsafe {
            let shader_location = gl::GetUniformLocation(shader.get_id(), string.as_bytes().as_ptr() as *const i8);
            if shader_location == -1 {
                panic!("Shader location for projection matrix is not existing.");
            }
            gl::UniformMatrix4fv(shader_location, 1, 0, model_matrix.as_ptr());
        }
    }
}

trait Render {
    fn render(&self);
}