use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Light {
    fn at(&self, origin: Vec3, world: &HittableList) -> LightInfo;
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
                color: (self.color * self.intensity) / (self.position - origin).length(),
                direction,
            },
            Some(_) => LightInfo {
                color: Vec3::new(0.0, 0.0, 0.0),
                direction,
            },
        }
    }
}
