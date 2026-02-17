use glam::{Mat4, Vec3};
use image::{ImageBuffer, RgbImage};

use crate::{
    lighting::{
        illumination::{
            IlluminationModel, IlluminationType, ashikhmin_shirley::AshikhminShirley, phong::Phong,
            phong_blinn::PhongBlinn,
        },
        ray::Ray,
    },
    world::World,
};

pub struct Camera {
    position: Vec3,
    view_transform: Mat4,
    focal_length: f32,
    image_height: u32,
    image_width: u32,
    film_plane_height: u32,
    film_plane_width: u32,
}

impl Camera {
    pub fn new(
        position: Vec3,
        look_at: Vec3,
        up: Vec3,
        focal_length: f32,
        img_dim: (u32, u32),
        film_plane_dim: (u32, u32),
    ) -> Self {
        let n = (position - look_at).normalize_or_zero();
        let u = up.cross(n).normalize_or_zero();
        let v = n.cross(u);

        // col-major ordering
        let view_transform = Mat4::from_cols_array_2d(&[
            [u.x, v.x, n.x, 0.0],
            [u.y, v.y, n.y, 0.0],
            [u.z, v.z, n.z, 0.0],
            [-position.dot(u), -position.dot(v), -position.dot(n), 1.0],
        ]);

        Self {
            position,
            view_transform,
            focal_length,
            image_height: img_dim.1,
            image_width: img_dim.0,
            film_plane_height: film_plane_dim.1,
            film_plane_width: film_plane_dim.0,
        }
    }

    pub fn get_view_transform(&self) -> &Mat4 {
        &self.view_transform
    }

    pub fn render(&self, world: &World, illumination_type: IlluminationType) {
        let pixel_height = (self.film_plane_height as f32) / (self.image_height as f32);
        let pixel_width = (self.film_plane_width as f32) / (self.image_width as f32);

        let x_start = -(self.film_plane_width as f32) / 2.0 + pixel_width / 2.0;

        let mut curr_position = Vec3::new(
            x_start,
            self.film_plane_height as f32 / 2.0 + pixel_height / 2.0,
            -self.focal_length,
        );

        let w_offset = Vec3::new(pixel_width, 0.0, 0.0);
        let h_offset = Vec3::new(0.0, pixel_height, 0.0);

        let mut rendered: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        let mut ill_model: Box<dyn IlluminationModel> = match illumination_type {
            IlluminationType::Phong => Box::new(Phong::new(0.1, 0.7, 0.3, 32.0)),
            IlluminationType::PhongBlinn => Box::new(PhongBlinn::new(0.1, 0.7, 0.3, 32.0)),
            IlluminationType::AshikhminShirley => Box::new(AshikhminShirley::new(100.0, 100.0)),
        };

        // look at all our rays for intersections
        for y in 0..self.image_height {
            curr_position -= h_offset;

            for x in 0..self.image_width {
                curr_position += w_offset;

                let origin = Vec3::new(0.0, 0.0, 0.0);

                let top_left = curr_position - (w_offset / 4.0) - (h_offset / 4.0);
                let top_right = top_left + (w_offset / 2.0);
                let bottom_left = top_left - (h_offset / 2.0);
                let bottom_right = top_right - (h_offset / 2.0);

                let tl_dir = top_left.normalize();
                let tr_dir = top_right.normalize();
                let bl_dir = bottom_left.normalize();
                let br_dir = bottom_right.normalize();

                let dirs = vec![tl_dir, tr_dir, bl_dir, br_dir];

                let mut rads = [world.background_radiance; 4];

                for dir_idx in 0..dirs.len() {
                    let dir = dirs[dir_idx];
                    let ray = Ray::new(origin, dir);
                    let intersection = world.intersection_from_ray(&ray);

                    if let Some(int) = intersection {
                        rads[dir_idx] = ill_model.illuminate(world, &int, self.position);
                    }
                }

                let mut avg_radiance = Vec3::new(0.0, 0.0, 0.0);
                for rad in rads {
                    avg_radiance += rad;
                }
                avg_radiance /= rads.len() as f32;

                let r = (avg_radiance.x * 255.0).min(255.0);
                let g = (avg_radiance.y * 255.0).min(255.0);
                let b = (avg_radiance.z * 255.0).min(255.0);

                *rendered.get_pixel_mut(x, y) = image::Rgb([r as u8, g as u8, b as u8]);
            }

            curr_position.x = x_start;
        }

        rendered.save("render.png").unwrap();
    }
}
