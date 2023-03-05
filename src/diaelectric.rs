use crate::material::Material;
use crate::vec3::Vec3;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Diaelectric {
    ir: f64
}

impl Diaelectric {
    pub fn new(ir: f64) -> Self {
        return Diaelectric { ir };
    }
}

impl Material for Diaelectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, ray_out: &mut Ray) -> bool {
        let ray_in_d = ray_in.direction();
        let front_face: bool = Vec3::dot(&ray_in_d, &rec.normal) < 0.;
        let k = if front_face == true { 1. / self.ir } else { self.ir };
        let normal = if front_face == true { Vec3::unit_vector(&rec.normal) } else { -Vec3::unit_vector(&rec.normal) };

        let unit_v = Vec3::unit_vector(&ray_in_d);
        let cos = Vec3::dot(&unit_v, &normal);
        let sin = (1. - (cos * cos)).sqrt();

        let cannot_refract: bool = (k * sin) > 1.;

        let scatter_direction: Vec3;

        if cannot_refract {
            scatter_direction = Vec3::reflect(&Vec3::unit_vector(&ray_in_d), &normal);
        } else {
            scatter_direction = Vec3::refract(&Vec3::unit_vector(&ray_in_d), &normal, k);
        }

        *ray_out = Ray::new(&rec.point, &(scatter_direction));
        *attenuation = Vec3::new(1., 1., 1.);
        return true;
    }
}