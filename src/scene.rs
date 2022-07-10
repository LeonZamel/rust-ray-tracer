use crate::hittable::HittableList;
use crate::light::Light;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scene {
    pub objects: HittableList,
    pub lights: Vec<Box<dyn Light>>,
    pub background_fn: fn(&Ray) -> Vec3,
}
