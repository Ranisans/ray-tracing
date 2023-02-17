use crate::libs::figures::hit_record::HitRecord;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;

    fn clone_box(&self) -> Box<dyn Material>;
}
