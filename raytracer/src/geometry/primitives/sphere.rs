use glam::Vec3;

use crate::{
    geometry::{intersection::Intersection, material::Material, object::Object},
    lighting::ray::Ray,
};

pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Material,
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Intersection {
        // calculate variables for quadratic equation
        let ray_to_center_x = ray.origin.x - self.center.x;
        let ray_to_center_y = ray.origin.y - self.center.y;
        let ray_to_center_z = ray.origin.z - self.center.z;

        let b = 2.0
            * (ray.direction.x * ray_to_center_x
                + ray.direction.y * ray_to_center_y
                + ray.direction.z * ray_to_center_z);

        let c = ray_to_center_x * ray_to_center_x
            + ray_to_center_y * ray_to_center_y
            + ray_to_center_z * ray_to_center_z;

        let discrim = b * b - 4.0 * c;

        // no intersections
        if discrim < 0.0 {
            return Intersection::new(None, None);
        }

        // exactly one intersection (hits surface)
        if discrim == 0.0 {
            let omega = -b / 2.0;

            // intersection behind the origin
            if omega < 0.0 {
                return Intersection::new(None, None);
            }

            let i_x = ray.origin.x + ray.direction.x * omega;
            let i_y = ray.origin.y + ray.direction.y * omega;
            let i_z = ray.origin.z + ray.direction.z * omega;

            let n_x = i_x - self.center.x;
            let n_y = i_y - self.center.y;
            let n_z = i_z - self.center.z;

            return Intersection::new(
                Some(Vec3::new(i_x, i_y, i_z)),
                Some(Vec3::new(n_x, n_y, n_z).normalize()),
            );
        }

        // multiple intersections
        let omega_p = (-b + discrim.sqrt()) / 2.0;
        let omega_m = (-b - discrim.sqrt()) / 2.0;

        let omega;

        // choose least positive
        if omega_p >= 0.0 && omega_m >= 0.0 {
            omega = omega_p.min(omega_m);
        } else if omega_p < 0.0 && omega_m < 0.0 {
            return Intersection::new(None, None);
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

        Intersection::new(
            Some(Vec3::new(i_x, i_y, i_z)),
            Some(Vec3::new(n_x, n_y, n_z).normalize()),
        )
    }

    fn get_material(&self) -> &Material {
        return &self.material;
    }
}
