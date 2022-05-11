use rand::Rng;
use std::fs;
use std::ops;

const INFINITY: f64 = 999999.0;
const MAX_BOUNCES: i32 = 20;
const SAMPLES_PER_PIXEL: i32 = 5;

static ASPECT_RATIO: f64 = 16.0 / 9.0;

static IMAGE_HEIGHT: usize = 400;
static IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

#[derive(Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    fn random() -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
            rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
            rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
        )
    }

    fn random_in_unit_sphere() -> Vec3 {
        loop {
            let vec = Vec3::random();
            if vec.length_squared() <= 1.0 {
                return vec;
            }
        }
    }

    fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    fn near_zero(&self) -> bool {
        let lambda = 1e-8;
        self.x.abs() < lambda && self.y.abs() < lambda && self.z.abs() < lambda
    }

    fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal * 2.0 * self.dot(&normal)
    }
}
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, _rhs: Vec3) {
        *self = Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}
impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}
impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Copy, Clone)]
struct Ray {
    origin: Vec3,
    direction: Vec3,
}
impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

trait Material {
    fn get_color(&self, ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3;
    // fn scatter(&self, hit: &Hit) -> Vec3; // TODO: Use this approach?
}

struct Hit<'a> {
    p: Vec3,
    normal: Vec3, // Always points opposite to hit ray
    t: f64,
    front_face: bool, // If the face that was hit was the front, i.e. outward face
    material: &'a Box<dyn Material>,
}

impl<'a> Hit<'a> {
    fn new(
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
    fn to_face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
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

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;

        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        Some(Hit::new(p, outward_normal, t, ray, &self.material))
    }
}

fn normal_to_color(normal: &Vec3) -> Vec3 {
    Vec3::new(1.0 + normal.x, 1.0 + normal.y, 1.0 + normal.z) * 0.5
}

struct NormalMaterial;
impl Material for NormalMaterial {
    fn get_color(&self, ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        normal_to_color(&hit.normal)
    }
}

struct ConstantColorMaterial {
    color: Vec3,
}
impl Material for ConstantColorMaterial {
    fn get_color(&self, ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
        self.color
    }
}

struct LambertianMaterial {
    albedo: Vec3,
}
impl Material for LambertianMaterial {
    fn get_color(&self, ray: &Ray, hit: &Hit, ray_color: &dyn Fn(&Ray) -> Vec3) -> Vec3 {
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

struct Camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,

    // TODO: This should be hidden/private
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lower_left_viewport_corner: Vec3,
}
impl Camera {
    fn new(
        aspect_ratio: f64,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
    ) -> Camera {
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let lower_left_viewport_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            horizontal,
            vertical,
            origin,
            lower_left_viewport_corner,
        }
    }
    fn get_ray(&self, horizontal_frac: f64, vertical_frac: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_viewport_corner
                + self.horizontal * horizontal_frac
                + self.vertical * vertical_frac,
        }
    }
}

fn main() {
    let viewport_height: f64 = 2.0;
    let camera = Camera::new(
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
