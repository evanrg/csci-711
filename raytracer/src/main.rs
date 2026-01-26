use glam::Vec3;

use crate::{
    geometry::{
        material::Material,
        primitives::{sphere::Sphere, triangle::Triangle},
    },
    world::{World, camera::Camera},
};

mod geometry;
mod lighting;
mod world;

fn main() {
    // Get our objects setup
    let sphere_color = Vec3::new(1.0, 0.0, 0.0);
    let sphere_material = Material::new(sphere_color);

    let triangle_color = Vec3::new(0.0, 0.0, 1.0);
    let triangle_material = Material::new(triangle_color);

    let sphere_radius = 2.0;

    // in model space
    let sphere_pos = Vec3::new(0.0, 0.0, 0.0);

    let v1 = Vec3::new(0.0, 0.5, 0.0);
    let v2 = Vec3::new(-0.5, 0.0, 0.0);
    let v3 = Vec3::new(0.5, 0.0, 0.0);

    let mut triangle = Triangle::new((v1, v2, v3), triangle_material);
    triangle.translate_mut(Vec3::new(0.0, 0.0, 0.0));

    let mut sphere = Sphere::new(sphere_pos, sphere_radius, sphere_material);
    sphere.translate_mut(Vec3::new(0.0, 0.0, -10.0));

    // Setup camera
    let camera_pos = Vec3::new(0.0, 0.0, 10.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let focal_length = 10.0;
    let img_dim = (500, 500);
    let film_plane_dim = (10, 10);

    let camera = Camera::new(
        camera_pos,
        look_at,
        up,
        focal_length,
        img_dim,
        film_plane_dim,
    );

    // Create our world
    let bg_color = Vec3::new(1.0, 1.0, 1.0);
    let mut world = World::new(bg_color);

    world.add(sphere);
    world.add(triangle);

    // convert to world space
    world.objects_to_world_space();

    // convert to view space
    world.objects_to_view_space(camera.get_view_transform());

    camera.render(&world);
}
