use crate::libs::figures::hit_record::HitRecord;
use crate::libs::figures::hittable::Hittable;
use crate::libs::material::Lambertian;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(item: Option<Box<dyn Hittable>>) -> Self {
        match item {
            None => HittableList { objects: vec![] },
            Some(value) => HittableList {
                objects: vec![value],
            },
        }
    }

    pub fn add(&mut self, item: Box<dyn Hittable>) {
        self.objects.push(item)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_records: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: Vec3::null(),
            normal: Vec3::null(),
            t: 0.0,
            front_face: false,
            material: Box::new(Lambertian::new(Vec3::null())),
        };

        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for item in &self.objects {
            if item.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *hit_records = temp_rec.clone();
            }
        }

        hit_anything
    }
}
