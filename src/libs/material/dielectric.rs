use crate::libs::figures::hit_record::HitRecord;
use crate::libs::material::Scatterable;
use crate::libs::ray::Ray;
use crate::libs::vec3::{reflect, refract, unit_vector, Vec3};

#[derive(Clone)]
pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Dielectric { ir }
    }

    fn reflect_once(&self, cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0.powf(2.0);
        r0 + (1.0 + r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ration = if hit_record.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = unit_vector(ray_in.direction());

        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powf(2.0)).sqrt();

        let cannot_refract = refraction_ration * sin_theta > 1.0;

        let direction = if cannot_refract
            || self.reflect_once(cos_theta, refraction_ration) > fastrand::f64()
        {
            reflect(&unit_direction, &hit_record.normal)
        } else {
            refract(&unit_direction, &hit_record.normal, refraction_ration)
        };

        let scattered = Ray::new(hit_record.p, direction, Some(ray_in.time()));

        Some((Some(scattered), attenuation))
    }
}
