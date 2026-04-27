use glam::Vec3;

use crate::world::tone_mapping::tone_map::ToneMap;

pub struct Reinhard {
    key: f32,
    key_over_avg: f32,
    ld_max: f32,
}

impl Reinhard {
    pub fn new(pct_gray: f32, ld_max: f32) -> Self {

        Self {
            key: pct_gray,
            key_over_avg: 0.0,
            ld_max,
        }
    }
}

impl ToneMap for Reinhard {

    fn set_log_avg(&mut self, log_avg: f32) {
        self.key_over_avg = self.key / log_avg;
    }

    fn compress(&self, radiance: Vec3) -> Vec3 {
        let r_s = self.key_over_avg * radiance;

        let r_r = r_s.x / (1.0 + r_s.x);
        let g_r = r_s.y / (1.0 + r_s.y);
        let b_r = r_s.z / (1.0 + r_s.z);

        let rad_r = Vec3::new(r_r, g_r, b_r);

        rad_r * self.ld_max
    }
}