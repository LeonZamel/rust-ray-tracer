use crate::{three_d_tree::Axis, util::EPSILON, vec3::Vec3};

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}
impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
    pub fn intersect_axis_plane(&self, axis: &Axis, h: f64) -> f64 {
        if (self.direction.get_axis(axis)) == 0.0 {
            f64::INFINITY
        } else {
            (h - self.origin.get_axis(axis)) / self.direction.get_axis(axis)
        }
    }
}
