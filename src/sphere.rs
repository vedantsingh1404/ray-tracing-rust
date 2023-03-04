use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Sphere {
    centre: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(c: Vec3, r: f64) -> Self {
        Sphere {
            centre: c,
            radius: r,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = r.origin() - self.centre;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&r.direction(), &oc) * 2.;
        let c = Vec3::dot(&oc, &oc) - (self.radius * self.radius);
        let discriminant = b * b - 4. * a * c;

        if discriminant < 0. {
            return false;
        }
        let sqrt_d: f64 = f64::sqrt(discriminant);

        let mut root: f64 = (-b - sqrt_d) / (2. * a);
        if root < *t_min || root > *t_max {
            root = (-b + sqrt_d) / (2. * a);
            if root < *t_min || root > *t_max {
                return false;
            }
        }

        let normal: Vec3 = Vec3::unit_vector(&(r.at(root) - self.centre));
        rec.point = r.at(root);
        rec.t = root;
        rec.normal = normal;
        return true;
    }
}
