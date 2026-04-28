use std::any::Any;

use glam::Vec3;

pub trait ToneMap: Any {
    fn compress(&self, radiance: Vec3) -> Vec3;

    fn set_log_avg(&mut self, log_avg: f32);

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub enum ToneMapType {
    Reinhard,
    Ward,
    AdaptiveLog,
}
