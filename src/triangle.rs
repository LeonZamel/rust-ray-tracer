use crate::ray::Ray;
use crate::util::EPSILON;
use crate::vec3::Vec3;

pub struct Triangle {
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
}
impl Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        let p12 = self.p1 - self.p2;
        let p13 = self.p1 - self.p3;
        let normal = p12.cross(&p13);
        if ray.direction.dot(&normal) < EPSILON {
            // Ray and normal of plane are othogonal => Ray and plane are parallel
            return Option::None;
        } else {
            // There is an intersection of the plane and the ray, calculate intersection-point
            // Get distance from ray-origin to plane
            let dist = normal.dot(&ray.origin) - normal.dot(&self.p1);
            // Get intersection point
            let intersection = ray.origin + (ray.direction / ray.direction.length()) * dist;
            // Check that intersection point is in triangle
            if intersection.
            Option::from(intersection)
        }
    }

}
