use crate::libs::figures::hit_record::HitRecord;
use crate::libs::ray::Ray;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
