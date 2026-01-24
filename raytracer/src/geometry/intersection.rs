use glam::Vec3;

pub struct Intersection {
    intersection_point: Option<Vec3>,
    normal: Option<Vec3>,
}

impl Intersection {
    pub fn new(intersection_point: Option<Vec3>, normal: Option<Vec3>) -> Self {
        Self {
            intersection_point,
            normal,
        }
    }
}
