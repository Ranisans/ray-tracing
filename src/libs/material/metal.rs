use crate::libs::figures::hit_record::HitRecord;
use crate::libs::material::Scatterable;
use crate::libs::ray::Ray;
use crate::libs::vec3::{random_unit_vector, reflect, unit_vector, Vec3};

#[derive(Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f64) -> Self {
        let fuzz = if f < 1.0 { f } else { 1.0 };

        Metal { albedo, fuzz }
    }
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let reflected = reflect(&unit_vector(ray_in.direction()), &hit_record.normal);
        let scattered = Ray::new(
            hit_record.p,
            reflected + self.fuzz * random_unit_vector(),
            Some(ray_in.time()),
        );
        let attenuation = self.albedo;
        if scattered.direction().dot(&hit_record.normal) > 0.0 {
            Some((Some(scattered), attenuation))
        } else {
            None
        }
    }
}
