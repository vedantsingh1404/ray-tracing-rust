use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio: f64 = (16 as f64) / (9 as f64);
        let viewport_h: i32 = 2;
        let viewport_w: i32 = ((viewport_h as f64) * aspect_ratio) as i32;
        let focal_length: i32 = 1;

        let origin: Vec3 = Vec3::new(0., 0., 0.);
        let horizontal: Vec3 = Vec3::new(viewport_w as f64, 0., 0.);
        let vertical: Vec3 = Vec3::new(0., viewport_h as f64, 0.);
        let lower_left: Vec3 = Vec3::new(
            -viewport_w as f64 / 2.,
            -viewport_h as f64 / 2.,
            -focal_length as f64,
        );

        return Camera {
            origin,
            lower_left,
            horizontal,
            vertical
        };
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            &self.origin,
            &(self.lower_left + self.horizontal * u + self.vertical * v - self.origin),
        );
    }
}