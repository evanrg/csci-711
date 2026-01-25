use glam::Vec3;

use crate::{
    geometry::{intersection::Intersection, material::Material, object::Object},
    lighting::ray::Ray,
};

pub struct Triangle {
    vertices: (Vec3, Vec3, Vec3),
    material: Material,
}

impl Triangle {
    pub fn new(vertices: (Vec3, Vec3, Vec3), material: Material) -> Self {
        Self { vertices, material }
    }
}

impl Object for Triangle {
    fn intersect(&self, ray: &Ray) -> Intersection {
        let e1 = self.vertices.1 - self.vertices.0;
        let e2 = self.vertices.2 - self.vertices.0;
        let t = ray.origin - self.vertices.0;
        let p = ray.direction.cross(e2);
        let q = t.cross(e1);

        let den = p.dot(e1);

        if den == 0.0 {
            return Intersection::new(None, None);
        }

        let factor = 1.0 / (p.dot(e1));
        let vect = Vec3::new(q.dot(e2), p.dot(t), q.dot(ray.direction));

        let intersect = factor * vect;

        // omega < = 0 --> behind origin
        if intersect.x < 0.0 {
            return Intersection::new(None, None);
        }

        // outside triangle
        if intersect.y < 0.0 || intersect.z < 0.0 || intersect.y + intersect.z > 1.0 {
            return Intersection::new(None, None);
        }

        let intersection_point = ray.origin + intersect.x * ray.direction;
        let norm = e1.cross(e2);

        Intersection::new(Some(intersection_point), Some(norm))
    }

    fn get_material(&self) -> &Material {
        return &self.material;
    }
}
