use glam::{Mat4, Vec3};

use crate::{geometry::intersection::Intersection, lighting::ray::Ray};

//
// Objects are responsible for knowing if a ray intersects
// with them, transforming to certain spaces, and knowing
// various material properties
//
pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection<'_>>;

    fn to_world_space_mut(&mut self);

    fn to_view_space_mut(&mut self, view_transform: &Mat4);

    fn compile_model(&mut self);

    fn get_color(&self, view_transform: &Mat4, int: Vec3) -> Vec3;

    fn get_specular_color(&self, view_transform: &Mat4, int: Vec3) -> Vec3;

    fn get_kr(&self) -> f32;

    fn get_kt(&self) -> f32;

    fn get_refraction_index(&self) -> f32;

    fn get_max_depth(&self) -> u32;
}
