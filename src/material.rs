use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn get_color(&self, ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3;
    // fn scatter(&self, hit: &Hit) -> Vec3; // TODO: Use this approach?
}
