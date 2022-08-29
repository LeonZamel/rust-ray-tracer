use crate::hittable::BoundingBox;
use crate::hittable::Hit;
use crate::hittable::Hittable;
use crate::material::Material;
use crate::ray::Ray;
pub struct Object {
    pub material: Box<dyn Material>,
    pub shape: Box<dyn Hittable>,
}
impl Object {
    pub fn new(material: Box<dyn Material>, shape: Box<dyn Hittable>) -> Object {
        Object { material, shape }
    }
}
impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.shape.hit(ray, t_min, t_max)
    }

    fn get_bounds(&self) -> BoundingBox {
        self.shape.get_bounds()
    }
}
