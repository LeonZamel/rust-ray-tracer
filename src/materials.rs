use crate::hittable::Hit;
use crate::light::LightInfo;
use crate::material::Material;
use crate::ray::Ray;
use crate::util;
use crate::vec3::Vec3;
use rand::Rng;

pub struct NormalMaterial;
impl Material for NormalMaterial {
    fn get_color(
        &self,
        _ray: &Ray,
        _light_info: LightInfo,
        hit: &Hit,
        _next_ray_color: Vec3,
    ) -> Vec3 {
        util::normal_to_color(&hit.normal)
    }

    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<Ray> {
        Option::None
    }
}

pub struct ConstantColorMaterial {
    pub color: Vec3,
}
impl Material for ConstantColorMaterial {
    fn get_color(
        &self,
        _ray: &Ray,
        _light_info: LightInfo,
        _hit: &Hit,
        _next_ray_color: Vec3,
    ) -> Vec3 {
        self.color
    }

    fn scatter(&self, _ray: &Ray, _hit: &Hit) -> Option<Ray> {
        Option::None
    }
}

pub struct Lambertian {
    pub albedo: Vec3,
}
impl Material for Lambertian {
    fn get_color(
        &self,
        _ray: &Ray,
        light_info: LightInfo,
        hit: &Hit,
        next_ray_color: Vec3,
    ) -> Vec3 {
        self.albedo * light_info.color * light_info.direction.dot(&hit.normal).max(0.0)
            + self.albedo * next_ray_color
    }

    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Ray> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scatter_direction = {
            if scatter_direction.near_zero() {
                hit.normal
            } else {
                scatter_direction
            }
        };
        Option::from(Ray {
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
    fn get_color(
        &self,
        _ray: &Ray,
        light_info: LightInfo,
        hit: &Hit,
        next_ray_color: Vec3,
    ) -> Vec3 {
        self.albedo * light_info.color * light_info.direction.dot(&hit.normal).max(0.0)
            + self.albedo * next_ray_color
    }

    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> {
        let reflected = ray.direction.unit_vector().reflect(hit.normal)
            + Vec3::random_in_unit_sphere() * self.fuzz;
        Option::from(Ray {
            origin: hit.p,
            direction: reflected,
        })
    }
}

pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}
impl Material for Dielectric {
    fn get_color(
        &self,
        _ray: &Ray,
        _light_info: LightInfo,
        _hit: &Hit,
        next_ray_color: Vec3,
    ) -> Vec3 {
        next_ray_color
    }

    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Ray> {
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
        let reflecting = cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio)
                > rand::thread_rng().gen::<f64>();
        let direction = if reflecting {
            unit_dir.reflect(hit.normal)
        } else {
            unit_dir.refract(hit.normal, refraction_ratio)
        };
        Option::from(Ray {
            origin: hit.p,
            direction,
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
