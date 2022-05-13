use crate::vec3::Vec3;

pub fn normal_to_color(normal: &Vec3) -> Vec3 {
    Vec3::new(1.0 + normal.x, 1.0 + normal.y, 1.0 + normal.z) * 0.5
}
