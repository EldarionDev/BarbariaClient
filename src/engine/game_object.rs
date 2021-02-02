use glm::{Vec3, Vec4};

use super::graphic::{ObjectClass, ObjectType};

pub(crate) mod gui_element;

pub trait GameObject {
    fn new(name: &str, position: Vec3, scale: Vec3, rotation: Vec3, rotation_angle: f32) -> Self;
    fn get_position(&self) -> &Vec3;
    fn get_scale(&self) -> &Vec3;
    fn get_rotation(&self) -> &Vec3;
    fn get_rotation_angle(&self) -> f32;
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &ObjectType;
    fn get_class(&self) -> &ObjectClass;
}
