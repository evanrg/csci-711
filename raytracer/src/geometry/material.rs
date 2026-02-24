use std::any::Any;

use glam::{Vec2, Vec3};
use image::{DynamicImage, GenericImageView, ImageReader};

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

#[derive(Clone)]
pub struct TextureMaterial {
    pub texture_img: DynamicImage,
    pub img_width: u32,
    pub img_height: u32,
}

impl TextureMaterial {
    pub fn new(image_path: String) -> Self {
        let texture = ImageReader::open(image_path).unwrap().decode().unwrap();
        let img_width = texture.width();
        let img_height = texture.height();

        Self {
            texture_img: texture,
            img_width,
            img_height,
        }
    }
}

impl Material for TextureMaterial {
    fn get_color(&self, uv: Option<Vec2>) -> Vec3 {
        let uv = uv.unwrap();
        let scaled_uv = Vec2::new(
            uv.x * (self.img_width - 1) as f32,
            uv.y * (self.img_height - 1) as f32,
        );
        let pixel_value = Vec2::new(scaled_uv.x.floor(), scaled_uv.y.floor());

        let rgba = self
            .texture_img
            .get_pixel(pixel_value.x as u32, pixel_value.y as u32)
            .0;

        Vec3::new(
            rgba[0] as f32 / 255.0,
            rgba[1] as f32 / 255.0,
            rgba[2] as f32 / 255.0,
        )
    }

    fn get_spec_color(&self, _: Option<Vec2>) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0)
    }
}
