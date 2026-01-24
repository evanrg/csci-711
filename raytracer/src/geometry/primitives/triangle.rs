use glam::Vec3;

use crate::{
    geometry::{material::Material, object::Object},
    lighting::ray::Ray,
};

pub struct Triangle {
    points: (Vec3, Vec3, Vec3),
    material: Material,
}

impl Object for Triangle {
    fn intersect(&self, ray: &Ray) -> bool {
        return false;
    }

    fn get_material(&self) -> &Material {
        return &self.material;
    }
}
