use glam::{Mat4, Vec3};

use crate::{geometry::intersection::Intersection, world::World};

pub mod ashikhmin_shirley;
pub mod phong;
pub mod phong_blinn;

pub enum IlluminationType {
    Phong,
    PhongBlinn,
    AshikhminShirley,
}

pub trait IlluminationModel {
    fn illuminate(
        &mut self,
        world: &World,
        intersection: &Intersection,
        cam_pos: Vec3,
        view_transform: &Mat4,
        depth: u32,
    ) -> Vec3;
}
