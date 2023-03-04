use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: &f64, t_max: &f64, rec: &mut HitRecord) -> bool {
        return false;
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn insert_hittable<T: Hittable>(&mut self, hittable: &T) {
        self.objects.push(Box::new(hittable.clone()) as Box<dyn Hittable>);
    }
}
