use glam::{Vec2, Vec3};

use crate::{
    geometry::{
        material::{FlatMaterial, ProceduralMaterial},
        primitives::{sphere::Sphere, triangle::Triangle},
    },
    lighting::{illumination::IlluminationType, light_source::LightSource},
    world::{World, camera::Camera},
};

mod geometry;
mod lighting;
mod world;

fn main() {
    // Get our objects setup
    let (sphere_gray, sphere_white) = create_spheres();
    let (triangle_left, triangle_right) = create_floor();

    // Setup camera
    let camera = create_camera();

    // Setup lighting
    let lights = create_lights();

    // Create our world
    let bg_color = Vec3::new(0.4, 0.6, 1.0);
    let mut world = World::new(bg_color);

    // Add all our lights
    for light in lights {
        world.add_light(light);
    }

    // Add all our objects
    world.add(sphere_gray);
    world.add(sphere_white);
    world.add(triangle_left);
    world.add(triangle_right);

    // apply all model transformations to the objects
    world.compile_object_models();

    // convert to world space
    world.objects_to_world_space();

    // convert to view space
    world.objects_to_view_space(camera.get_view_transform());
    world.lights_to_view_space(camera.get_view_transform());

    camera.render(&world, IlluminationType::PhongBlinn);
}

fn create_camera() -> Camera {
    let camera_pos = Vec3::new(0.0, 6.0, 10.0);
    let look_at = Vec3::new(0.0, 5.0, 0.0);
    let up = Vec3::new(0.0, 1.0, 0.0);

    let focal_length = 5.0;
    let img_dim = (1000, 600);
    let film_plane_dim = (5, 3);

    Camera::new(
        camera_pos,
        look_at,
        up,
        focal_length,
        img_dim,
        film_plane_dim,
    )
}

fn create_lights() -> Vec<LightSource> {
    let mut lights = vec![];

    let main_light_pos = Vec3::new(-1.0, 20.0, 10.0);
    let main_light_radiance = Vec3::new(1.0, 1.0, 1.0);
    let main_light_ambient = Vec3::new(1.0, 1.0, 1.0);

    lights.push(LightSource::new(
        main_light_pos,
        main_light_radiance,
        main_light_ambient,
    ));

    lights
}

fn checkerboard_color_func(uv: Vec2) -> Vec3 {
    let scaled_u = (uv.x * 10.0).floor() as i32;
    let scaled_v = (uv.y * 30.0).floor() as i32;

    if scaled_u % 2 == 0 && scaled_v % 2 == 0 {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    if scaled_u % 2 != 0 && scaled_v % 2 != 0 {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    Vec3::new(1.0, 1.0, 0.0)
}

fn checkerboard_spec_color_func(_: Vec2) -> Vec3 {
    Vec3::new(1.0, 1.0, 1.0)
}

fn create_floor() -> (Triangle, Triangle) {
    let triangle_l_material =
        ProceduralMaterial::new(&checkerboard_color_func, &checkerboard_spec_color_func, 0.0, 0.0, 1);
    let triangle_r_material =
        ProceduralMaterial::new(&checkerboard_color_func, &checkerboard_spec_color_func, 0.0, 0.0, 1);

    let tl_v1 = Vec3::new(0.0, 0.0, 0.0);
    let tl_v2 = Vec3::new(1.0, 0.0, 0.0);
    let tl_v3 = Vec3::new(0.0, 1.0, 0.0);

    let tr_v1 = Vec3::new(1.0, 0.0, 0.0);
    let tr_v2 = Vec3::new(1.0, 1.0, 0.0);
    let tr_v3 = Vec3::new(0.0, 1.0, 0.0);

    let mut triangle_l = Triangle::new((tl_v1, tl_v2, tl_v3), Box::new(triangle_l_material));
    let mut triangle_r = Triangle::new((tr_v1, tr_v2, tr_v3), Box::new(triangle_r_material));

    triangle_l.translate_mut(Vec3::new(-8.0, 0.0, 0.0));
    triangle_l.scale_mut(Vec3::new(30.0, 200.0, 1.0));
    triangle_l.rotate_x_mut(-90.0);

    triangle_r.translate_mut(Vec3::new(-8.0, 0.0, 0.0));
    triangle_r.scale_mut(Vec3::new(30.0, 200.0, 1.0));
    triangle_r.rotate_x_mut(-90.0);

    (triangle_l, triangle_r)
}

fn create_spheres() -> (Sphere, Sphere) {
    let spec_color = Vec3::new(1.0, 1.0, 1.0);

    let sphere_1_color = Vec3::new(0.25, 0.25, 0.25);
    let sphere_1_material = FlatMaterial::new(sphere_1_color, spec_color, 0.0, 0.0, 1);

    let sphere_2_color = Vec3::new(0.5, 0.5, 0.5);
    let sphere_2_material = FlatMaterial::new(sphere_2_color, spec_color, 0.5, 0.0, 20);

    let sphere_radius = 2.0;

    // in model space
    let sphere_pos = Vec3::new(0.0, 0.0, 0.0);

    let mut sphere_gray = Sphere::new(sphere_pos, sphere_radius, Box::new(sphere_1_material));
    sphere_gray.translate_mut(Vec3::new(-1.0, 6.0, -1.5));

    let mut sphere_white = Sphere::new(sphere_pos, sphere_radius, Box::new(sphere_2_material));
    sphere_white.translate_mut(Vec3::new(1.5, 4.5, -6.0));

    (sphere_gray, sphere_white)
}
