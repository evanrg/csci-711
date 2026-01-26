use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

use crate::{
    geometry::{intersection::Intersection, material::Material, object::Object},
    lighting::ray::Ray,
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
    model_transform: Mat4,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
            model_transform: Mat4::IDENTITY,
        }
    }

    pub fn translate_mut(&mut self, distance: Vec3) {
        self.model_transform.col_mut(3).x = distance.x;
        self.model_transform.col_mut(3).y = distance.y;
        self.model_transform.col_mut(3).z = distance.z;
    }

    fn center_mut(&mut self, transform: &Mat4) {
        let center_h = Vec4::from((self.center, 1.0));
        self.center = transform.mul_vec4(center_h).xyz();
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // calculate variables for quadratic equation
        let ray_to_center = ray.origin - self.center;

        let b = 2.0 * (ray.direction.dot(ray_to_center));

        let c = ray_to_center.length_squared() - self.radius * self.radius;

        let discrim = b * b - 4.0 * c;

        // no intersections
        if discrim < 0.0 {
            return None;
        }

        // exactly one intersection (hits surface)
        if discrim == 0.0 {
            let omega = -b / 2.0;

            // intersection behind the origin
            if omega < 0.0 {
                return None;
            }

            let i_x = ray.origin.x + ray.direction.x * omega;
            let i_y = ray.origin.y + ray.direction.y * omega;
            let i_z = ray.origin.z + ray.direction.z * omega;

            let n_x = i_x - self.center.x;
            let n_y = i_y - self.center.y;
            let n_z = i_z - self.center.z;

            return Some(Intersection::new(
                Vec3::new(i_x, i_y, i_z),
                Vec3::new(n_x, n_y, n_z).normalize(),
                self.material,
            ));
        }

        // multiple intersections
        let omega_p = (-b + discrim.sqrt()) / 2.0;
        let omega_m = (-b - discrim.sqrt()) / 2.0;

        let omega;

        // choose least positive
        if omega_p >= 0.0 && omega_m >= 0.0 {
            omega = omega_p.min(omega_m);
        } else if omega_p < 0.0 && omega_m < 0.0 {
            // println!("sphere: both roots negative");
            return None;
        } else {
            if omega_m >= 0.0 {
                omega = omega_m;
            } else {
                omega = omega_p;
            }
        }

        let i_x = ray.origin.x + ray.direction.x * omega;
        let i_y = ray.origin.y + ray.direction.y * omega;
        let i_z = ray.origin.z + ray.direction.z * omega;

        let n_x = i_x - self.center.x;
        let n_y = i_y - self.center.y;
        let n_z = i_z - self.center.z;

        Some(Intersection::new(
            Vec3::new(i_x, i_y, i_z),
            Vec3::new(n_x, n_y, n_z).normalize(),
            self.material,
        ))
    }

    fn to_world_space_mut(&mut self) {
        self.center_mut(&self.model_transform.clone());
    }

    fn to_view_space_mut(&mut self, view_transform: &Mat4) {
        self.center_mut(view_transform);
    }
}
