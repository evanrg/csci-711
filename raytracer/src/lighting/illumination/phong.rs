use glam::{Mat4, Vec3};

use crate::{
    geometry::intersection::Intersection, lighting::illumination::IlluminationModel, world::World,
};

pub struct Phong {
    ka: f32,
    kd: f32,
    ks: f32,
    ke: f32,
}

impl Phong {
    pub fn new(ka: f32, kd: f32, ks: f32, ke: f32) -> Self {
        Self { ka, kd, ks, ke }
    }
}

impl IlluminationModel for Phong {
    fn illuminate(
        &mut self,
        world: &World,
        intersection: &Intersection,
        cam_pos: Vec3,
        view_transform: &Mat4,
        depth: u32,
    ) -> Vec3 {
        let mat_color = intersection
            .object
            .get_color(view_transform, intersection.intersection_point);

        let mat_r = mat_color.x;
        let mat_g = mat_color.y;
        let mat_b = mat_color.z;

        // La term which will just be average ambient of all the lights
        let mut avg_amb = Vec3::new(0.0, 0.0, 0.0);
        for light in world.lights.iter() {
            avg_amb += light.ambient;
        }
        avg_amb /= world.lights.len() as f32;

        let amb_r = mat_r * avg_amb.x;
        let amb_g = mat_g * avg_amb.y;
        let amb_b = mat_b * avg_amb.z;

        // default radiance is just the ambient color
        let mut radiance = self.ka * Vec3::new(amb_r, amb_g, amb_b);

        let mut total_diff_r = 0.0;
        let mut total_diff_g = 0.0;
        let mut total_diff_b = 0.0;

        let mat_spec_color = intersection
            .object
            .get_specular_color(view_transform, intersection.intersection_point);

        let mat_s_r = mat_spec_color.x;
        let mat_s_g = mat_spec_color.y;
        let mat_s_b = mat_spec_color.z;

        let mut total_spec_r = 0.0;
        let mut total_spec_g = 0.0;
        let mut total_spec_b = 0.0;

        let view_dir = (cam_pos - intersection.intersection_point).normalize();

        for light in world.lights.iter() {
            if !world.can_see_light(intersection, light.position) {
                continue;
            }

            // diffuse calculation
            let s_i = (light.position - intersection.intersection_point).normalize();
            let angle = s_i.dot(intersection.normal);

            let l_r = light.radiance.x;
            let l_g = light.radiance.y;
            let l_b = light.radiance.z;

            total_diff_r += l_r * mat_r * angle;
            total_diff_g += l_g * mat_g * angle;
            total_diff_b += l_b * mat_b * angle;

            // specular calculation
            let r_i = s_i.reflect(intersection.normal).normalize();
            let spec_angle = r_i.dot(view_dir);
            let spec_factor = spec_angle.powf(self.ke);

            total_spec_r += l_r * mat_s_r * spec_factor;
            total_spec_g += l_g * mat_s_g * spec_factor;
            total_spec_b += l_b * mat_s_b * spec_factor;
        }

        let mut diffuse = Vec3::new(total_diff_r, total_diff_g, total_diff_b);
        diffuse *= self.kd;

        let mut specular = Vec3::new(total_spec_r, total_spec_g, total_spec_b);
        specular *= self.ks;

        radiance += diffuse;
        radiance += specular;

        radiance
    }
}
