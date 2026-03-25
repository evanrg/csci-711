use std::any::Any;

use glam::{Vec2, Vec3};

pub trait Material: Any {
    fn get_color(&self, uv: Option<Vec2>) -> Vec3;
    fn get_spec_color(&self, uv: Option<Vec2>) -> Vec3;
    fn get_kr(&self) -> f32;
    fn get_kt(&self) -> f32;
    fn get_max_depth(&self) -> u32;
}

#[derive(Clone, Copy)]
pub struct FlatMaterial {
    color: Vec3,
    specular_color: Vec3,
    kr: f32,
    kt: f32,
    max_depth: u32,
}

impl FlatMaterial {
    pub fn new(color: Vec3, specular_color: Vec3, kr: f32, kt: f32, max_depth: u32) -> Self {
        Self {
            color,
            specular_color,
            kr,
            kt,
            max_depth,
        }
    }
}

impl Material for FlatMaterial {
    fn get_color(&self, _: Option<Vec2>) -> Vec3 {
        self.color
    }

    fn get_spec_color(&self, _: Option<Vec2>) -> Vec3 {
        self.specular_color
    }

    fn get_kr(&self) -> f32 {
        self.kr
    }

    fn get_kt(&self) -> f32 {
        self.kt
    }

    fn get_max_depth(&self) -> u32 {
        self.max_depth
    }
}

#[derive(Clone)]
pub struct ProceduralMaterial<'a> {
    color_func: &'a dyn Fn(Vec2) -> Vec3,
    specular_color_func: &'a dyn Fn(Vec2) -> Vec3,
    kr: f32,
    kt: f32,
    max_depth: u32,
}

impl<'a> ProceduralMaterial<'a> {
    pub fn new(
        color_func: &'a dyn Fn(Vec2) -> Vec3,
        specular_color_func: &'a dyn Fn(Vec2) -> Vec3,
        kr: f32,
        kt: f32,
        max_depth: u32,
    ) -> Self {
        Self {
            color_func,
            specular_color_func,
            kr,
            kt,
            max_depth,
        }
    }
}

impl<'a> Material for ProceduralMaterial<'static> {
    fn get_color(&self, uv: Option<Vec2>) -> Vec3 {
        (self.color_func)(uv.unwrap())
    }

    fn get_spec_color(&self, uv: Option<Vec2>) -> Vec3 {
        (self.specular_color_func)(uv.unwrap())
    }

    fn get_kr(&self) -> f32 {
        self.kr
    }

    fn get_kt(&self) -> f32 {
        self.kt
    }

    fn get_max_depth(&self) -> u32 {
        self.max_depth
    }
}
