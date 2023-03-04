use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::lambertian::Lambertian;
use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        return Self { objects: Vec::new() };
    }

    pub fn insert_hittable(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec: HitRecord = HitRecord {
            point: Vec3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            t: 0.,
            material: Rc::new(Lambertian::new(Vec3::new(0., 0., 0.)))
        };

        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = *t_max;

        for object in &self.objects {
            if object.hit(r, t_min, &closest_so_far, &mut temp_rec)  {
                hit_anything = true;
                closest_so_far = temp_rec.t;
            }
        }

        *rec = temp_rec;
        return hit_anything;
    }
}