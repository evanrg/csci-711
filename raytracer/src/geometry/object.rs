use crate::{geometry::material::Material, lighting::ray::Ray};

pub trait Object {
    fn intersect(&self, ray: &Ray) -> bool;

    fn get_material(&self) -> &Material;
}
