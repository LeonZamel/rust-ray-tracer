use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Hit {
    pub p: Vec3,
    pub normal: Vec3, // Always points opposite to hit ray
    pub t: f64,
    pub front_face: bool, // If the face that was hit was the front, i.e. outward face
}

impl Hit {
    pub fn new(p: Vec3, outward_normal: Vec3, t: f64, ray: &Ray) -> Hit {
        let (front_face, normal) = Hit::to_face_normal(ray, outward_normal);
        Hit {
            p,
            normal,
            t,
            front_face,
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

pub fn hit_list<T: Hittable>(
    hittables: Vec<T>,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<(T, Hit)> {
    let mut closest: Option<(T, Hit)> = None;
    let mut closest_dist = t_max;
    for obj in hittables {
        match obj.hit(ray, t_min, closest_dist) {
            None => continue,
            Some(hit) => {
                closest_dist = hit.t;
                closest = Some((obj, hit));
            }
        }
    }
    closest
}
