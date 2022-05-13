use crate::hittable::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;

pub struct NormalMaterial;
impl Material for NormalMaterial {
    fn get_color(&self, _ray: &Ray, hit: &Hit, _ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        util::normal_to_color(&hit.normal)
    }
}

pub struct ConstantColorMaterial {
    pub color: Vec3,
}
impl Material for ConstantColorMaterial {
    fn get_color(&self, _ray: &Ray, _hit: &Hit, _ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        self.color
    }
}

pub struct LambertianMaterial {
    pub albedo: Vec3,
}
impl Material for LambertianMaterial {
    fn get_color(&self, _ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scatter_direction = {
            if scatter_direction.near_zero() {
                hit.normal
            } else {
                scatter_direction
            }
        };
        self.albedo
            * ray_color(&Ray {
                origin: hit.p,
                direction: scatter_direction,
            })
    }
}

pub struct MetalMaterial {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Material for MetalMaterial {
    fn get_color(&self, ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        let reflected = ray.direction.unit_vector().reflect(hit.normal)
            + Vec3::random_in_unit_sphere() * self.fuzz;
        self.albedo
            * ray_color(&Ray {
                origin: hit.p,
                direction: reflected,
            })
    }
}
