use crate::hittable::HittableList;
use crate::light::Light;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scene<'a, 'b> {
    pub objects: &'a HittableList,
    pub light: &'b dyn Light,
    pub background_fn: fn(&Ray) -> Vec3,
}
