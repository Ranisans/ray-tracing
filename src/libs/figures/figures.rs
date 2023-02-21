use crate::libs::figures::hit_record::HitRecord;
use crate::libs::figures::hittable::Hittable;
use crate::libs::figures::sphere::Sphere;
use crate::libs::ray::Ray;

pub enum Figures {
    Sphere(Sphere),
}

impl Hittable for Figures {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Figures::Sphere(s) => s.hit(ray, t_min, t_max),
        }
    }
}
