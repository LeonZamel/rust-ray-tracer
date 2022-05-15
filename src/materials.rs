use crate::hittable::Hit;
use crate::material::Material;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;
use rand::Rng;

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

pub struct Lambertian {
    pub albedo: Vec3,
}
impl Material for Lambertian {
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

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}
impl Material for Metal {
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

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}
impl Material for Dielectric {
    fn get_color(&self, ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_dir = ray.direction.unit_vector();
        let temp = (-unit_dir).dot(&hit.normal);
        let cos_theta = if temp < 1.0 { temp } else { 1.0 };
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio)
                > rand::thread_rng().gen::<f64>()
        {
            unit_dir.reflect(hit.normal)
        } else {
            unit_dir.refract(hit.normal, refraction_ratio)
        };
        ray_color(&Ray {
            origin: hit.p,
            direction: direction,
        })
    }
}
impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}
