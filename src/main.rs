use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use rt_in_one_weekend::ray::Ray;
use rt_in_one_weekend::vec3::Vec3;

fn hit_sphere(centre: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = ray.origin() - *centre;
    let a = Vec3::dot(&ray.direction(), &ray.direction());
    let b = Vec3::dot(&ray.direction(), &oc) * 2.;
    let c = Vec3::dot(&oc, &oc) - (radius * radius);
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        return -1.;
    } else {
        return ((-b) - f64::sqrt(discriminant)) / (2. * a);
    }
}

fn color_ray(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0., 0., -1.), 0.5, r);

    if t > 0. {
        let normal: Vec3 = Vec3::unit_vector(&(r.at(t) - Vec3::new(0., 0., -1.)));
        return (normal + 1.) * 0.5;
    }
    let unit_direction: Vec3 = Vec3::unit_vector(&r.direction());
    let t: f64 = unit_direction.y();
    let color: Vec3 = Vec3::new(1., 1., 1.) * (t) + Vec3::new(0.5, 0.7, 1.) * (1. - t);
    return color;
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let aspect_ratio: f64 = (16 as f64) / (9 as f64);
    let image_width: i32 = 400;
    let image_height: i32 = ((image_width as f64) / aspect_ratio) as i32;

    // Camera details

    let viewport_h: i32 = 2;
    let viewport_w: i32 = ((viewport_h as f64) * aspect_ratio) as i32;
    let focal_length: i32 = 1;

    let origin: Vec3 = Vec3::new(0., 0., 0.);
    let horizontal: Vec3 = Vec3::new(viewport_w as f64, 0., 0.);
    let vertical: Vec3 = Vec3::new(0., viewport_h as f64, 0.);

    let lower_left_corner: Vec3 = Vec3::new(
        -viewport_w as f64 / 2.,
        -viewport_h as f64 / 2.,
        -focal_length as f64,
    );

    let filename = &args[1];
    let filename_path = Path::new(filename);
    let f = File::create(filename_path)?;

    let mut writer = BufWriter::new(&f);

    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
    writer.write(header.as_bytes()).unwrap();

    for y in 0..image_height {
        for x in 0..image_width {
            let u: f64 = (x as f64) / ((image_width - 1) as f64);
            let v: f64 = (y as f64) / ((image_height - 1) as f64);

            let ray: Ray = Ray::new(
                &origin,
                &(lower_left_corner + horizontal * u + vertical * v - origin),
            );
            let c: Vec3 = color_ray(&ray);
            color(&mut writer, &c);
        }
    }
    Ok(())
}

fn color<W: Write>(writer: &mut std::io::BufWriter<W>, color: &Vec3) {
    let x_u8: u8 = (color.x() * 255.99) as u8;
    let y_u8: u8 = (color.y() * 255.99) as u8;
    let z_u8: u8 = (color.z() * 255.99) as u8;
    let line_to_write = format!("{} {} {}\n", x_u8, y_u8, z_u8);
    writer.write(line_to_write.as_bytes()).unwrap();
}
