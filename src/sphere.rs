use crate::hittable::BoundingBox;
use crate::hittable::Hit;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(Hit::new(p, outward_normal, t, ray))
    }
    fn get_bounds(&self) -> BoundingBox {
        let radius = if self.radius < 0.0 {
            -self.radius
        } else {
            self.radius
        };
        BoundingBox::new(
            self.center.x - radius,
            self.center.x + radius,
            self.center.y - radius,
            self.center.y + radius,
            self.center.z - radius,
            self.center.z + radius,
        )
    }
}
