use std::any::Any;

use glam::{Vec2, Vec3};

pub trait Material: Any {
    fn get_color(&self, uv: Option<Vec2>) -> Vec3;
    fn get_spec_color(&self, uv: Option<Vec2>) -> Vec3;
}

#[derive(Clone, Copy)]
pub struct FlatMaterial {
    pub color: Vec3,
    pub specular_color: Vec3,
}

impl FlatMaterial {
    pub fn new(color: Vec3, specular_color: Vec3) -> Self {
        Self {
            color,
            specular_color,
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
}

#[derive(Clone)]
pub struct ProceduralMaterial<'a> {
    pub color_func: &'a dyn Fn(Vec2) -> Vec3,
    pub specular_color_func: &'a dyn Fn(Vec2) -> Vec3,
}

impl<'a> ProceduralMaterial<'a> {
    pub fn new(
        color_func: &'a dyn Fn(Vec2) -> Vec3,
        specular_color_func: &'a dyn Fn(Vec2) -> Vec3,
    ) -> Self {
        Self {
            color_func,
            specular_color_func,
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
}
