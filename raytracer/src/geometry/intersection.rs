use glam::Vec3;

use crate::geometry::object::Object;

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub intersection_point: Vec3,
    pub normal: Vec3,
    pub object: &'a dyn Object,
}

impl<'a> Intersection<'a> {
    pub fn new(intersection_point: Vec3, normal: Vec3, object: &'a dyn Object) -> Self {
        Self {
            intersection_point,
            normal,
            object,
        }
    }
}
