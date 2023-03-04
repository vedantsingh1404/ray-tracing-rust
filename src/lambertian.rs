use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(a: Vec3) -> Self {
        return Lambertian {albedo: a};
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, ray_out: &mut Ray) -> bool {
        let mut scatter_direction: Vec3 = rec.normal + Vec3::unit_vector(&Vec3::random_vec_in_unit_sphere());

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *ray_out = Ray::new(&rec.point, &(scatter_direction - rec.point));
        *attenuation = self.albedo;
        return true;
    }
}