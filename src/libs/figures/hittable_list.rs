use crate::libs::figures::figures::Figures;
use crate::libs::figures::hit_record::HitRecord;
use crate::libs::figures::hittable::Hittable;
use crate::libs::ray::Ray;

pub struct HittableList {
    pub objects: Vec<Figures>,
}

impl HittableList {
    pub fn new(item: Option<Figures>) -> Self {
        match item {
            None => HittableList { objects: vec![] },
            Some(value) => HittableList {
                objects: vec![value],
            },
        }
    }

    pub fn add(&mut self, item: Figures) {
        self.objects.push(item)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit)
            }
        }

        hit_record
    }
}
