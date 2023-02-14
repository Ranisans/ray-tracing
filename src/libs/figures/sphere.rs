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

    pub fn hit(&self, ray: &Ray) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * oc.dot(ray.direction());
        let c = oc.dot(oc) - self.radius.powf(2.0);

        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}

