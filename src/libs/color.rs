use crate::libs::figures::hittable::Hittable;
use crate::libs::figures::hittable_list::HittableList;
use crate::libs::material::Scatterable;
use crate::libs::ray::Ray;
use crate::libs::vec3::{unit_vector, Vec3};

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
}
