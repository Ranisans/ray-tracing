use fastrand;
use rayon::prelude::*;

use crate::libs::camera::Camera;
use crate::libs::color::ray_color;
use crate::libs::figures::figures::Figures;
use crate::libs::figures::hittable_list::HittableList;
use crate::libs::figures::sphere::Sphere;
use crate::libs::material::{Dielectric, Lambertian, Material, Metal};
use crate::libs::vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: usize = 400;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 50;
const MAX_BIT_VALUE: f64 = 256.0;
const MIN_RANGE_VALUE: f64 = 0.0;
const MAX_RANGE_VALUE: f64 = 0.999;

pub struct ImageGenerator {
    image_width: usize,
    image_height: usize,
}

impl Default for ImageGenerator {
    fn default() -> Self {
        let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
        ImageGenerator {
            image_height,
            image_width: IMAGE_WIDTH,
        }
    }
}

impl ImageGenerator {
    pub fn generate(&self) {
        let look_from = Vec3::new(3.0, 3.0, 2.0);
        let look_at = Vec3::new(0.0, 0.0, -1.0);
        let v_up = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = (look_from - look_at).length();
        let aperture = 2.0;

        let camera = Camera::new(
            look_from,
            look_at,
            v_up,
            20.0,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
        );

        let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
        let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
        let material_left = Dielectric::new(1.5);
        let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);

        let mut hittable_list = HittableList::new(None);
        hittable_list.add(Figures::Sphere(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            Material::Lambertial(material_ground),
        )));
        hittable_list.add(Figures::Sphere(Sphere::new(
            Vec3::new(0.0, 0.0, -1.0),
            0.5,
            Material::Lambertial(material_center),
        )));
        hittable_list.add(Figures::Sphere(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            0.5,
            Material::Dielectric(material_left.clone()),
        )));
        hittable_list.add(Figures::Sphere(Sphere::new(
            Vec3::new(-1.0, 0.0, -1.0),
            -0.45,
            Material::Dielectric(material_left),
        )));
        hittable_list.add(Figures::Sphere(Sphere::new(
            Vec3::new(1.0, 0.0, -1.0),
            0.5,
            Material::Metal(material_right),
        )));

        let mut pixels = vec![0; self.image_height * self.image_width * 3];

        let parts: Vec<(usize, &mut [u8])> = pixels.chunks_mut(3).enumerate().collect();

        parts.into_par_iter().for_each(|(i, pixel)| {
            let mut pixel_color = Vec3::null();
            let x = i % self.image_width;
            let y = self.image_height - i / self.image_width;

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + fastrand::f64()) / (self.image_width as f64 - 1.0);
                let v = (y as f64 + fastrand::f64()) / (self.image_height as f64 - 1.0);

                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &hittable_list, MAX_DEPTH);
            }

            let color = vec3_to_array(&pixel_color);
            pixel[0] = color[0];
            pixel[1] = color[1];
            pixel[2] = color[2];
        });

        self.write_image(&pixels)
    }

    fn write_image(&self, pixels: &[u8]) {
        image::save_buffer(
            "output.png",
            pixels,
            self.image_width as u32,
            self.image_height as u32,
            image::ColorType::Rgb8,
        )
        .unwrap();
    }
}

fn vec3_to_array(pixel_color: &Vec3) -> [u8; 3] {
    let scale = 1.0 / SAMPLES_PER_PIXEL as f64;

    let r = (MAX_BIT_VALUE * clamp(pixel_color.x() * scale).sqrt()) as u8;
    let g = (MAX_BIT_VALUE * clamp(pixel_color.y() * scale).sqrt()) as u8;
    let b = (MAX_BIT_VALUE * clamp(pixel_color.z() * scale).sqrt()) as u8;

    [r, g, b]
}

fn clamp(x: f64) -> f64 {
    x.clamp(MIN_RANGE_VALUE, MAX_RANGE_VALUE)
}
