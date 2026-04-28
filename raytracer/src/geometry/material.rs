use glam::{Vec2, Vec3};
use image::{DynamicImage, GenericImageView, ImageReader};
use std::any::Any;

//
// Materials are responsible for knowing
// diffuse and specular color given some UV
// coordinate, as well as reflection and
// transmission coefficients, and max depth
// for reflections
//
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

//
// Procedural materials take two functions
// for colors rather than Vec3's in order
// to get the colors at runtime based on
// UV coordinates
//

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

//
// Texture materials take an image
// rather than colors since the colors
// can be grabbed from the image.
// Specular maps are not implemented
// so as a default, specular color
// is always white.
//
#[derive(Clone)]
pub struct TextureMaterial {
    pub texture_img: DynamicImage,
    pub img_width: u32,
    pub img_height: u32,
    kr: f32,
    kt: f32,
    max_depth: u32,
}

impl TextureMaterial {
    pub fn new(image_path: String, kr: f32, kt: f32, max_depth: u32) -> Self {
        let texture = ImageReader::open(image_path).unwrap().decode().unwrap();
        let img_width = texture.width();
        let img_height = texture.height();

        Self {
            texture_img: texture,
            img_width,
            img_height,
            kr,
            kt,
            max_depth,
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
