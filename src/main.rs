use std::fs;
use std::ops;

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

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

fn main() {
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

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            image[j][i] = Vec3 {
                x: 1.0 * j as f64,
                y: 1.0 * i as f64,
                z: 0.5 * 255.0,
            }
        }
    }

    let mut data = "P3\n".to_string()
        + " "
        + &IMAGE_WIDTH.to_string()
        + " "
        + &IMAGE_HEIGHT.to_string()
        + "\n255\n";

    for row in image {
        for vec in row {
            data += &((vec.x as i32).to_string()
                + " "
                + &(vec.y as i32).to_string()
                + " "
                + &(vec.z as i32).to_string()
                + "\n")
        }
    }

    fs::write("image.ppm", data).expect("ERROR: Couldn't write to file!");
}
