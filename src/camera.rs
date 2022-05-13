use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub focal_length: f64,
    horizontal: Vec3,
    vertical: Vec3,
    pub lower_left_viewport_corner: Vec3,
}
impl Camera {
    pub fn new(
        origin: Vec3,
        aspect_ratio: f64,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
    ) -> Camera {
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_viewport_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            horizontal,
            vertical,
            origin,
            lower_left_viewport_corner,
        }
    }
    pub fn get_ray(&self, horizontal_frac: f64, vertical_frac: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_viewport_corner
                + self.horizontal * horizontal_frac
                + self.vertical * vertical_frac,
        }
    }
}
