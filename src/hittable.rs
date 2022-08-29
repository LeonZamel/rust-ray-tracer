use crate::object::Object;
use crate::ray::Ray;
use crate::util;
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
#[derive(Clone)]
pub struct BoundingBox {
    pub x_low: f64,
    pub x_high: f64,
    pub y_low: f64,
    pub y_high: f64,
    pub z_low: f64,
    pub z_high: f64,
}
impl BoundingBox {
    pub fn new(
        x_low: f64,
        x_high: f64,
        y_low: f64,
        y_high: f64,
        z_low: f64,
        z_high: f64,
    ) -> BoundingBox {
        BoundingBox {
            x_low,
            x_high,
            y_low,
            y_high,
            z_low,
            z_high,
        }
    }
    pub fn lower(&self) -> Vec3 {
        Vec3::new(self.x_low, self.y_low, self.z_low)
    }
    pub fn higher(&self) -> Vec3 {
        Vec3::new(self.x_high, self.y_high, self.z_high)
    }
    pub fn encloses_point(&self, p: &Vec3) -> bool {
        self.x_low <= p.x
            && p.x <= self.x_high
            && self.y_low <= p.y
            && p.y <= self.y_high
            && self.z_low <= p.z
            && p.z <= self.z_high
    }
    pub fn encloses_bounds(&self, bounds: BoundingBox) -> bool {
        self.encloses_point(&bounds.lower()) && self.encloses_point(&bounds.higher())
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn get_bounds(&self) -> BoundingBox;
}
pub trait ObjectContainer {
    fn get_object_hit(&self, ray: &Ray) -> Option<(&Object, Hit)>;
}

pub fn hit_list<'a, T: Hittable>(
    hittables: &Vec<&'a T>,
    ray: &Ray,
    t_min: f64,
    t_max: f64,
) -> Option<(&'a T, Hit)> {
    // Super inefficient. Add segmentation structure like 3-D tree
    let mut closest: Option<(&'a T, Hit)> = None;
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

pub fn hit_list_default<'a, T: Hittable>(
    hittables: &Vec<&'a T>,
    ray: &Ray,
) -> Option<(&'a T, Hit)> {
    hit_list(hittables, ray, util::EPSILON, util::INFINITY)
}
