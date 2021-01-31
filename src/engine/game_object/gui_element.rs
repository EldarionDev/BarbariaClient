use crate::{
    engine::graphic::{ObjectClass, ObjectType}
};

use glm::{Vec3, Vec4};

use super::GameObject;

pub struct GuiElement {
    position: Vec3,
    scale: Vec3,
    rotation: Vec4,
    name: String,
    dimension: ObjectType,
    class: ObjectClass,
}

impl GameObject for GuiElement {
    fn new(name: &str, position: Vec3, scale: Vec3, rotation: Vec4) -> Self {
        GuiElement {
            position,
            scale,
            rotation,
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

    fn get_rotation(&self) -> &Vec4 {
        return &self.rotation;
    }
}
