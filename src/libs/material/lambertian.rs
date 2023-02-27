use crate::libs::figures::hit_record::HitRecord;
use crate::libs::material::Scatterable;
use crate::libs::ray::Ray;
use crate::libs::vec3::{random_unit_vector, Vec3};

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.p, scatter_direction, Some(ray_in.time()));
        let attenuation = self.albedo;

        Some((Some(scattered), attenuation))
    }
}
