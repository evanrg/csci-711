use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Vec3,
    pub specular_color: Vec3,
}

impl Material {
    pub fn new(color: Vec3, specular_color: Vec3) -> Self {
        Self {
            color,
            specular_color,
        }
    }
}
