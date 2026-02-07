use glam::Vec3;

use crate::{geometry::intersection::Intersection, world::World};

pub mod phong;
pub mod phong_blinn;

pub enum IlluminationType {
    Phong,
    PhongBlinn,
}

pub trait IlluminationModel {
    fn illuminate(&mut self, world: &World, intersection: &Intersection, cam_pos: Vec3) -> Vec3;
}
