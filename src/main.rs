mod camera;
mod hittable;
mod light;
mod lights;
mod material;
mod materials;
mod mesh;
mod object;
mod polygon;
mod ray;
mod scene;
mod sphere;
mod three_d_tree;
mod triangle;
mod util;
mod vec3;

use rand::Rng;
use std::fs;
use std::path::Path;
use three_d_tree::build_tdtree;

use camera::Camera;
use hittable::ObjectContainer;
use light::Light;
use lights::AmbientLight;
use lights::PointLight;
use mesh::Mesh;
use object::Object;
use ray::Ray;
use scene::Scene;
use sphere::Sphere;
use triangle::Triangle;
use vec3::Vec3;

const MAX_BOUNCES: i32 = 20;
const SAMPLES_PER_PIXEL: i32 = 50;
const MAX_LIGHT_VAL: f64 = 2.0;

static ASPECT_RATIO: f64 = 16.0 / 9.0;

static IMAGE_HEIGHT: usize = 800;
static IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

fn ray_color_per_light(ray: &Ray, world: &Scene, bounces_left: i32, dist_so_far: f64) -> Vec<Vec3> {
    // Function that gets the color for a given ray in the scene for every light source
    if bounces_left == 0 {
        return world.lights.iter().map(|_| Vec3::z()).collect();
    }
    let hit = &world.objects.get_object_hit(ray);
    match hit {
        None => world
            .lights
            .iter()
            .map(|light| light.no_hit(&ray, dist_so_far))
            .collect(),
        Some((obj, hit)) => match obj.material.scatter(&ray, &hit) {
            None => world.lights.iter().map(|_| Vec3::z()).collect(),
            Some(next_ray) => ray_color_per_light(
                &next_ray,
                world,
                bounces_left - 1,
                dist_so_far + (hit.p - ray.origin).length(),
            ),
        }
        .iter()
        .zip(world.lights.iter())
        .map(|(next_color, light)| {
            obj.material.get_color(
                &ray,
                light.at(hit.p, &world.objects, dist_so_far),
                &hit,
                *next_color,
            )
        })
        .collect(),
    }
}

fn ray_color(ray: &Ray, world: &Scene, bounces_left: i32) -> Vec3 {
    ray_color_per_light(ray, world, bounces_left, 0.0)
        .iter()
        .fold(Vec3::z(), |acc, x| acc + *x)
        // Fixes issues when objects become too bright
        .clamp(Vec3::new(MAX_LIGHT_VAL, MAX_LIGHT_VAL, MAX_LIGHT_VAL))
        .ln_1p()
}

fn main() {
    let camera = Camera::new_with_fov(Vec3::new(0.51, 0.2, 1.5), ASPECT_RATIO, 80.0);

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
    let mut objects = Vec::new();
    objects.push(Object::new(
        Box::new(materials::Lambertian {
            albedo: Vec3::new(0.2, 0.8, 0.2),
        }),
        Box::new(Sphere {
            center: Vec3::new(1.0, 0.0, -1.0),
            radius: 0.5,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Lambertian {
            albedo: Vec3::new(0.2, 0.2, 0.1),
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Metal {
            albedo: Vec3::new(0.8, 0.2, 0.2),
            fuzz: 0.02,
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.0, 0.0, -1.0),
            radius: 0.5,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Dielectric { ir: 1.5 }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Dielectric { ir: 1.5 }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: -0.45,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Dielectric { ir: 1.5 }),
        Box::new(Sphere {
            center: Vec3::new(2.0, 0.0, -1.0),
            radius: 0.5,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Lambertian {
            albedo: Vec3::new(0.2, 0.2, 1.0),
        }),
        Box::new(Triangle {
            p1: Vec3::new(-1.0, -0.5, 0.0),
            p2: Vec3::new(-1.0, 0.5, -0.5),
            p3: Vec3::new(-1.0, 0.5, 0.5),
        }),
    ));
    let bunny_mesh = Mesh::from_file(
        Path::new("data/LowPolyTree1.obj"),
        Vec3::new(2.0, -0.5, 0.0),
    )
    .unwrap();
    objects.push(Object::new(
        Box::new(materials::Lambertian {
            albedo: Vec3::new(0.1, 0.5, 0.1),
        }),
        Box::new(bunny_mesh),
    ));

    let mut lights: Vec<Box<dyn Light>> = Vec::new();
    lights.push(Box::new(PointLight {
        color: Vec3::new(1.0, 1.0, 1.0),
        position: Vec3::new(3.0, 2.0, 3.0),
        intensity: 20.0,
    }));

    lights.push(Box::new(AmbientLight {
        color_from_ray: Box::new(|ray| lights::sky_background(1.0, ray)),
    }));

    let scene: Scene = Scene {
        objects: build_tdtree(&objects),
        lights: lights,
    };

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            for _ in 0..SAMPLES_PER_PIXEL {
                let horizontal_frac =
                    (i as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let vertical_frac =
                    (j as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = camera.get_ray(horizontal_frac, vertical_frac);
                image[j][i] += ray_color(&ray, &scene, MAX_BOUNCES);
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
