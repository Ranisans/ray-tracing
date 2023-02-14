use crate::libs::figures::hit_record::HitRecord;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_records: &mut HitRecord) -> bool;
}