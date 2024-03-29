extern crate rayon;

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

use hittable::Hit;
use rayon::prelude::*;

use rand::Rng;
use std::fs;
use std::path::Path;
use three_d_tree::build_tdtree;
use util::EPSILON;

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
const BASE_SAMPLES_PER_PIXEL: i32 = 30;
const DO_DYNAMIC_OVERSAMPLING: bool = true;
const MAX_DYNAMIC_OVERSAMPLING_FACTOR: i32 = 30;
const MAX_LIGHT_VAL: f64 = 2.0;
const COLOR_MEAN_UNCERTAINTY_THRESHOLD: f64 = 0.001;
const DO_POST_PROCESSING: bool = true;
const INTER_SIMILARITY_DIST_THRESHOLD: f64 = 2.0;
const SMOOTH_SIZE: i32 = 3;
const SMOOTHING_BASE_WEIGHT: f64 = 4.0;

static ASPECT_RATIO: f64 = 16.0 / 9.0;

static IMAGE_HEIGHT: usize = 1000;
static IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;

fn ray_color_per_light(
    ray: &Ray,
    world: &Scene,
    bounces_left: i32,
    dist_so_far: f64,
) -> (Vec<Vec3>, Option<Hit>) {
    // Function that gets the color for a given ray in the scene for every light source and passes back the first hit
    if bounces_left == 0 {
        return (world.lights.iter().map(|_| Vec3::z()).collect(), None);
    }
    // Calculate hit once, then get info for all lights
    let object_hit = world.objects.get_object_hit(ray);
    match object_hit {
        None => (
            world
                .lights
                .iter()
                .map(|light| light.no_hit(&ray, dist_so_far))
                .collect(),
            None,
        ),
        Some((obj, hit)) => (
            match obj.material.scatter(&ray, &hit) {
                None => world.lights.iter().map(|_| Vec3::z()).collect(),
                Some(next_ray) => {
                    ray_color_per_light(
                        &next_ray,
                        world,
                        bounces_left - 1,
                        dist_so_far + (hit.p - ray.origin).length(),
                    )
                    .0
                }
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
            Option::Some(hit),
        ),
    }
}

fn ray_color(ray: &Ray, world: &Scene, bounces_left: i32) -> (Vec3, Option<Hit>) {
    // Gets the color of a specific ray in the scene
    let (rays, hit) = ray_color_per_light(ray, world, bounces_left, 0.0);
    (
        rays.iter()
            .fold(Vec3::z(), |acc, x| acc + *x)
            // Fixes issues when objects become too bright
            .ln_1p()
            .clamp(Vec3::new(MAX_LIGHT_VAL, MAX_LIGHT_VAL, MAX_LIGHT_VAL)),
        hit,
    )
}

fn ray_from_image_pos(i: usize, j: usize, camera: &Camera) -> Ray {
    let horizontal_frac = (i as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
    let vertical_frac = (j as f64 + rand::thread_rng().gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
    camera.get_ray(horizontal_frac, vertical_frac)
}

fn main() {
    let camera = Camera::new_with_fov(Vec3::new(0.51, 0.2, 1.5), ASPECT_RATIO, 80.0);

    // Initialize the image buffer
    let mut image_buf: Vec<Vec<Vec3>> = vec![
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

    let mut image_normal: Vec<Vec<Option<Vec3>>> = vec![vec![None; IMAGE_WIDTH]; IMAGE_HEIGHT];

    let mut image_var: Vec<Vec<f64>> = vec![vec![0.0; IMAGE_WIDTH]; IMAGE_HEIGHT];

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

    let object_container = build_tdtree(&objects, 12);

    let scene: Scene = Scene {
        objects: &object_container,
        lights: &lights,
    };

    // Render
    image_buf
        .iter_mut()
        .zip(image_normal.iter_mut())
        .zip(image_var.iter_mut())
        .zip(0..IMAGE_HEIGHT)
        .par_bridge()
        .for_each(|(((row, normal_row), var_row), j)| {
            (0..IMAGE_WIDTH).for_each(|i| {
                let mut color_uncertain = true;
                let mut current_iteration = 1;
                let mut hits: Vec<Option<Hit>> = Vec::new();
                while (DO_DYNAMIC_OVERSAMPLING
                    && color_uncertain
                    && current_iteration <= MAX_DYNAMIC_OVERSAMPLING_FACTOR)
                    || current_iteration == 1
                {
                    let mut colors = Vec::new();
                    // Get the new color samples
                    for _ in 0..BASE_SAMPLES_PER_PIXEL {
                        let ray = ray_from_image_pos(i, j, &camera);
                        let (c, h_o) = ray_color(&ray, &scene, MAX_BOUNCES);
                        colors.push(c);
                        hits.push(h_o);
                    }
                    // Calculate color mean of current samples
                    let current_mean = colors.iter().fold(Vec3::z(), |acc, v| acc + *v)
                        * (1.0 / BASE_SAMPLES_PER_PIXEL as f64);
                    // Calculate mean of all iterations combined
                    let total_mean = (current_mean + row[i] * (current_iteration - 1) as f64)
                        * (1.0 / current_iteration as f64);
                    // Calculate the corrected sample standard deviation
                    let corrected_sample_std = colors
                        .iter()
                        .fold(0.0, |acc, v| acc + (total_mean - *v).length_squared())
                        / ((BASE_SAMPLES_PER_PIXEL - 1) as f64).sqrt();

                    var_row[i] = corrected_sample_std.max(EPSILON);

                    // Check value of the standard error of the mean, if it is high, get more samples
                    color_uncertain = corrected_sample_std
                        / ((current_iteration * BASE_SAMPLES_PER_PIXEL as i32) as f64).sqrt()
                        > COLOR_MEAN_UNCERTAINTY_THRESHOLD;

                    row[i] = total_mean;
                    current_iteration += 1;
                }

                // Calculate the inter similarity, i.e. for a pixel how similar all the rays are that contributed to it
                // The value will be true if all rays hit an object and the distance of the hits is lower than some threshold
                // This should also somehow take into account if the object is transparent, e.g. the dielectric to make sure
                // that we don't get blurry edges inside the object where another edge is refracted
                let sum_o = hits.iter().fold(Some(Vec3::z()), |sum_o_acc, h_o| {
                    sum_o_acc.and_then(|sum| h_o.as_ref().and_then(|h| Some(sum + h.normal)))
                });
                normal_row[i] = sum_o.and_then(|sum_vec| {
                    let avg_vec = sum_vec * (1.0 / hits.len() as f64);
                    if hits.iter().any(|hit| {
                        (hit.as_ref().unwrap().normal - avg_vec).length()
                            > INTER_SIMILARITY_DIST_THRESHOLD
                    }) {
                        None
                    } else {
                        Some(avg_vec)
                    }
                });
            });
        });

    let mut image: Vec<Vec<Vec3>> = vec![vec![Vec3::z(); IMAGE_WIDTH]; IMAGE_HEIGHT];

    // Post processing
    (0..IMAGE_HEIGHT)
        .into_iter()
        .zip(image.iter_mut())
        .par_bridge()
        .for_each(|(i, image_row)| {
            for j in 0..IMAGE_WIDTH {
                if DO_POST_PROCESSING {
                    // Apply smoothing by averaging (weighted) over squares of pixels of size SMOOTH_SIZE
                    // Will only average if pixels correspond to the same object in scene and are then scaled by normal vector similarity
                    let mut m = Vec3::z();
                    let mut w = 0.0;
                    let normal_avg_o = &image_normal[i][j];
                    image_row[j] = match normal_avg_o {
                        Some(normal_avg) => {
                            for k in 0..SMOOTH_SIZE {
                                for l in 0..SMOOTH_SIZE {
                                    let pos_h: i32 =
                                        i32::try_from(i).unwrap() + k - SMOOTH_SIZE / 2;
                                    let pos_w: i32 =
                                        i32::try_from(j).unwrap() + l - SMOOTH_SIZE / 2;
                                    if pos_h < 0
                                        || pos_h >= IMAGE_HEIGHT.try_into().unwrap()
                                        || pos_w < 0
                                        || pos_w >= IMAGE_WIDTH.try_into().unwrap()
                                    {
                                        continue;
                                    }
                                    let pw = if k == SMOOTH_SIZE / 2 && l == SMOOTH_SIZE / 2 {
                                        // Base weight gets scaled depending on the variance of the pixel
                                        // If we are super sure of the pixel color (variance close to 0) this will make sure it doesn't get blurred
                                        SMOOTHING_BASE_WEIGHT / &image_var[i][j]
                                    } else {
                                        match &image_normal[usize::try_from(pos_h).unwrap()]
                                            [usize::try_from(pos_w).unwrap()]
                                        {
                                            Some(normal_avg_other) => {
                                                normal_avg.dot(normal_avg_other).max(0.0)
                                            }
                                            None => 0.0,
                                        }
                                    };
                                    m += image_buf[usize::try_from(pos_h).unwrap()]
                                        [usize::try_from(pos_w).unwrap()]
                                        * pw;
                                    w += pw;
                                }
                            }
                            m / w
                        }
                        None => image_buf[i][j],
                    }
                } else {
                    image_row[j] = image_buf[i][j];
                }
            }
        });

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
