use crate::vec3::Vec3;
use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::HitRecord;

pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(a: Vec3, f: f64) -> Self {
        let fuzz: f64;
        if f > 1. {
            fuzz = 1.;
        } else if f < 0. {
            fuzz = 0.;
        } else {
            fuzz = f;
        }
        
        return Metal {
            albedo: a,
            fuzz
        };
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, ray_out: &mut Ray) -> bool {
        let ray_in_d = Vec3::unit_vector(&ray_in.direction());
        let scatter_direction = Vec3::reflect(ray_in_d, (Vec3::unit_vector(&rec.normal)));

        *ray_out = Ray::new(&rec.point, &(scatter_direction + Vec3::random_vec_in_unit_sphere() * self.fuzz));
        *attenuation = self.albedo;
        return true;
    }
}