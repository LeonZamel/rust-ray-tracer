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
