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
    let sphere_1_color = Vec3::new(0.5, 0.5, 0.5);
    let sphere_1_material = Material::new(sphere_1_color);

    let sphere_2_color = Vec3::new(1.0, 1.0, 1.0);
    let sphere_2_material = Material::new(sphere_2_color);

    let triangle_1_color = Vec3::new(1.0, 0.0, 0.0);
    let triangle_2_color = Vec3::new(1.0, 1.0, 0.0);

    let triangle_1_material = Material::new(triangle_1_color);
    let triangle_2_material = Material::new(triangle_2_color);

    let sphere_radius = 2.0;

    // in model space
    let sphere_pos = Vec3::new(0.0, 0.0, 0.0);

    let t1_v1 = Vec3::new(0.0, 0.0, 0.0);
    let t1_v2 = Vec3::new(1.0, 0.0, 0.0);
    let t1_v3 = Vec3::new(0.0, 1.0, 0.0);

    let t2_v1 = Vec3::new(1.0, 0.0, 0.0);
    let t2_v2 = Vec3::new(1.0, 1.0, 0.0);
    let t2_v3 = Vec3::new(0.0, 1.0, 0.0);

    let mut triangle_1 = Triangle::new((t1_v1, t1_v2, t1_v3), triangle_1_material);
    let mut triangle_2 = Triangle::new((t2_v1, t2_v2, t2_v3), triangle_2_material);

    triangle_1.translate_mut(Vec3::new(-8.0, -6.0, -10.0));
    triangle_1.scale_mut(Vec3::new(30.0, 200.0, 1.0));
    triangle_1.rotate_x_mut(-90.0);

    triangle_2.translate_mut(Vec3::new(-8.0, -6.0, -10.0));
    triangle_2.scale_mut(Vec3::new(30.0, 200.0, 1.0));
    triangle_2.rotate_x_mut(-90.0);

    triangle_1.compile_model();
    triangle_2.compile_model();

    let mut sphere_gray = Sphere::new(sphere_pos, sphere_radius, sphere_1_material);
    sphere_gray.translate_mut(Vec3::new(-1.0, 0.5, -1.5));

    let mut sphere_white = Sphere::new(sphere_pos, sphere_radius, sphere_2_material);
    sphere_white.translate_mut(Vec3::new(1.5, -1.0, -6.0));

    // Setup camera
    let camera_pos = Vec3::new(0.0, 0.0, 10.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let focal_length = 5.0;
    let img_dim = (800, 600);
    let film_plane_dim = (4, 3);

    let camera = Camera::new(
        camera_pos,
        look_at,
        up,
        focal_length,
        img_dim,
        film_plane_dim,
    );

    // Create our world
    let bg_color = Vec3::new(0.5, 0.9, 1.0);
    let mut world = World::new(bg_color);

    world.add(sphere_gray);
    world.add(sphere_white);
    world.add(triangle_1);
    world.add(triangle_2);

    // convert to world space
    world.objects_to_world_space();

    // convert to view space
    world.objects_to_view_space(camera.get_view_transform());

    camera.render(&world);
}
