use crate::hittable::{hit_list, Hittable};
use crate::polygon::Polygon;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::vec3::Vec3;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path;

pub struct Mesh {
    faces: Vec<Triangle>,
    offset: Vec3,
}
impl Mesh {
    pub fn from_file(path: &path::Path, offset: Vec3) -> Result<Mesh, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut faces = Vec::new();
        let mut vertices: Vec<Vec3> = Vec::new();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let mut parts = line.split_whitespace();
            let line_type = parts.next();
            match line_type {
                Some("v") => {
                    // Vertex
                    let parsed: Result<Vec<f64>, _> = parts.map(|s| s.parse::<f64>()).collect();
                    let vs: Vec<f64> = parsed?;
                    vertices.push(Vec3::new(vs[0], vs[1], vs[2]))
                }
                Some("f") => {
                    // Face
                    // Currently only works with triangles
                    let parsed: Result<Vec<usize>, _> = parts
                        .map(|s| s.split("/").next().unwrap().parse::<usize>())
                        .collect();
                    let vertex_indicies: Vec<usize> = parsed?;
                    faces.push(Triangle {
                        p1: vertices[vertex_indicies[0] - 1],
                        p2: vertices[vertex_indicies[1] - 1],
                        p3: vertices[vertex_indicies[2] - 1],
                    })
                }
                Some("#") => continue, // Comment
                Some(_) => continue,   // Anything else, unhandled
                None => continue,      // Emppty line
            }
        }
        Ok(Mesh { faces, offset })
    }
}
impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<crate::hittable::Hit> {
        hit_list(
            &self.faces,
            &Ray {
                // Instead of moving the mesh, we just move the ray in the opposite direction
                origin: ray.origin - self.offset,
                direction: ray.direction,
            },
            t_min,
            t_max,
        )
        .map(|(_, h)| h)
    }
}
