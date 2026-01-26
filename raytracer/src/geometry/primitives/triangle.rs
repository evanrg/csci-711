use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

use crate::{
    geometry::{intersection::Intersection, material::Material, object::Object},
    lighting::ray::Ray,
};

pub struct Triangle {
    vertices: (Vec3, Vec3, Vec3),
    material: Material,
    model_transform: Mat4,
}

impl Triangle {
    pub fn new(vertices: (Vec3, Vec3, Vec3), material: Material) -> Self {
        Self {
            vertices,
            material,
            model_transform: Mat4::IDENTITY,
        }
    }

    pub fn translate_mut(&mut self, distance: Vec3) {
        self.model_transform.col_mut(3).x = distance.x;
        self.model_transform.col_mut(3).y = distance.y;
        self.model_transform.col_mut(3).z = distance.z;
    }

    fn verts_mut(&mut self, transform: &Mat4) {
        let v1_h = Vec4::from((self.vertices.0, 1.0));
        let v2_h = Vec4::from((self.vertices.1, 1.0));
        let v3_h = Vec4::from((self.vertices.2, 1.0));

        let v1 = transform.mul_vec4(v1_h).xyz();
        let v2 = transform.mul_vec4(v2_h).xyz();
        let v3 = transform.mul_vec4(v3_h).xyz();

        self.vertices = (v1, v2, v3);
    }
}

impl Object for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let e1 = self.vertices.1 - self.vertices.0;
        let e2 = self.vertices.2 - self.vertices.0;
        let t = ray.origin - self.vertices.0;
        let p = ray.direction.cross(e2);
        let q = t.cross(e1);

        let den = p.dot(e1);

        if den == 0.0 {
            return None;
        }

        let factor = 1.0 / (p.dot(e1));
        let vect = Vec3::new(q.dot(e2), p.dot(t), q.dot(ray.direction));

        let intersect = factor * vect;

        // omega < = 0 --> behind origin
        if intersect.x < 0.0 {
            return None;
        }

        // outside triangle
        if intersect.y < 0.0 || intersect.z < 0.0 || intersect.y + intersect.z > 1.0 {
            return None;
        }

        let intersection_point = ray.origin + intersect.x * ray.direction;
        let norm = e1.cross(e2);

        Some(Intersection::new(intersection_point, norm, self.material))
    }

    fn to_world_space_mut(&mut self) {
        self.verts_mut(&self.model_transform.clone());
    }

    fn to_view_space_mut(&mut self, view_transform: &Mat4) {
        self.verts_mut(view_transform);
    }
}
