mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use rand::Rng;
use std::fs;

use camera::Camera;
use hittable::Hit;
use hittable::Hittable;
use material::Material;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

const INFINITY: f64 = 999999.0;
const MAX_BOUNCES: i32 = 20;
const SAMPLES_PER_PIXEL: i32 = 100;

static ASPECT_RATIO: f64 = 16.0 / 9.0;

static IMAGE_HEIGHT: usize = 400;
static IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

fn normal_to_color(normal: &Vec3) -> Vec3 {
    Vec3::new(1.0 + normal.x, 1.0 + normal.y, 1.0 + normal.z) * 0.5
}

struct NormalMaterial;
impl Material for NormalMaterial {
    fn get_color(&self, _ray: &Ray, hit: &Hit, _ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        normal_to_color(&hit.normal)
    }
}

struct ConstantColorMaterial {
    color: Vec3,
}
impl Material for ConstantColorMaterial {
    fn get_color(&self, _ray: &Ray, _hit: &Hit, _ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        self.color
    }
}

struct LambertianMaterial {
    albedo: Vec3,
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
struct MetalMaterial {
    albedo: Vec3,
    fuzz: f64,
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

fn ray_color(ray: &Ray, world: &HittableList, bounces_left: i32) -> Vec3 {
    if bounces_left == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    match world.hit(ray, 0.001, INFINITY) {
        None => background(&ray),
        Some(hit) => hit.material.get_color(ray, &hit, &|ray: &Ray| -> Vec3 {
            ray_color(ray, world, bounces_left - 1)
        }),
    }
}

fn background(ray: &Ray) -> Vec3 {
    let unit_dir = ray.direction.unit_vector();
    Vec3 {
        x: 1.0 - ((unit_dir.y + 1.0) / 4.0),
        y: 1.0 - ((unit_dir.y + 1.0) / 8.0),
        z: 1.0,
    }
}

struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}
impl HittableList {
    fn new() -> HittableList {
        HittableList {
            hittables: Vec::new(),
        }
    }
    fn push(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}
impl HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut closest_hit: Option<Hit> = None;
        let mut closest_dist = t_max;
        for obj in &self.hittables {
            match obj.hit(ray, t_min, closest_dist) {
                None => continue,
                Some(hit) => {
                    closest_dist = hit.t;
                    closest_hit = Some(hit);
                }
            }
        }
        closest_hit
    }
}

fn main() {
    let viewport_height: f64 = 2.0;
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        ASPECT_RATIO,
        viewport_height,
        viewport_height * ASPECT_RATIO,
        1.0,
    );

    // Init
    let mut image: Vec<Vec<Vec3>> = vec![
        vec![
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            IMAGE_WIDTH
        ];
        IMAGE_HEIGHT
    ];

    // World
    let mut objects = HittableList::new();
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(LambertianMaterial {
            albedo: Vec3::new(0.2, 0.8, 0.2),
        }),
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Box::new(LambertianMaterial {
            albedo: Vec3::new(0.7, 0.7, 0.1),
        }),
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(MetalMaterial {
            albedo: Vec3::new(0.8, 0.2, 0.2),
            fuzz: 0.1,
        }),
    }));
    objects.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Box::new(MetalMaterial {
            albedo: Vec3::new(0.5, 0.2, 1.0),
            fuzz: 0.4,
        }),
    }));

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            for _ in 0..SAMPLES_PER_PIXEL {
                let horizontal_frac =
                    (i as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let vertical_frac =
                    (j as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(horizontal_frac, vertical_frac);
                image[j][i] += ray_color(&ray, &objects, MAX_BOUNCES);
            }
        }
    }

    // Write to file
    let mut data = "P3\n".to_string()
        + " "
        + &IMAGE_WIDTH.to_string()
        + " "
        + &IMAGE_HEIGHT.to_string()
        + "\n255\n";

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let c = image[j][i] / (SAMPLES_PER_PIXEL as f64);
            let r = c.x.sqrt();
            let g = c.y.sqrt();
            let b = c.z.sqrt();
            data += &(((r * 255.0) as i32).to_string()
                + " "
                + &((g * 255.0) as i32).to_string()
                + " "
                + &((b * 255.0) as i32).to_string()
                + "\n")
        }
    }

    fs::write("image.ppm", data).expect("ERROR: Couldn't write to file!");
}
