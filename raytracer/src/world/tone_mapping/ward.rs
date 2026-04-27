use glam::Vec3;

use crate::world::tone_mapping::tone_map::ToneMap;

pub struct Ward {
    num: f32,
    sf: f32,
}

impl Ward {
    pub fn new(ld_max: f32) -> Self {
        let num = 1.219 + (ld_max / 2.0).powf(0.4);


        Self {
            num,
            sf: 0.0,
        }
    }
}

impl ToneMap for Ward {
    fn set_log_avg(&mut self, log_avg: f32) {
        let denom = 1.219 + log_avg.powf(0.4);

        self.sf = (self.num / denom).powf(2.5);
    }

    fn compress(&self, radiance: Vec3) -> Vec3 {
        self.sf * radiance
    }
}