use glam::{Mat4, Vec3};

use crate::{
    geometry::intersection::Intersection, lighting::{illumination::IlluminationModel, ray::Ray}, world::World,
};

pub struct PhongBlinn {
    ka: f32,
    kd: f32,
    ks: f32,
    ke: f32,
}

impl PhongBlinn {
    pub fn new(ka: f32, kd: f32, ks: f32, ke: f32) -> Self {
        Self { ka, kd, ks, ke }
    }
}

impl IlluminationModel for PhongBlinn {
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

        let mut total_ref = Vec3::new(0.0, 0.0, 0.0);

        let kr = intersection.object.get_kr();
        let kt = intersection.object.get_kt();

        let max_depth = intersection.object.get_max_depth();

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
            let h_i = (s_i + view_dir).normalize();
            let spec_angle = h_i.dot(intersection.normal);
            let spec_factor = spec_angle.powf(self.ke);

            total_spec_r += l_r * mat_s_r * spec_factor;
            total_spec_g += l_g * mat_s_g * spec_factor;
            total_spec_b += l_b * mat_s_b * spec_factor;

            // reflection calculation
            if depth < max_depth {

                if kr > 0.0 {
                    let reflected = s_i + 2.0 * angle / intersection.normal.length().powi(2) * intersection.normal;
                    let offset = reflected.normalize() * 0.001;
                    let refl_ray = Ray::new(intersection.intersection_point + offset, reflected.normalize());

                    if let Some(refl_int) = world.intersection_from_ray(&refl_ray) {
                        let refl_color = self.illuminate(world, &refl_int, cam_pos, view_transform, depth + 1);
                        total_ref += kr * refl_color;
                    } else {
                        total_ref += kr * world.background_radiance;
                    }
                }

                if kt > 0.0 {
                    // not handling this yet
                }
            }
        }

        let mut diffuse = Vec3::new(total_diff_r, total_diff_g, total_diff_b);
        diffuse *= self.kd;

        let mut specular = Vec3::new(total_spec_r, total_spec_g, total_spec_b);
        specular *= self.ks;

        radiance += diffuse;
        radiance += specular;
        radiance += total_ref;

        radiance
    }
}
