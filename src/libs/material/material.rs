use crate::libs::figures::hit_record::HitRecord;
use crate::libs::material::{Dielectric, Lambertian, Metal};
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub trait OldMaterial {
    fn old_scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn clone_box(&self) -> Box<dyn OldMaterial>;
}

pub enum Material {
    Lambertial(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)>;
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Option<Ray>, Vec3)> {
        match self {
            Material::Lambertial(l) => l.scatter(ray_in, hit_record),
            Material::Dielectric(d) => d.scatter(ray_in, hit_record),
            Material::Metal(m) => m.scatter(ray_in, hit_record),
        }
    }
}
