use std::fs::File;
use std::io::Write;
use crate::libs::figures::hit_record::HitRecord;
use crate::libs::figures::hittable::Hittable;
use crate::libs::figures::hittable_list::HittableList;
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

pub fn ray_color(ray: &Ray, hittable_list: &HittableList) -> Vec3 {
    let mut hit_record: HitRecord = HitRecord {
        p: Vec3::null(),
        normal: Vec3::null(),
        t: 0.0,
        front_face: false,
    };
    if hittable_list.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
        0.5 * (hit_record.normal + Vec3::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = unit_vector(ray.direction() - Vec3::new(0.0, 0.0, -1.0));
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}