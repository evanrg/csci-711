use glam::Mat4;

use crate::{geometry::intersection::Intersection, lighting::ray::Ray};

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;

    fn to_world_space_mut(&mut self);

    fn to_view_space_mut(&mut self, view_transform: &Mat4);

    fn compile_model(&mut self);
}
