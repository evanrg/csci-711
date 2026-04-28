use std::any::Any;

use glam::Vec3;

use crate::world::tone_mapping::tone_map::ToneMap;

pub struct AdaptiveLog {
    b: f32,
    ld_max: f32,
    log_avg: f32,
    lw_max: f32,
}

impl AdaptiveLog {
    pub fn new(b: f32, ld_max: f32) -> Self {
        Self {
            b,
            ld_max,
            log_avg: 0.0,
            lw_max: 0.0,
        }
    }

    pub fn set_lw_max(&mut self, lw_max: f32) {
        self.lw_max = lw_max;
    }
}

impl ToneMap for AdaptiveLog {
    fn set_log_avg(&mut self, log_avg: f32) {
        self.log_avg = log_avg;

        self.lw_max /= self.log_avg;
    }

    fn compress(&self, radiance: Vec3) -> Vec3 {
        let raw_lum = 0.27 * radiance.x + 0.67 * radiance.y + 0.06 * radiance.z;
        let lum = raw_lum / self.log_avg;

        let ff = 1.0 / (self.lw_max + 1.0).log10();
        let num = (lum + 1.0).ln();

        let pow = self.b.ln() / (0.5 as f32).ln();
        let ratio = lum / self.lw_max;

        let denom = (2.0 + (ratio.powf(pow)) * 8.0).ln();

        let ld = self.ld_max * (ff * (num / denom));

        radiance * (ld / raw_lum)
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
