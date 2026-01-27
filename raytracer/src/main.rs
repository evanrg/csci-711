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

    let triangle_1_color = Vec3::new(0.0, 0.0, 1.0);
    let triangle_2_color = Vec3::new(0.0, 1.0, 0.0);

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

    let mut triangle_blue = Triangle::new((t1_v1, t1_v2, t1_v3), triangle_1_material);
    let mut triangle_green = Triangle::new((t2_v1, t2_v2, t2_v3), triangle_2_material);

    triangle_blue.translate_mut(Vec3::new(-6.0, -6.0, 0.0));
    triangle_blue.scale_mut(Vec3::new(20.0, 100.0, 1.0));
    triangle_blue.rotate_x_mut(-90.0);

    triangle_green.translate_mut(Vec3::new(-6.0, -6.0, 0.0));
    triangle_green.scale_mut(Vec3::new(20.0,100.0, 1.0));
    triangle_green.rotate_x_mut(-90.0);

    triangle_blue.compile_model();
    triangle_green.compile_model();

    let mut sphere = Sphere::new(sphere_pos, sphere_radius, sphere_material);
    sphere.translate_mut(Vec3::new(0.0, 0.0, -10.0));

    // Setup camera
    let camera_pos = Vec3::new(0.0, 0.0, 10.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let focal_length = 5.0;
    let img_dim = (800, 600);
    let film_plane_dim = (5, 5);

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
    world.add(triangle_blue);
    world.add(triangle_green);
    

    // convert to world space
    world.objects_to_world_space();

    // convert to view space
    world.objects_to_view_space(camera.get_view_transform());

    camera.render(&world);
}
