use glam::Vec3;

use crate::{geometry::intersection::Intersection, world::World};

pub mod ashikhmin_shirley;
pub mod phong;

pub enum IlluminationType {
    AshikhminShirley,
    Phong,
}

pub trait IlluminationModel {
    fn illuminate(&mut self, world: &World, intersection: &Intersection, cam_pos: Vec3) -> Vec3;
}
