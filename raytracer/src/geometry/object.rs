use crate::{
    geometry::{intersection::Intersection, material::Material},
    lighting::ray::Ray,
};

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Intersection;

    fn get_material(&self) -> &Material;
}
