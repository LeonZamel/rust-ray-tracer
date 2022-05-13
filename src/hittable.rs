use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Hit<'a> {
    pub p: Vec3,
    pub normal: Vec3, // Always points opposite to hit ray
    pub t: f64,
    pub front_face: bool, // If the face that was hit was the front, i.e. outward face
    pub material: &'a Box<dyn Material>,
}

impl<'a> Hit<'a> {
    pub fn new(
        p: Vec3,
        outward_normal: Vec3,
        t: f64,
        ray: &Ray,
        material: &'a Box<dyn Material>,
    ) -> Hit<'a> {
        let (front_face, normal) = Hit::to_face_normal(ray, outward_normal);
        Hit {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
    pub fn to_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = {
            if front_face {
                outward_normal
            } else {
                -outward_normal
            }
        };
        (front_face, normal)
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
