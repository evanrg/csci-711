// Even though this is listed as a primitive,
// it's just two triangles but going to keep
// it here anyways lol
use std::f32::consts::PI;

use glam::{Mat4, Quat, Vec3};

use crate::{
    geometry::{
        intersection::Intersection, material::Material, object::Object,
        primitives::triangle::Triangle,
    },
    lighting::ray::Ray,
};

pub struct Cube {
    triangles: Vec<Triangle>,
    material: Material,
    model_transform: Mat4,
    scaling_matrix: Mat4,
    translation_matrix: Mat4,
    rotation_matrix: Mat4,
}

impl Cube {
    pub fn new(material: Material) -> Self {
        let mut triangles = Vec::new();

        // front face
        let f1_v1 = Vec3::new(-0.5, -0.5, 0.5);
        let f1_v2 = Vec3::new(0.5, -0.5, 0.5);
        let f1_v3 = Vec3::new(0.5, 0.5, 0.5);
        triangles.push(Triangle::new((f1_v1, f1_v2, f1_v3), material));

        let f2_v1 = Vec3::new(-0.5, -0.5, 0.5);
        let f2_v2 = Vec3::new(0.5, 0.5, 0.5);
        let f2_v3 = Vec3::new(-0.5, 0.5, 0.5);
        triangles.push(Triangle::new((f2_v1, f2_v2, f2_v3), material));

        // back face
        let b1_v1 = Vec3::new(0.5, -0.5, -0.5);
        let b1_v2 = Vec3::new(-0.5, -0.5, -0.5);
        let b1_v3 = Vec3::new(-0.5, 0.5, -0.5);
        triangles.push(Triangle::new((b1_v1, b1_v2, b1_v3), material));

        let b2_v1 = Vec3::new(0.5, -0.5, -0.5);
        let b2_v2 = Vec3::new(-0.5, 0.5, -0.5);
        let b2_v3 = Vec3::new(0.5, 0.5, -0.5);
        triangles.push(Triangle::new((b2_v1, b2_v2, b2_v3), material));

        // left face
        let l1_v1 = Vec3::new(-0.5, -0.5, -0.5);
        let l1_v2 = Vec3::new(-0.5, -0.5, 0.5);
        let l1_v3 = Vec3::new(-0.5, 0.5, 0.5);
        triangles.push(Triangle::new((l1_v1, l1_v2, l1_v3), material));

        let l2_v1 = Vec3::new(-0.5, -0.5, -0.5);
        let l2_v2 = Vec3::new(-0.5, 0.5, 0.5);
        let l2_v3 = Vec3::new(-0.5, 0.5, -0.5);
        triangles.push(Triangle::new((l2_v1, l2_v2, l2_v3), material));

        // right face
        let r1_v1 = Vec3::new(0.5, -0.5, 0.5);
        let r1_v2 = Vec3::new(0.5, -0.5, -0.5);
        let r1_v3 = Vec3::new(0.5, 0.5, -0.5);
        triangles.push(Triangle::new((r1_v1, r1_v2, r1_v3), material));

        let r2_v1 = Vec3::new(0.5, -0.5, 0.5);
        let r2_v2 = Vec3::new(0.5, 0.5, -0.5);
        let r2_v3 = Vec3::new(0.5, 0.5, 0.5);
        triangles.push(Triangle::new((r2_v1, r2_v2, r2_v3), material));

        // top face
        let t1_v1 = Vec3::new(-0.5, 0.5, 0.5);
        let t1_v2 = Vec3::new(0.5, 0.5, 0.5);
        let t1_v3 = Vec3::new(0.5, 0.5, -0.5);
        triangles.push(Triangle::new((t1_v1, t1_v2, t1_v3), material));

        let t2_v1 = Vec3::new(-0.5, 0.5, 0.5);
        let t2_v2 = Vec3::new(0.5, 0.5, -0.5);
        let t2_v3 = Vec3::new(-0.5, 0.5, -0.5);
        triangles.push(Triangle::new((t2_v1, t2_v2, t2_v3), material));

        // bottom face
        let bt1_v1 = Vec3::new(-0.5, -0.5, -0.5);
        let bt1_v2 = Vec3::new(0.5, -0.5, -0.5);
        let bt1_v3 = Vec3::new(0.5, -0.5, 0.5);
        triangles.push(Triangle::new((bt1_v1, bt1_v2, bt1_v3), material));

        let bt2_v1 = Vec3::new(-0.5, -0.5, -0.5);
        let bt2_v2 = Vec3::new(0.5, -0.5, 0.5);
        let bt2_v3 = Vec3::new(-0.5, -0.5, 0.5);
        triangles.push(Triangle::new((bt2_v1, bt2_v2, bt2_v3), material));

        Self {
            triangles,
            material,
            model_transform: Mat4::IDENTITY,
            scaling_matrix: Mat4::IDENTITY,
            translation_matrix: Mat4::IDENTITY,
            rotation_matrix: Mat4::IDENTITY,
        }
    }

    pub fn translate_mut(&mut self, distance: Vec3) {
        self.translation_matrix.col_mut(3).x = distance.x;
        self.translation_matrix.col_mut(3).y = distance.y;
        self.translation_matrix.col_mut(3).z = distance.z;
    }

    pub fn scale_mut(&mut self, scalars: Vec3) {
        self.scaling_matrix.col_mut(0).x = scalars.x;
        self.scaling_matrix.col_mut(1).y = scalars.y;
        self.scaling_matrix.col_mut(2).z = scalars.z;
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

    fn triangles_mut(&mut self, transform: &Mat4) {
        for triangle in self.triangles.iter_mut() {
            triangle.verts_mut(transform);
        }
    }
}

impl Object for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut intersections: Vec<Intersection> = Vec::new();

        for triangle in &self.triangles {
            if let Some(int) = triangle.intersect(ray) {
                intersections.push(int);
            }
        }

        if intersections.is_empty() {
            return None;
        }

        if intersections.len() == 1 {
            return Some(intersections[0]);
        }

        let mut min_intersect = intersections[0];

        for int in intersections {
            let min_dist = min_intersect.intersection_point.distance(ray.origin);
            let curr_dist = int.intersection_point.distance(ray.origin);

            if curr_dist < min_dist {
                min_intersect = int;
            }
        }

        Some(min_intersect)
    }

    fn to_world_space_mut(&mut self) {
        self.triangles_mut(&self.model_transform.clone());
    }

    fn to_view_space_mut(&mut self, view_transform: &Mat4) {
        self.triangles_mut(view_transform);
    }

    fn compile_model(&mut self) {
        self.model_transform = self
            .translation_matrix
            .mul_mat4(&self.rotation_matrix)
            .mul_mat4(&self.scaling_matrix)
            .mul_mat4(&self.model_transform);
    }
}
