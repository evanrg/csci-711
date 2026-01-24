use glam::Vec3;

use crate::{
    geometry::{material::Material, object::Object},
    lighting::ray::Ray,
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        return false;
    }

    fn get_material(&self) -> &Material {
        return &self.material;
    }
}
