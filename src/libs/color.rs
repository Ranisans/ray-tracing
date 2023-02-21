use crate::libs::figures::hittable::Hittable;
use crate::libs::figures::hittable_list::HittableList;
use crate::libs::material::Scatterable;
use crate::libs::ray::Ray;
use crate::libs::vec3::{unit_vector, Vec3};
use std::fs::File;
use std::io::Write;

const MAX_BIT_VALUE: f64 = 256.0;
const MIN_RANGE_VALUE: f64 = 0.0;
const MAX_RANGE_VALUE: f64 = 0.999;

fn clamp(x: f64) -> f64 {
    x.clamp(MIN_RANGE_VALUE, MAX_RANGE_VALUE)
}

pub fn write_color(file: &mut File, pixel_color: Vec3, samples_per_pixel: u32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let r = (MAX_BIT_VALUE * clamp(pixel_color.x() * scale).sqrt()) as u32;
    let g = (MAX_BIT_VALUE * clamp(pixel_color.y() * scale).sqrt()) as u32;
    let b = (MAX_BIT_VALUE * clamp(pixel_color.z() * scale).sqrt()) as u32;

    file.write_fmt(format_args!("{} {} {}\n", r, g, b))
        .expect("Cant write to file");
}

pub fn ray_color(ray: &Ray, hittable_list: &HittableList, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Vec3::null();
    }

    let hit = hittable_list.hit(ray, 0.001, f64::INFINITY);

    match hit {
        Some(hit_record) => {
            let scattered = hit_record.material.scatter(ray, &hit_record);
            return match scattered {
                Some((Some(scattered), attenuation)) => {
                    attenuation * ray_color(&scattered, hittable_list, depth - 1)
                }
                _ => Vec3::null(),
            };
        }
        None => {
            let unit_direction = unit_vector(ray.direction() - Vec3::new(0.0, 0.0, -1.0));
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }

    // let mut hit_record: HitRecord = HitRecord {
    //     p: Vec3::null(),
    //     normal: Vec3::null(),
    //     material: Box::new(Lambertian::new(Vec3::null())),
    //     t: 0.0,
    //     front_face: false,
    // };
    // if hittable_list.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
    //     let mut scattered = Ray::new(Vec3::null(), Vec3::null());
    //     let mut attenuation = Vec3::null();
    //
    //     if hit_record
    //         .material
    //         .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
    //     {
    //         return attenuation * ray_color(&scattered, hittable_list, depth - 1);
    //     }
    //
    //     return Vec3::null();
    // } else {
    //     let unit_direction = unit_vector(ray.direction() - Vec3::new(0.0, 0.0, -1.0));
    //     let t = 0.5 * (unit_direction.y() + 1.0);
    //     (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    // }
}
