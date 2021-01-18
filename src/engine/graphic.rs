use crate::{game::Game, maths::Vec3, Config};
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
    Building,
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
                projection::Projection::new(x),
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

        Graphic {
            animations,
            cameras,
            models,
            projections,
            shaders,
            textures,
        }
    }

    pub fn create_object(&self, name: &str, dimension: &ObjectType, class: &ObjectClass) -> Object {
        let mut shader_name: String = String::new();
        let mut projection_name: String = String::new();
        let mut camera_name: String = String::new();
        let mut model_name = name.to_string();
        let texture_name = name.to_string();

        match dimension {
            ObjectType::Dimension2 => {
                shader_name += "2d_default";
                projection_name += "2d_default";
                camera_name += "2d_default";
                model_name = "2d_default".to_string();
            }
            ObjectType::Dimension3 => {
                shader_name += "3d_default";
                projection_name += "3d_default";
                camera_name += "3d_default";
            }
        }

        match class {
            ObjectClass::Army => shader_name += "_army",
            ObjectClass::Lake => shader_name += "_lake",
            ObjectClass::Settlement => shader_name += "_settlement",
            ObjectClass::Terrain => shader_name += "_terrain",
            ObjectClass::GUI => shader_name += "_gui",
            ObjectClass::Building => shader_name += "_building",
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
            class: class.clone(),
            dimension: dimension.clone(),
            shader: shader_ref,
            camera: camera::Camera::new("bef"),
            projection: projection::Projection::new("test"),
            model: model_ref,
            texture: texture_ref,
            game_objects: None,
        }
    }

    pub fn draw(&self, obj: &Object) {
        let game_objects = match &obj.game_objects {
            Some(i) => i,
            None => return,
        };

        obj.shader.bind();
        obj.projection.bind(&obj.shader);
        obj.camera.bind(&obj.shader);
        obj.texture.bind();

        for i in game_objects.iter() {
            obj.model.bind(&i.position);
            obj.model.draw();
        }
    }
}

pub struct Object {
    pub object_name: String,
    class: ObjectClass,
    dimension: ObjectType,
    shader: shader::Shader,
    camera: camera::Camera,
    projection: projection::Projection,
    model: model::Model,
    texture: texture::Texture,
    game_objects: Option<Vec<ObjectInstance>>,
}

impl Object {
    pub fn add(&mut self, position: &Vec3) {
        let position = (*position).clone();
        match &mut self.game_objects {
            Some(i) => {
                i.push(ObjectInstance { position });
            }
            None => {
                self.game_objects = Some(vec![ObjectInstance { position }]);
            }
        }
    }

    pub fn remove(&mut self, position: &Vec3) {
        let position = (*position).clone();
        let vec = match &mut self.game_objects {
            Some(i) => i,
            None => panic!("No object instance exists to remove!"),
        };

        let mut index = 0;
        let mut found = false;
        for g in vec.iter() {
            if g.position == position {
                found = true;
                break;
            } else {
                index += 1;
            }
        }

        if found {
            vec.remove(index);
        } else {
            panic!("Could not find object instance to remove!");
        }
    }
}

pub struct ObjectInstance {
    position: Vec3,
}
