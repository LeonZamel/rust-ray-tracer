use crate::three_d_tree::Axis;
use rand::Rng;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn z() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn random() -> Vec3 {
        Vec3::new(
            rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
            rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
            rand::thread_rng().gen::<f64>() * 2.0 - 1.0,
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let vec = Vec3::random();
            if vec.length_squared() <= 1.0 {
                return vec;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }

    pub fn near_zero(&self) -> bool {
        let lambda = 1e-8;
        self.x.abs() < lambda && self.y.abs() < lambda && self.z.abs() < lambda
    }

    pub fn reflect(self, normal: Vec3) -> Vec3 {
        self - normal * 2.0 * self.dot(&normal)
    }

    pub fn refract(self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let temp = (-self).dot(&normal);
        let cos_theta = {
            if temp < 1.0 {
                temp
            } else {
                1.0
            }
        };
        let r_out_perp = (self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        return r_out_perp + r_out_parallel;
    }

    pub fn clamp(self, clamp: Vec3) -> Vec3 {
        Vec3::new(
            self.x.min(clamp.x),
            self.y.min(clamp.y),
            self.z.min(clamp.z),
        )
    }

    pub fn ln_1p(self) -> Vec3 {
        Vec3::new(self.x.ln_1p(), self.y.ln_1p(), self.z.ln_1p())
    }

    pub fn cross(self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            -(self.x * other.z - self.z * other.x),
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn get_axis(&self, axis: &Axis) -> f64 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
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
