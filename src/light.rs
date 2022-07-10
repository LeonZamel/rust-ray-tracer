use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Light {
    fn at(&self, origin: Vec3, world: &HittableList) -> LightInfo;
    fn no_hit(&self, ray: &Ray) -> Vec3;
}

pub struct LightInfo {
    pub color: Vec3,
    pub direction: Vec3, // Direction of the light from the hit
}

pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f64,
}
impl Light for PointLight {
    fn at(&self, origin: Vec3, world: &HittableList) -> LightInfo {
        let direction = (self.position - origin).unit_vector();
        match world.hit_default(&Ray {
            direction,
            origin: origin,
        }) {
            None => LightInfo {
                color: (self.color * self.intensity) / (self.position - origin).length().powf(2.0),
                direction,
            },
            Some(_) => LightInfo {
                color: Vec3::z(),
                direction,
            },
        }
    }

    fn no_hit(&self, ray: &Ray) -> Vec3 {
        // Gives the light a "body" which looks good in reflections
        let dist = ((ray.origin
            + ray.direction * (self.position - ray.origin).dot(&ray.direction))
            - self.position)
            .length();
        (((self.color * self.intensity) / (self.position - ray.origin).length().powf(2.0))
            / dist.powf(2.0))
            * (self.position - ray.origin)
                .unit_vector()
                .dot(&ray.direction)
    }
}
