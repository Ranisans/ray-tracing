use crate::libs::figures::hit_record::HitRecord;
use crate::libs::figures::hittable::Hittable;
use crate::libs::material::Material;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.length_squared() - self.radius.powf(2.0);

        let discriminant = half_b.powf(2.0) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let front_face = ray.direction().dot(&normal) < 0.0;

        Some(HitRecord {
            t: root,
            p,
            material: &self.material,
            normal: if front_face { normal } else { -normal },
            front_face,
        })
    }
}
