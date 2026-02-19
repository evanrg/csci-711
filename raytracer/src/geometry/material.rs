use glam::{Vec2, Vec3};

pub trait Material {
    fn get_color(&self, uv: Vec2) -> Vec3;
    fn get_spec_color(&self, uv: Vec2) -> Vec3;
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
    fn get_color(&self, uv: Vec2) -> Vec3 {
        self.color
    }

    fn get_spec_color(&self, uv: Vec2) -> Vec3 {
        self.specular_color
    }
}

#[derive(Clone, Copy)]
pub struct ProceduralMaterial<F>
where
    F: Fn(Vec2) -> Vec3,
{
    pub color_func: F,
    pub specular_color_func: F,
}

impl<F> ProceduralMaterial<F>
where
    F: Fn(Vec2) -> Vec3,
{
    fn new(color_func: F, specular_color_func: F) -> Self {
        Self {
            color_func,
            specular_color_func,
        }
    }
}

impl<F> Material for ProceduralMaterial<F>
where
    F: Fn(Vec2) -> Vec3,
{
    fn get_color(&self, uv: Vec2) -> Vec3 {
        (self.color_func)(uv)
    }

    fn get_spec_color(&self, uv: Vec2) -> Vec3 {
        (self.specular_color_func)(uv)
    }
}
