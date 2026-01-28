use std::f32::consts::PI;

use glam::{Mat4, Quat, Vec3, Vec4, Vec4Swizzles};

use crate::{
    geometry::{intersection::Intersection, material::Material, object::Object},
    lighting::ray::Ray,
};

pub struct Triangle {
    vertices: (Vec3, Vec3, Vec3),
    material: Material,
    model_transform: Mat4,
    scaling_matrix: Mat4,
    translation_matrix: Mat4,
    rotation_matrix: Mat4,
}

impl Triangle {
    pub fn new(vertices: (Vec3, Vec3, Vec3), material: Material) -> Self {
        Self {
            vertices,
            material,
            model_transform: Mat4::IDENTITY,
            scaling_matrix: Mat4::IDENTITY,
            translation_matrix: Mat4::IDENTITY,
            rotation_matrix: Mat4::IDENTITY,
        }
    }

    pub fn scale_mut(&mut self, scalars: Vec3) {
        self.scaling_matrix.col_mut(0).x = scalars.x;
        self.scaling_matrix.col_mut(1).y = scalars.y;
        self.scaling_matrix.col_mut(2).z = scalars.z;
    }

    pub fn translate_mut(&mut self, distance: Vec3) {
        self.translation_matrix.col_mut(3).x = distance.x;
        self.translation_matrix.col_mut(3).y = distance.y;
        self.translation_matrix.col_mut(3).z = distance.z;
    }

    pub fn rotate_x_mut(&mut self, degrees: f32) {
        let radians = degrees * PI / 180.0;

        let r_x = Mat4::from_rotation_x(radians);

        let q_x = Quat::from_mat4(&r_x);
        let q_r = Quat::from_mat4(&self.rotation_matrix);

        let q = q_x.mul_quat(q_r);

        self.rotation_matrix = Mat4::from_quat(q);
    }

    pub fn rotate_y_mut(&mut self, degrees: f32) {
        let radians = degrees * PI / 180.0;

        let r_y = Mat4::from_rotation_y(radians);

        let q_y = Quat::from_mat4(&r_y);
        let q_r = Quat::from_mat4(&self.rotation_matrix);

        let q = q_y.mul_quat(q_r);

        self.rotation_matrix = Mat4::from_quat(q);
    }

    pub fn rotate_z_mut(&mut self, degrees: f32) {
        let radians = degrees * PI / 180.0;

        let r_z = Mat4::from_rotation_z(radians);

        let q_z = Quat::from_mat4(&r_z);
        let q_r = Quat::from_mat4(&self.rotation_matrix);

        let q = q_z.mul_quat(q_r);

        self.rotation_matrix = Mat4::from_quat(q);
    }

    pub fn verts_mut(&mut self, transform: &Mat4) {
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

    fn compile_model(&mut self) {
        self.model_transform = self
            .translation_matrix
            .mul_mat4(&self.rotation_matrix)
            .mul_mat4(&self.scaling_matrix)
            .mul_mat4(&self.model_transform);
    }
}
