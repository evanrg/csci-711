use glam::Vec3;

use crate::{
    geometry::{material::Material, primitives::sphere::Sphere},
    world::{World, camera::Camera},
};

mod geometry;
mod lighting;
mod world;

fn main() {
    // Get our objects setup
    let sphere_color = Vec3::new(1.0, 0.0, 0.0);
    let sphere_material = Material::new(sphere_color);

    let sphere_radius = 2.0;

    let sphere_1_pos = Vec3::new(7.82, 6.09, 6.43);
    let sphere_2_pos = Vec3::new(11.13, 4.0, -1.23);

    let sphere_1 = Sphere::new(sphere_1_pos, sphere_radius, sphere_material);
    let sphere_2 = Sphere::new(sphere_2_pos, sphere_radius, sphere_material);

    // Setup camera
    let camera_pos = Vec3::new(8.25, 6.43, 21.52);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let camera = Camera::new(camera_pos, look_at, up);

    // Create our world
    let mut world = World::new();

    world.add(sphere_1);
    world.add(sphere_2);

    camera.render(&world);
}
