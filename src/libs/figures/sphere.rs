use crate::libs::figures::hit_record::HitRecord;
use crate::libs::figures::hittable::Hittable;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hittable_records: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius.powf(2.0);

        let discriminant = half_b.powf(2.0) - a * c;

        if discriminant < 0.0 {
            return false
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false
            }
        }

        hittable_records.t = root;
        hittable_records.p = ray.at(root);

        let outward_normal = (hittable_records.p - self.center) / self.radius;
        hittable_records.set_face_normal(ray, outward_normal);

        true
    }
}
