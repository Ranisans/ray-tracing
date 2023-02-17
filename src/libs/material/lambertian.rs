use crate::libs::figures::hit_record::HitRecord;
use crate::libs::material::Material;
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

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered = Ray::new(hit_record.p, scatter_direction);
        *attenuation = self.albedo;

        true
    }

    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
