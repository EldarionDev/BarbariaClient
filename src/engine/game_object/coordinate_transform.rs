pub fn to_gl_space(number: f32) -> f32{
    (number - 500.0) / 500.0
}

pub fn to_world_space(number: f32) -> f32{
    (number + 1.0) * 500.0
}