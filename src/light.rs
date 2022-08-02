use crate::object::Object;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Light {
    fn at(&self, origin: Vec3, world: &Vec<Object>, dist_so_far: f64) -> LightInfo;
    fn falloff(&self, dist: f64) -> f64;
    fn no_hit(&self, ray: &Ray, dist_so_far: f64) -> Vec3;
}

pub struct LightInfo {
    pub color: Vec3,
    pub direction: Vec3, // Direction of the light from the hit
}
