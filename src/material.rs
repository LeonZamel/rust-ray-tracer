use crate::hittable::Hit;
use crate::light::LightInfo;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn get_color(&self, ray: &Ray, light_info: LightInfo, hit: &Hit, next_ray_color: Vec3) -> Vec3;
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray>;
}
impl std::fmt::Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", "derp")
    }
}
