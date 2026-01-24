use glam::Vec3;

pub struct Material {
    color: Vec3,
}

impl Material {
    pub fn get_color(&self) -> Vec3 {
        self.color
    }
}
