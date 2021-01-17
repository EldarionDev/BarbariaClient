pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

