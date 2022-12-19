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

const MAX_BOUNCES: i32 = 10;
const BASE_SAMPLES_PER_PIXEL: usize = 5;
const MAX_DYNAMIC_OVERSAMPLING_FACTOR: i32 = 10;
const MAX_LIGHT_VAL: f64 = 2.0;

static ASPECT_RATIO: f64 = 16.0 / 9.0;

static IMAGE_HEIGHT: usize = 400;
static IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

fn ray_color_per_light(ray: &Ray, world: &Scene, bounces_left: i32, dist_so_far: f64) -> Vec<Vec3> {
    // Function that gets the color for a given ray in the scene for every light source
    if bounces_left == 0 {
        return world.lights.iter().map(|_| Vec3::z()).collect();
    }
    // Calculate hit once, then get info for all lights
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
                light.at(hit.p, world.objects, dist_so_far),
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

fn ray_from_image_pos(i: usize, j: usize, camera: &Camera) -> Ray {
    let horizontal_frac = (i as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
    let vertical_frac = (j as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
    camera.get_ray(horizontal_frac, vertical_frac)
}

fn vec_mean(vecs: &[Vec3]) -> Vec3 {
    vecs.into_iter().fold(Vec3::z(), |acc, v| acc + *v) * (1.0 / vecs.len() as f64)
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
            radius: 0.4,
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
            radius: 0.4,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Dielectric { ir: 1.5 }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.4,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Dielectric { ir: 1.5 }),
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: -0.35,
        }),
    ));
    objects.push(Object::new(
        Box::new(materials::Dielectric { ir: 1.5 }),
        Box::new(Sphere {
            center: Vec3::new(2.0, 0.0, -1.0),
            radius: 0.4,
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
    let tree_mesh = Mesh::from_file(
        Path::new("data/LowPolyTree1.obj"),
        Vec3::new(2.0, -0.49, 0.1),
    )
    .unwrap();
    objects.push(Object::new(
        Box::new(materials::Lambertian {
            albedo: Vec3::new(0.1, 0.5, 0.1),
        }),
        Box::new(tree_mesh),
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

    let object_container = build_tdtree(&objects, 6);

    let scene: Scene = Scene {
        objects: &object_container,
        lights: lights,
    };

    // Render
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let mut color_uncertain = true;
            let mut current_iteration = 1;
            while color_uncertain && current_iteration <= MAX_DYNAMIC_OVERSAMPLING_FACTOR {
                let mut colors = Vec::new();
                for k in 0..BASE_SAMPLES_PER_PIXEL {
                    let ray = ray_from_image_pos(i, j, &camera);
                    let c = ray_color(&ray, &scene, MAX_BOUNCES);
                    colors.push(c);
                }
                let m = vec_mean(&colors);
                let current_mean = (m + image[j][i] * (current_iteration - 1) as f64)
                    * (1.0 / current_iteration as f64);
                let corrected_sample_std = (colors
                    .iter()
                    .fold(0.0, |acc, v| acc + (current_mean - *v).length_squared())
                    / (colors.len() - 1) as f64)
                    .sqrt();

                // Check value of the standard error of the mean
                if corrected_sample_std
                    / ((current_iteration * BASE_SAMPLES_PER_PIXEL as i32) as f64).sqrt()
                    <= 0.001
                {
                    color_uncertain = false
                }

                image[j][i] = current_mean;
                current_iteration += 1;
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
            let c = image[j][i];
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
