use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    color: Vec3,
}

impl Material {
    pub fn new(color: Vec3) -> Self {
        Self { color }
    }

    pub fn get_color(&self) -> Vec3 {
        self.color
    }
}
