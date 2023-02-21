use crate::libs::material::Material;
use crate::libs::vec3::Vec3;

pub struct HitRecord<'material> {
    pub p: Vec3,
    pub normal: Vec3,
    pub material: &'material Material,
    pub t: f64,
    pub front_face: bool,
}
