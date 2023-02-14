use std::fs::File;
use std::io::Write;
use crate::libs::figures::sphere::Sphere;
use crate::libs::ray::Ray;
use crate::libs::vec3::{unit_vector, Vec3};

const MAX_VALUE: f64 = 255.99;


pub fn write_color(file: &mut File, pixel_color: Vec3) {
    let r: u32 = (MAX_VALUE * pixel_color.x()) as u32;
    let g: u32 = (MAX_VALUE * pixel_color.y()) as u32;
    let b: u32 = (MAX_VALUE * pixel_color.z()) as u32;

    file.write_fmt(format_args!("{} {} {}\n", r, g, b)).expect("Cant write to file");
}

pub fn ray_color(ray: &Ray, sphere: &Sphere) -> Vec3 {
    if sphere.hit(ray) {
        return Vec3::new(1.0, 0.0, 0.0)
    }
    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}