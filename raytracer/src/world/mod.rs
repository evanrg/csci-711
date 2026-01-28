pub mod camera;

use glam::{Mat4, Vec3};

use crate::{
    geometry::{intersection::Intersection, object::Object},
    lighting::ray::Ray,
};

pub struct World {
    objects: Vec<Box<dyn Object + 'static>>,
    pub background_color: Vec3,
}

impl World {
    pub fn new(background_color: Vec3) -> Self {
        Self {
            objects: vec![],
            background_color,
        }
    }

    pub fn add<T: Object + 'static>(&mut self, obj: T) {
        let boxed_obj = Box::new(obj);
        self.objects.push(boxed_obj);
    }

    pub fn intersection_from_ray(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersects: Vec<Intersection> = vec![];

        for obj in &self.objects {
            if let Some(int) = obj.as_ref().intersect(ray) {
                intersects.push(int);
            }
        }

        if intersects.is_empty() {
            return None;
        }

        if intersects.len() == 1 {
            return Some(intersects[0]);
        }

        let mut min_intersect = intersects[0];

        for int in intersects {
            let min_dist = min_intersect.intersection_point.distance(ray.origin);
            let curr_dist = int.intersection_point.distance(ray.origin);

            if curr_dist < min_dist {
                min_intersect = int;
            }
        }

        Some(min_intersect)
    }

    pub fn objects_to_world_space(&mut self) {
        for obj_idx in 0..self.objects.len() {
            let obj = self.objects.get_mut(obj_idx).unwrap();
            obj.as_mut().to_world_space_mut();
        }
    }

    pub fn objects_to_view_space(&mut self, view_transform: &Mat4) {
        for obj_idx in 0..self.objects.len() {
            let obj = self.objects.get_mut(obj_idx).unwrap();
            obj.as_mut().to_view_space_mut(view_transform);
        }
    }

    pub fn compile_object_models(&mut self) {
        for obj_idx in 0..self.objects.len() {
            let obj = self.objects.get_mut(obj_idx).unwrap();
            obj.as_mut().compile_model();
        }
    }
}
