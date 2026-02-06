use glam::{Mat4, Vec3, Vec4, Vec4Swizzles};

pub struct LightSource {
    pub position: Vec3,
    pub radiance: Vec3,
    pub ambient: Vec3,
}

impl LightSource {
    pub fn new(position: Vec3, radiance: Vec3, ambient: Vec3) -> Self {
        Self {
            position,
            radiance,
            ambient,
        }
    }

    pub fn to_view_space_mut(&mut self, view_transform: &Mat4) {
        let pos_h = Vec4::from((self.position, 1.0));
        self.position = view_transform.mul_vec4(pos_h).xyz();
    }
}
