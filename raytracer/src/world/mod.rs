pub mod camera;
pub mod tone_mapping;

use glam::{Mat4, Vec3};

use crate::{
    geometry::{intersection::Intersection, object::Object},
    lighting::{light_source::LightSource, ray::Ray},
};

//
// World keeps track of the objects and light sources
// Has a background (sky) color in case no intersections
// are found
//
pub struct World {
    objects: Vec<Box<dyn Object + 'static>>,
    pub lights: Vec<LightSource>,
    pub background_radiance: Vec3,
}

impl World {
    pub fn new(background_radiance: Vec3) -> Self {
        Self {
            objects: vec![],
            lights: vec![],
            background_radiance,
        }
    }

    //
    // Add an object into the world to render
    //
    pub fn add<T: Object + 'static>(&mut self, obj: T) {
        let boxed_obj = Box::new(obj);
        self.objects.push(boxed_obj);
    }

    //
    // Add a light source into the world
    //
    pub fn add_light(&mut self, light: LightSource) {
        self.lights.push(light);
    }

    //
    // Determine the closest intersection of objects
    // given a ray
    //
    pub fn intersection_from_ray(&self, ray: &Ray) -> Option<Intersection<'_>> {
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

    //
    // Given an intersection, can it see the light source?
    //
    pub fn can_see_light(&self, intersection: &Intersection, light_pos: Vec3) -> bool {
        let offset_origin = intersection.intersection_point + intersection.normal * 0.001;
        let ray = Ray::new(
            offset_origin,
            (light_pos - intersection.intersection_point).normalize(),
        );

        if let Some(_) = self.intersection_from_ray(&ray) {
            return false;
        }

        true
    }

    //
    // Convert all objects to world space
    //
    pub fn objects_to_world_space(&mut self) {
        for obj_idx in 0..self.objects.len() {
            let obj = self.objects.get_mut(obj_idx).unwrap();
            obj.as_mut().to_world_space_mut();
        }
    }

    //
    // Convert all objects to view space
    //
    pub fn objects_to_view_space(&mut self, view_transform: &Mat4) {
        for obj_idx in 0..self.objects.len() {
            let obj = self.objects.get_mut(obj_idx).unwrap();
            obj.as_mut().to_view_space_mut(view_transform);
        }
    }

    //
    // Make sure all the objects
    // are properly translated, scaled,
    // etc.
    //
    pub fn compile_object_models(&mut self) {
        for obj_idx in 0..self.objects.len() {
            let obj = self.objects.get_mut(obj_idx).unwrap();
            obj.as_mut().compile_model();
        }
    }

    //
    // Transform the lights into view space
    //
    pub fn lights_to_view_space(&mut self, view_transform: &Mat4) {
        for light in self.lights.iter_mut() {
            light.to_view_space_mut(view_transform);
        }
    }
}
