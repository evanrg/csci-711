use glam::Vec3;

pub trait ToneMap {
    fn compress(&self, radiance: Vec3) -> Vec3;

    fn set_log_avg(&mut self, log_avg: f32);
}

pub enum ToneMapType {
    Reinhard,
    Ward
}