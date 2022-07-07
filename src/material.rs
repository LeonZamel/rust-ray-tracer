use crate::hittable::Hit;
use crate::light::LightInfo;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn get_color(
        &self,
        ray: &Ray,
        light_info: LightInfo,
        hit: &Hit,
        ray_color: &dyn Fn(&Ray) -> Vec3,
    ) -> Vec3;
}
