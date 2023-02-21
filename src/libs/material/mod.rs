mod dielectric;
mod lambertian;
mod material;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::{Material, Scatterable};
pub use metal::Metal;
