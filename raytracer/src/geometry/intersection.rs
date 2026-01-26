use glam::Vec3;

use crate::geometry::material::Material;

#[derive(Clone, Copy)]
pub struct Intersection {
    pub intersection_point: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Intersection {
    pub fn new(intersection_point: Vec3, normal: Vec3, material: Material) -> Self {
        Self {
            intersection_point,
            normal,
            material,
        }
    }
}
