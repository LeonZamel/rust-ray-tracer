use crate::triangle::Triangle;
use crate::vec3::Vec3;

pub struct Polygon {
    pub vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
}
impl Polygon {
    fn new(vertices: Vec<Vec3>) -> Polygon {
        let mut triangles: Vec<Triangle> = Vec::new();
        // Turn n-polygon into n-2 triangles. Trivial, but only works for convex shapes
        for i in 1..vertices.len() - 2 {
            triangles.push(Triangle {
                p1: vertices[i],
                p2: vertices[i + 1],
                p3: vertices[i + 2],
            })
        }
        Polygon {
            vertices,
            triangles,
        }
    }
}
