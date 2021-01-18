use crate::{
    engine::graphic::{ObjectClass, ObjectType},
    maths::Vec3,
};

use super::GameObject;

pub struct GuiElement {
    position: Vec3,
    name: String,
    dimension: ObjectType,
    class: ObjectClass,
}

impl GameObject for GuiElement {
    fn new(name: &str, position: Vec3) -> Self {
        GuiElement {
            position,
            name: name.to_string(),
            dimension: ObjectType::Dimension2,
            class: ObjectClass::GUI,
        }
    }

    fn get_position(&self) -> &crate::maths::Vec3 {
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
}
