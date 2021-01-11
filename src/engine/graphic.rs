use std::collections::HashMap;
use crate::Config;

mod animation;
mod camera;
mod model;
mod projection;
mod shader;
mod texture;

pub enum ObjectClass {
    Settlement,
    Army,
    Lake,
    Terrain,
    GUI
}

pub enum ObjectType {
    Dimension2,
    Dimension3
}

pub struct Graphic {
    animations: HashMap<String, animation::Animation>,
    cameras: HashMap<String, camera::Camera>,
    models: HashMap<String, model::Model>,
    projections: HashMap<String, projection::Projection>,
    shaders: HashMap<String, shader::Shader>,
    textures: HashMap<String, texture::Texture>
}

impl Graphic {
    pub fn new(paths: Config) -> Self{
        let mut animations: HashMap<String, animation::Animation> = HashMap::new();
        let mut cameras: HashMap<String, camera::Camera> = HashMap::new();
        let mut models: HashMap<String, model::Model> = HashMap::new();
        let mut projections: HashMap<String, projection::Projection> = HashMap::new();
        let mut shaders: HashMap<String, shader::Shader> = HashMap::new();
        let mut textures: HashMap<String, texture::Texture> = HashMap::new();

        for (i, x) in paths.resource_manager.get_assets("animations").iter().enumerate() {
            let mut splitted_string = x.split('/');
            let mut splitted_string = splitted_string.last().unwrap();
            let mut splitted_string = splitted_string.split('.');
            let splitted_string = splitted_string.next().unwrap();
            animations.insert(splitted_string.to_string(), animation::Animation::new(x));
        }

        for (i, x) in paths.resource_manager.get_assets("cameras").iter().enumerate() {
            let mut splitted_string = x.split('/');
            let mut splitted_string = splitted_string.last().unwrap();
            let mut splitted_string = splitted_string.split('.');
            let splitted_string = splitted_string.next().unwrap();
            cameras.insert(splitted_string.to_string(), camera::Camera::new(x));
        }

        for (i, x) in paths.resource_manager.get_assets("models").iter().enumerate() {
            let mut splitted_string = x.split('/');
            let mut splitted_string = splitted_string.last().unwrap();
            let mut splitted_string = splitted_string.split('.');
            let splitted_string = splitted_string.next().unwrap();
            models.insert(splitted_string.to_string(), model::Model::new(x.to_string()));
        }

        for (i, x) in paths.resource_manager.get_assets("projections").iter().enumerate() {
            let mut splitted_string = x.split('/');
            let mut splitted_string = splitted_string.last().unwrap();
            let mut splitted_string = splitted_string.split('.');
            let splitted_string = splitted_string.next().unwrap();
            projections.insert(splitted_string.to_string(), projection::Projection::new(x));
        }

        for (i, x) in paths.resource_manager.get_assets("shaders").iter().enumerate() {
            let mut splitted_string = x.split('/');
            let mut splitted_string = splitted_string.last().unwrap();
            let mut splitted_string = splitted_string.split('.');
            let splitted_string = splitted_string.next().unwrap();
            if shaders.contains_key(splitted_string) {continue};
            shaders.insert(splitted_string.to_string(), shader::Shader::new(x.to_string()));
        }

        for (i, x) in paths.resource_manager.get_assets("textures").iter().enumerate() {
            let mut splitted_string = x.split('/');
            let mut splitted_string = splitted_string.last().unwrap();
            let mut splitted_string = splitted_string.split('.');
            let splitted_string = splitted_string.next().unwrap();
            textures.insert(splitted_string.to_string(), texture::Texture::new(x.to_string()));
        }

        Graphic {
            animations,
            cameras,
            models,
            projections,
            shaders,
            textures
        }
    }

    pub fn draw(mut self, obj: Object) {
        obj.shader.bind();
        obj.projection.bind(obj.shader);
        obj.camera.bind(obj.shader);
        obj.texture.bind();
        obj.model.bind();
        obj.model.draw();
    }
}

pub struct Object<'b> {
    object_name: String,
    class: ObjectClass,
    dimension: ObjectType,
    shader: &'b shader::Shader,
    camera: &'b camera::Camera,
    projection: &'b projection::Projection,
    model: &'b model::Model,
    texture: &'b texture::Texture
}

impl<'b> Object<'b> {
    pub fn new(graphic: &'b mut Graphic, name: &str, dimension: ObjectType, class: ObjectClass) -> Self {
        let mut shader_name: String = String::new();
        let mut projection_name: String = String::new();
        let mut camera_name: String = String::new();
        let mut model_name = name.to_string() + ".pmf";
        let mut texture_name = name.to_string() + ".tga";

        match dimension {
            ObjectType::Dimension2 => {
                shader_name += "2d_default";
                projection_name += "2d_default.json";
                camera_name += "2d_default.json";
                model_name = "gui_default.pmf".to_string();
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
            ObjectClass::GUI => shader_name += "_gui"
        }

        let shader_ref: &'b shader::Shader = match graphic.shaders.get_mut(&shader_name) {
            None => panic!("Specified shader: {} could not be referenced while creating instance of: {}", shader_name, name),
            Some(i) => i
        };

        let camera_ref: &'b camera::Camera = match graphic.cameras.get_mut(&camera_name) {
            None => panic!("Specified camera: {} could not be referenced while creating instance of: {}", camera_name, name),
            Some(i) => i
        };

        let projection_ref: &'b projection::Projection = match graphic.projections.get_mut(&camera_name) {
            None => panic!("Specified projection: {} could not be referenced while creating instance of: {}", projection_name, name),
            Some(i) => i
        };

        let model_ref: &'b model::Model = match graphic.models.get_mut(&model_name) {
            None => panic!("Specified model: {} could not be referenced while creating instance of: {}", model_name, name),
            Some(i) => i
        };

        let texture_ref: &'b texture::Texture = match graphic.textures.get_mut(&texture_name) {
            None => panic!("Specified texture: {} could not be referenced while creating instance of: {}", texture_name, name),
            Some(i) => i
        };

        Object {
            object_name: name.to_string(),
            class,
            dimension,
            shader: shader_ref,
            camera: camera_ref,
            projection: projection_ref,
            model: model_ref,
            texture: texture_ref
        }
    }
}