use crate::hittable::Hit;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
use crate::util::EPSILON;
use crate::vec3::Vec3;
pub struct Triangle {
    pub material: Box<dyn Material>,
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3,
}
impl Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Vec3> {
        let p12 = self.p2 - self.p1;
        let p23 = self.p3 - self.p2;
        let p31 = self.p1 - self.p3;
        let normal = p12.cross(&p31);
        if ray.direction.dot(&normal) < EPSILON {
            // Ray and normal of plane are othogonal => Ray and plane are parallel
            return Option::None;
        } else {
            // There is an intersection of the plane and the ray

            // Get distance from ray-origin to plane
            let dist = normal.dot(&ray.origin) - normal.dot(&self.p1);
            if dist < 0.0 {
                // The triangle is behind the ray
                return Option::None;
            }
            // Get intersection point of plane and ray
            let t = dist / normal.dot(&ray.direction);
            let intersection = ray.origin + (ray.direction / ray.direction.length()) * t;
            // Check that intersection point is in triangle
            let pi1 = intersection - self.p1;
            let pi2 = intersection - self.p2;
            let pi3 = intersection - self.p3;

            if (p23 - p12 * p12.dot(&p23)).dot(&pi1) > 0.0
                && (p31 - p23 * p23.dot(&p31)).dot(&pi2) > 0.0
                && (p12 - p31 * p31.dot(&p12)).dot(&pi3) > 0.0
            {
                Option::from(intersection)
            } else {
                Option::None
            }
        }
    }
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
        if dist < 0.0 {
            // The triangle is behind the ray
            return Option::None;
        }
        // Get intersection point of plane and ray
        let t = -dist / normal.dot(&ray.direction);
        let intersection = ray.origin + ray.direction * t;
        // Check that intersection point is in triangle
        let pi1 = intersection - self.p1;
        let pi2 = intersection - self.p2;
        let pi3 = intersection - self.p3;

        // return Option::from(Hit {
        //     front_face: true,
        //     normal,
        //     t: t,
        //     p: intersection,
        //     material: self.material.as_ref(),
        // });Option::None
        if (p23 - p12 * (p12.dot(&p23) / p12.length_squared())).dot(&pi1) > 0.0
            && (p31 - p23 * p23.dot(&p31) / p23.length_squared()).dot(&pi2) > 0.0
            && (p12 - p31 * p31.dot(&p12) / p31.length_squared()).dot(&pi3) > 0.0
        {
            Option::from(Hit {
                front_face: true,
                normal,
                t: t,
                p: intersection,
                material: self.material.as_ref(),
            })
        } else {
            Option::None
        }
    }
}
