use std::ops::{Add, Div, Mul, Neg, Sub};
use crate::random::random;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn unit_vector(vec: &Vec3) -> Vec3 {
        let length = vec.length();
        Vec3 {
            x: vec.x() / length,
            y: vec.y() / length,
            z: vec.z() / length,
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(vec1: &Vec3, vec2: &Vec3) -> f64 {
        vec1.x * vec2.x + vec1.y * vec2.y + vec1.z * vec2.z
    }

    pub fn distance(&self, vec: &Vec3) -> f64 {
        let diff_vec = Vec3 {
            x: vec.x() - self.x,
            y: vec.y() - self.y,
            z: vec.z() - self.z,
        };

        return diff_vec.length_squared();
    }

    pub fn cross(vec1: &Vec3, vec2: &Vec3) -> Self {
        Vec3 {
            x: vec1.y * vec2.z - vec1.z * vec2.y,
            y: vec1.z * vec2.x - vec1.x * vec2.z,
            z: vec1.x * vec2.y - vec1.y * vec2.x,
        }
    }

    pub fn random_vec(min: &f64, max: &f64) -> Self {
        Vec3 {
            x: random(min, max),
            y: random(min, max),
            z: random(min, max)
        }
    }

    pub fn random_vec_in_unit_sphere() -> Self {
        loop {
            let rand_vec = Vec3::random_vec(&-1., &1.);
            if rand_vec.distance(&Vec3::new(0., 0., 0.)) < 1. {
                return rand_vec;
            }
        }
    }

    pub fn random_vec_in_same_hemisphere(vec: &Vec3) -> Self {
        let rand_vec = Vec3::random_vec_in_unit_sphere();
        if Vec3::dot(&rand_vec, vec) > 0. {
            return rand_vec;
        } else {
            return -rand_vec;
        }
    }

    pub fn near_zero(&self) -> bool {
        let EPSILON: f64 = 0.00001;
        return self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON;
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Self {
        return v - n * 2. * (Vec3::dot(&v, &n));
    }

    pub fn refract(v: Vec3, n: Vec3, k: f64) -> Self {
        let unit_v = Vec3::unit_vector(&v);
        let cos = -Vec3::dot(&unit_v, &n);

        let r_perp = (unit_v + n * cos) * k;
        let r_parr = -n * (-r_perp.length_squared() + 1.).sqrt();

        return r_perp + r_parr;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x - rhs,
            y: self.y - rhs,
            z: self.z - rhs,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
