use crate::libs::figures::hit_record::HitRecord;
use crate::libs::material::Material;
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

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&unit_vector(ray_in.direction()), &hit_record.normal);
        *scattered = Ray::new(hit_record.p, reflected + self.fuzz * random_unit_vector());
        *attenuation = self.albedo;
        scattered.direction().dot(&hit_record.normal) > 0.0
    }
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
