use crate::maths::Vec3;

use super::graphic::{ObjectClass, ObjectType};

mod gui_element;

pub trait GameObject {
    fn new() -> Self;
    fn get_position(&self) -> Vec3;
    fn get_name(&self) -> &str;
    fn get_type(&self) -> &ObjectType;
    fn get_class(&self) -> &ObjectClass;
}