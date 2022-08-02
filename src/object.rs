use crate::hittable::Hit;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;

pub struct Object {
    pub material: Box<dyn Material>,
    pub shape: Box<dyn Hittable>,
}
impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.shape.hit(ray, t_min, t_max)
    }
}
