use crate::libs::material::Material;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn clone(&self) -> Self {
        HitRecord {
            p: self.p,
            normal: self.normal,
            material: self.material.clone_box(),
            t: self.t,
            front_face: self.front_face,
        }
    }
}
