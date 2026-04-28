use std::any::Any;

use glam::Vec3;

//
// Tone maps use the compress function to
// determine the new output color
// This should ultimately be in the 0.0-1.0
// range, but the camera will clamp it just
// in case.
//

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
