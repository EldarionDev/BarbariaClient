use crate::{
    engine::graphic::{ObjectClass, ObjectType}
};

use glm::{Vec3, Vec4};

use super::GameObject;

pub struct GuiElement {
    position: Vec3,
    scale: Vec3,
    rotation: Vec3,
    rotation_angle: f32,
    name: String,
    dimension: ObjectType,
    class: ObjectClass,
}

impl GameObject for GuiElement {
    fn new(name: &str, position: Vec3, scale: Vec3, rotation: Vec3, rotation_angle: f32) -> Self {
        GuiElement {
            position,
            scale,
            rotation,
            rotation_angle,
            name: name.to_string(),
            dimension: ObjectType::Dimension2,
            class: ObjectClass::GUI,
        }
    }

    fn get_position(&self) -> &Vec3 {
        return &self.position;
    }

    fn get_name(&self) -> &str {
        return &self.name[..];
    }

    fn get_type(&self) -> &crate::engine::graphic::ObjectType {
        return &self.dimension;
    }

    fn get_class(&self) -> &crate::engine::graphic::ObjectClass {
        return &self.class;
    }

    fn get_scale(&self) -> &Vec3 {
        return &self.scale;
    }

    fn get_rotation(&self) -> &Vec3 {
        return &self.rotation;
    }

    fn get_rotation_angle(&self) -> f32 {
        return self.rotation_angle;
    }
}
