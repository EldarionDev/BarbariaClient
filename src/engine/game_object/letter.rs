use crate::engine::graphic::{ObjectClass, ObjectType};

use super::GameObject;

pub struct Letter {
    position: glm::Vec3,
    scale: glm::Vec3,
    rotation: glm::Vec3,
    rotation_angle: f32,
    pub letter: String,
    dimension: ObjectType,
    class: ObjectClass,
}

impl GameObject for Letter {
    fn new(name: &str, position: glm::Vec3, scale: glm::Vec3, rotation: glm::Vec3, rotation_angle: f32) -> Self {
        Self {
            letter: name.to_string(),
            position,
            rotation,
            scale,
            rotation_angle,
            class: ObjectClass::Font,
            dimension: ObjectType::Dimension2
        }
    }

    fn get_position(&self) -> &glm::Vec3 {
        &self.position
    }

    fn get_scale(&self) -> &glm::Vec3 {
        &self.scale
    }

    fn get_rotation(&self) -> &glm::Vec3 {
        &self.rotation
    }

    fn get_rotation_angle(&self) -> f32 {
        self.rotation_angle
    }

    fn get_name(&self) -> &str {
        &self.letter[0..0]
    }

    fn get_type(&self) -> &crate::engine::graphic::ObjectType {
        &self.dimension
    }

    fn get_class(&self) -> &crate::engine::graphic::ObjectClass {
        &self.class
    }
}