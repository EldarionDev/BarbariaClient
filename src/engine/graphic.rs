use crate::{Config, maths::Vec3};
use std::collections::HashMap;

mod animation;
mod camera;
mod model;
mod projection;
mod shader;
mod texture;

#[derive(Clone)]
pub enum ObjectClass {
    Settlement,
    Army,
    Lake,
    Terrain,
    GUI,
}

#[derive(Clone)]
pub enum ObjectType {
    Dimension2,
    Dimension3,
}

#[derive(Clone)]
pub struct Graphic {
    animations: HashMap<String, animation::Animation>,
    cameras: HashMap<String, camera::Camera>,
    models: HashMap<String, model::Model>,
    projections: HashMap<String, projection::Projection>,
    shaders: HashMap<String, shader::Shader>,
    textures: HashMap<String, texture::Texture>,
}

impl Graphic {
    pub fn new(paths: &Config) -> Self {
        let mut animations: HashMap<String, animation::Animation> = HashMap::new();
        let mut cameras: HashMap<String, camera::Camera> = HashMap::new();
        let mut models: HashMap<String, model::Model> = HashMap::new();
        let mut projections: HashMap<String, projection::Projection> = HashMap::new();
        let mut shaders: HashMap<String, shader::Shader> = HashMap::new();
        let mut textures: HashMap<String, texture::Texture> = HashMap::new();

        let split_string = |s: &str, split: char| -> String {
            let s = s.split(split);
            match s.last() {
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
                split_string(&split_string(x, '/')[..], '.'),
                animation::Animation::new(x),
            );
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("cameras")
            .iter()
            .enumerate()
        {
            cameras.insert(split_string(x, '/'), camera::Camera::new(x));
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("models")
            .iter()
            .enumerate()
        {
            models.insert(split_string(x, '/'), model::Model::new(x.to_string()));
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("projections")
            .iter()
            .enumerate()
        {
            projections.insert(split_string(x, '/'), projection::Projection::new(x));
        }

        for (_, x) in paths
            .resource_manager
            .get_assets("shaders")
            .iter()
            .enumerate()
        {
            let string = split_string(x, '/');
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
            textures.insert(x.to_string(), texture::Texture::new(x.to_string()));
        }

        Graphic {
            animations,
            cameras,
            models,
            projections,
            shaders,
            textures,
        }
    }

    pub fn create_object(&self, name: &str, dimension: ObjectType, class: ObjectClass) -> Object {
        let mut shader_name: String = String::new();
        let mut projection_name: String = String::new();
        let mut camera_name: String = String::new();
        let mut model_name = name.to_string() + ".pmf";
        let texture_name = name.to_string() + ".tga";

        match dimension {
            ObjectType::Dimension2 => {
                shader_name += "2d_default";
                projection_name += "2d_default.json";
                camera_name += "2d_default.json";
                model_name = "2d_default.pmf".to_string();
            }
            ObjectType::Dimension3 => {
                shader_name += "3d_default";
                projection_name += "3d_default.json";
                camera_name += "3d_default.json";
            }
        }

        match class {
            ObjectClass::Army => shader_name += "_army",
            ObjectClass::Lake => shader_name += "_lake",
            ObjectClass::Settlement => shader_name += "_settlement",
            ObjectClass::Terrain => shader_name += "_terrain",
            ObjectClass::GUI => shader_name += "_gui",
        }

        let shader_ref: shader::Shader = match self.shaders.get(&shader_name) {
            None => panic!(
                "Specified shader: {} could not be referenced while creating instance of: {}",
                shader_name, name
            ),
            Some(i) => i.to_owned(),
        };

        /*let camera_ref: camera::Camera = match self.cameras.get(&camera_name) {
            None => panic!("Specified camera: {} could not be referenced while creating instance of: {}", camera_name, name),
            Some(i) => i.to_owned()
        }; */

        /* let projection_ref: projection::Projection = match self.projections.get(&camera_name) {
            None => panic!("Specified projection: {} could not be referenced while creating instance of: {}", projection_name, name),
            Some(i) => i.to_owned()
        }; */

        let model_ref: model::Model = match self.models.get(&model_name) {
            None => panic!(
                "Specified model: {} could not be referenced while creating instance of: {}",
                model_name, name
            ),
            Some(i) => i.to_owned(),
        };

        let texture_ref: texture::Texture = match self.textures.get(&texture_name) {
            None => panic!(
                "Specified texture: {} could not be referenced while creating instance of: {}",
                texture_name, name
            ),
            Some(i) => i.to_owned(),
        };

        Object {
            object_name: name.to_string(),
            class,
            dimension,
            shader: shader_ref,
            camera: camera::Camera::new("bef"),
            projection: projection::Projection::new("test"),
            model: model_ref,
            texture: texture_ref,
        }
    }

    pub fn draw(&self, obj: &Object) {
        obj.shader.bind();
        obj.projection.bind(&obj.shader);
        obj.camera.bind(&obj.shader);
        obj.texture.bind();
        obj.model.bind();
        obj.model.draw();
    }
}

#[derive(Clone)]

pub struct Object {
    object_name: String,
    class: ObjectClass,
    dimension: ObjectType,
    shader: shader::Shader,
    camera: camera::Camera,
    projection: projection::Projection,
    model: model::Model,
    texture: texture::Texture,
}
