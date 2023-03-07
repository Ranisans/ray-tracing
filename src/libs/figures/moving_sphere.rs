use crate::libs::figures::hit_record::HitRecord;
use crate::libs::figures::hittable::Hittable;
use crate::libs::material::Material;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub struct MovingSphere {
    center_0: Vec3,
    center_1: Vec3,
    time_0: f64,
    time_1: f64,
    radius: f64,
    material: Material,
}

impl MovingSphere {
    pub fn new(
        center_0: Vec3,
        center_1: Vec3,
        time_0: f64,
        time_1: f64,
        radius: f64,
        material: Material,
    ) -> Self {
        MovingSphere {
            center_0,
            center_1,
            time_0,
            time_1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center_0
            + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
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
        let normal = (p - self.center(ray.time())) / self.radius;
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
