use crate::vec3::Vec3;

pub const PI: f64 = 3.1415926535897932385;
pub const EPSILON: f64 = 0.001;

pub fn normal_to_color(normal: &Vec3) -> Vec3 {
    Vec3::new(1.0 + normal.x, 1.0 + normal.y, 1.0 + normal.z) * 0.5
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
