use crate::ray::Ray;
use crate::hittable::{HitRecord};
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, ray_out: &mut Ray) -> bool;
}