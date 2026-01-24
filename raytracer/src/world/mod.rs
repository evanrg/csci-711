pub mod camera;

use crate::{geometry::object::Object, lighting::ray::Ray};

pub struct World {
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn add<T: Object + 'static>(&mut self, obj: T) {
        let boxed_obj = Box::new(obj);
        self.objects.push(boxed_obj);
    }

    pub fn transform<T: Object>(&self, obj: T) {
        unimplemented!();
    }

    pub fn transform_all_objects(&mut self) {
        unimplemented!();
    }

    pub fn spawn_ray(&mut self, ray: Ray) {
        unimplemented!();
    }
}
