use glam::Vec3;

use crate::world::World;

pub struct Camera {
    position: Vec3,
    look_at: Vec3,
    up: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3) -> Self {
        Self {
            position,
            look_at,
            up,
        }
    }

    pub fn render(&self, world: &World) {
        println!("RENDERING HERE!");
    }
}
