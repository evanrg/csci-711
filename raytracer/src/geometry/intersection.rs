use glam::Vec3;

use crate::geometry::material::Material;

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub intersection_point: Vec3,
    pub normal: Vec3,
    pub material: &'a Box<dyn Material>,
}

impl<'a> Intersection<'a> {
    pub fn new(intersection_point: Vec3, normal: Vec3, material: &'a Box<dyn Material>) -> Self {
        Self {
            intersection_point,
            normal,
            material,
        }
    }
}
