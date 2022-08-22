use crate::hittable::BoundingBox;
use crate::hittable::Hit;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::util::EPSILON;
use crate::vec3::Vec3;
pub struct Triangle {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}
impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let p12 = self.p2 - self.p1;
        let p23 = self.p3 - self.p2;
        let p31 = self.p1 - self.p3;
        let normal = p12.cross(&p23).unit_vector();
        if ray.direction.dot(&normal).abs() < EPSILON {
            // Ray and normal of plane are othogonal => Ray and plane are parallel
            return Option::None;
        }
        // There is an intersection of the plane and the ray

        // Get distance from ray-origin to plane
        let dist: f64 = normal.dot(&ray.origin) - normal.dot(&self.p1);
        // Get intersection point of plane and ray
        let t = -dist / normal.dot(&ray.direction);
        if t < t_min || t > t_max {
            return Option::None;
        }
        let intersection = ray.origin + ray.direction * t;
        // Check that intersection point is in triangle
        let pi1 = intersection - self.p1;
        let pi2 = intersection - self.p2;
        let pi3 = intersection - self.p3;

        if (p23 - p12 * (p12.dot(&p23) / p12.length_squared())).dot(&pi1) > 0.0
            && (p31 - p23 * p23.dot(&p31) / p23.length_squared()).dot(&pi2) > 0.0
            && (p12 - p31 * p31.dot(&p12) / p31.length_squared()).dot(&pi3) > 0.0
        {
            Option::from(Hit::new(intersection, normal, t, ray))
        } else {
            Option::None
        }
    }

    fn get_bounds(&self) -> BoundingBox {
        BoundingBox::new(
            self.p1.x.min(self.p2.x).min(self.p3.x),
            self.p1.x.max(self.p2.x).max(self.p3.x),
            self.p1.y.min(self.p2.y).min(self.p3.y),
            self.p1.y.max(self.p2.x).max(self.p3.y),
            self.p1.z.min(self.p2.z).min(self.p3.z),
            self.p1.z.max(self.p2.z).max(self.p3.z),
        )
    }
}
