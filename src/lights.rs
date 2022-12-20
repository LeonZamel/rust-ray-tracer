use crate::hittable::ObjectContainer;
use crate::light::Light;
use crate::light::LightInfo;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
    pub intensity: f64,
}
impl Light for PointLight {
    fn falloff(&self, dist: f64) -> f64 {
        1.0 / dist.powf(2.0)
    }

    fn at(&self, origin: Vec3, world: &dyn ObjectContainer, dist_so_far: f64) -> LightInfo {
        let direction = (self.position - origin).unit_vector();
        match world.get_object_hit(&Ray {
            direction,
            origin: origin,
        }) {
            None => LightInfo {
                color: (self.color * self.intensity)
                    * self.falloff((self.position - origin).length() + dist_so_far),
                direction,
            },
            Some(_) => LightInfo {
                color: Vec3::z(),
                direction,
            },
        }
    }

    fn no_hit(&self, ray: &Ray, dist_so_far: f64) -> Vec3 {
        // Gives the light a "body" which looks good in reflections
        let dist = ((ray.origin
            + ray.direction * (self.position - ray.origin).dot(&ray.direction))
            - self.position)
            .length();
        (((self.color * self.intensity) / (self.position - ray.origin).length().powf(2.0))
            / dist.powf(5.0))
            * (self.position - ray.origin)
                .unit_vector()
                .dot(&ray.direction)
    }
}
unsafe impl Sync for PointLight {}

pub struct AmbientLight {
    pub color_from_ray: Box<dyn Fn(&Ray) -> Vec3>,
}
impl Light for AmbientLight {
    fn falloff(&self, _dist: f64) -> f64 {
        1.0
    }

    fn at(&self, _origin: Vec3, _world: &dyn ObjectContainer, _dist_so_far: f64) -> LightInfo {
        LightInfo {
            color: Vec3::z(),
            direction: Vec3::z(),
        }
    }

    fn no_hit(&self, ray: &Ray, dist_so_far: f64) -> Vec3 {
        (self.color_from_ray)(ray)
    }
}
unsafe impl Sync for AmbientLight {}

pub fn sky_background(brightness: f64, ray: &Ray) -> Vec3 {
    let unit_dir = ray.direction.unit_vector();
    Vec3 {
        x: (1.0 - ((unit_dir.y + 1.0) / 4.0)) * brightness,
        y: (1.0 - ((unit_dir.y + 1.0) / 8.0)) * brightness,
        z: 1.0 * brightness,
    }
}
