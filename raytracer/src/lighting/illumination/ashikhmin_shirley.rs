use std::f32::consts::PI;

use glam::{Mat4, Vec3};

use crate::{
    geometry::intersection::Intersection, lighting::illumination::IlluminationModel, world::World,
};

pub struct AshikhminShirley {
    nu: f32,
    nv: f32,
}

impl AshikhminShirley {
    pub fn new(nu: f32, nv: f32) -> Self {
        Self { nu, nv }
    }
}

impl IlluminationModel for AshikhminShirley {
    fn illuminate(
        &mut self,
        world: &World,
        intersection: &Intersection,
        cam_pos: Vec3,
        view_transform: &Mat4,
    ) -> Vec3 {
        // La term which will just be average ambient of all the lights
        let mut avg_amb = Vec3::new(0.0, 0.0, 0.0);
        for light in world.lights.iter() {
            avg_amb += light.ambient;
        }
        avg_amb /= world.lights.len() as f32;

        let mat_color = intersection
            .object
            .get_color(view_transform, intersection.intersection_point);

        let mat_r = mat_color.x;
        let mat_g = mat_color.y;
        let mat_b = mat_color.z;

        let amb_r = mat_r * avg_amb.x;
        let amb_g = mat_g * avg_amb.y;
        let amb_b = mat_b * avg_amb.z;

        let mut radiance = Vec3::new(amb_r, amb_g, amb_b);

        let mat_spec_color = intersection
            .object
            .get_specular_color(view_transform, intersection.intersection_point);

        let mat_s_r = mat_spec_color.x;
        let mat_s_g = mat_spec_color.y;
        let mat_s_b = mat_spec_color.z;

        // diffuse stuff we will need
        let diff_fact = 28.0 / (23.0 * PI);

        let diff_fact_r = diff_fact * mat_r * (1.0 - mat_s_r);
        let diff_fact_g = diff_fact * mat_g * (1.0 - mat_s_g);
        let diff_fact_b = diff_fact * mat_b * (1.0 - mat_s_b);

        let k2 = (cam_pos - intersection.intersection_point).normalize();

        let mut diffuse = Vec3::new(0.0, 0.0, 0.0);

        // specular stuff we will need
        let spec_fact = ((self.nu + 1.0) * (self.nv + 1.0)).sqrt() / (8.0 * PI);

        let n = intersection.normal;

        let mut specular = Vec3::new(0.0, 0.0, 0.0);

        for light in world.lights.iter() {
            if !world.can_see_light(intersection, light.position) {
                continue;
            }

            // diffuse calculations
            let k1 = (light.position - intersection.intersection_point).normalize();
            let light_angle = n.dot(k1);

            let k1_fact = 1.0 - (1.0 - (n.dot(k1)) / 2.0).powi(5);
            let k2_fact = 1.0 - (1.0 - (n.dot(k2)) / 2.0).powi(5);

            let diff_r = diff_fact_r * k1_fact * k2_fact * light.radiance.x;
            let diff_g = diff_fact_g * k1_fact * k2_fact * light.radiance.y;
            let diff_b = diff_fact_b * k1_fact * k2_fact * light.radiance.z;

            diffuse += Vec3::new(diff_r, diff_g, diff_b) * light_angle;

            // specular calculations
            if !(light_angle > 0.0 && n.dot(k2) > 0.0) {
                continue;
            }
            let h = (k1 + k2).normalize();

            let fresnel_r = mat_s_r + (1.0 - mat_s_r) * (1.0 - (k1.dot(h))).powi(5);
            let fresnel_g = mat_s_g + (1.0 - mat_s_g) * (1.0 - (k1.dot(h))).powi(5);
            let fresnel_b = mat_s_b + (1.0 - mat_s_b) * (1.0 - (k1.dot(h))).powi(5);

            let arb = if n.x.abs() > 0.9 {
                Vec3::new(1.0, 0.0, 0.0)
            } else {
                Vec3::new(0.0, 1.0, 0.0)
            };

            let u = arb.cross(n).normalize();
            let v = n.cross(u);

            let exp = (self.nu * (h.dot(u)).powi(2) + self.nv * (h.dot(v).powi(2)))
                / (1.0 - h.dot(n).powi(2));
            let num = (n.dot(h)).powf(exp);
            let den = h.dot(k1) * n.dot(k1).max(n.dot(k2));

            let spec_most = spec_fact * num / den;

            let spec_r = spec_most * fresnel_r * light.radiance.x;
            let spec_g = spec_most * fresnel_g * light.radiance.y;
            let spec_b = spec_most * fresnel_b * light.radiance.z;

            specular += Vec3::new(spec_r, spec_g, spec_b) * light_angle;
        }

        radiance += diffuse;
        radiance += specular;

        radiance
    }

    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
