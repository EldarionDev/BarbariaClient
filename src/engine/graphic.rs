use std::collections::HashMap;

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
    shaders: HashMap<String, shader::Shader>
}

impl Graphic {
    pub fn new() -> Self{
        Graphic {
            animations: HashMap::new(),
            cameras: HashMap::new(),
            models: HashMap::new(),
            projections: HashMap::new(),
            shaders: HashMap::new()
        }
    }

    pub fn draw(mut self, obj: Object) {
        obj.shader.bind();
        obj.projection.bind(obj.shader);
        obj.camera.bind(obj.shader);
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
    model: &'b model::Model
}

impl<'b> Object<'b> {
    pub fn new(graphic: &'b mut Graphic, name: &str, dimension: ObjectType, class: ObjectClass) -> Self {
        let mut shader_name: String = String::new();
        let mut projection_name: String = String::new();
        let mut camera_name: String = String::new();

        match dimension {
            ObjectType::Dimension2 => {
                shader_name += "2d_default";
                projection_name += "2d_default.json";
                camera_name += "2d_default.json"
            }
            ObjectType::Dimension3 => {
                shader_name += "3d_default";
                projection_name += "3d_default.json";
                camera_name += "3d_default.json"
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

        let model_ref: &'b model::Model = match graphic.models.get_mut(name) {
            None => panic!("Specified model: {} could not be referenced while creating instance of: {}", name, name),
            Some(i) => i
        };

        Object {
            object_name: name.to_string(),
            class,
            dimension,
            shader: shader_ref,
            camera: camera_ref,
            projection: projection_ref,
            model: model_ref
        }
    }
}