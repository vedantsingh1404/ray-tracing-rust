use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use rand::Rng;

use rt_in_one_weekend::ray::Ray;
use rt_in_one_weekend::vec3::Vec3;
use rt_in_one_weekend::hittable_list::HittableList;
use rt_in_one_weekend::sphere::Sphere;
use rt_in_one_weekend::hittable::{Hittable, HitRecord};
use rt_in_one_weekend::camera::Camera;

fn color_ray(r: &Ray, world: &impl Hittable) -> Vec3 {
    let mut rec: HitRecord = HitRecord {
        point: Vec3::new(0., 0., 0.),
        normal: Vec3::new(0., 0., 0.),
        t: 0.5
    };

    if world.hit(r, &0., &f64::INFINITY, &mut rec) {
        let normal: Vec3 = rec.normal;
        return (normal + 1.) * 0.5
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t: f64 = unit_direction.y();
    let color: Vec3 = Vec3::new(1., 1., 1.) * (t) + Vec3::new(0.5, 0.7, 1.) * (1. - t);
    return color;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let aspect_ratio: f64 = (16 as f64) / (9 as f64);
    let image_width: i32 = 800;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;
    let num_samples = (&args[2]).parse::<i32>().unwrap();
    let mut rng = rand::thread_rng();

    // World Creation
    let mut world: HittableList = HittableList::new();
    world.insert_hittable(Box::new(Sphere::new(Vec3::new(0., 0., -1.), 0.5)));
    world.insert_hittable(Box::new(Sphere::new(Vec3::new(0., -100.5, -1.), 100.)));

    let camera = Camera::new();

    let filename = &args[1];
    let filename_path = Path::new(filename);
    let f = File::create(filename_path)?;

    let mut writer = BufWriter::new(&f);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    writer.write(header.as_bytes()).unwrap();

    for y in (0..image_height).rev() {
        for x in 0..image_width {
            let mut c: Vec3 = Vec3::new(0., 0., 0.);
            for _i in 0..num_samples {
                let rand: f64 = rng.gen_range(0.0..1.0);
                let u: f64 = ((x as f64) + rand) / ((image_width - 1) as f64);
                let v: f64 = ((y as f64) + rand) / ((image_height - 1) as f64);
                let ray: Ray = camera.get_ray(u, v);
                let c_: Vec3 = color_ray(&ray, &world);
                c = c + c_;
            }

            color(&mut writer, &c, num_samples);
        }
    }
    Ok(())
}

fn color<W: Write>(writer: &mut std::io::BufWriter<W>, color: &Vec3, num_samples: i32) {
    let mut r: f64 = color.x();
    let mut g: f64 = color.y();
    let mut b: f64 = color.z();

    let ratio: f64 = 1. / (num_samples as f64);

    r = r * ratio;
    g = g * ratio;
    b = b * ratio;

    let x_u8: u8 = (256. * clamp(r, 0., 0.999)) as u8;
    let y_u8: u8 = (256. * clamp(g, 0., 0.999)) as u8;
    let z_u8: u8 = (256. * clamp(b, 0., 0.999)) as u8;
    let line_to_write = format!("{} {} {}\n", x_u8, y_u8, z_u8);
    writer.write(line_to_write.as_bytes()).unwrap();
}

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    return value;
}
