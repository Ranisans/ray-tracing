use fastrand;
use rayon::prelude::*;

use crate::libs::camera::Camera;
use crate::libs::color::ray_color;
use crate::libs::figures::figures::Figures;
use crate::libs::figures::hittable_list::HittableList;
use crate::libs::figures::moving_spheres::MovingSpheres;
use crate::libs::figures::sphere::Sphere;
use crate::libs::material::{Dielectric, Lambertian, Material, Metal};
use crate::libs::vec3::{random_between, Vec3};

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
        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let v_up = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;

        let camera = Camera::new(
            look_from,
            look_at,
            v_up,
            20.0,
            ASPECT_RATIO,
            aperture,
            dist_to_focus,
            Some(0.0),
            Some(1.0),
        );

        let hittable_list = random_scene();

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

fn random_scene() -> HittableList {
    let mut hittable_list = HittableList::new(None);

    let ground_material = Material::Lambertial(Lambertian::new(Vec3::new(0.5, 0.5, 0.5)));
    hittable_list.add(Figures::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let tmp_vector = Vec3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let chose_mat = fastrand::f64();
            let center = Vec3::new(
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            );

            if (center - tmp_vector).length() > 0.9 {
                if chose_mat < 0.8 {
                    // diffuse
                    let albedo = Vec3::simple_random() * Vec3::simple_random();
                    let material = Material::Lambertial(Lambertian::new(albedo));
                    let center_2 = center + Vec3::new(0.0, random_between(0.0, 0.5), 0.0);
                    hittable_list.add(Figures::MovingSpheres(MovingSpheres::new(
                        center, center_2, 0.0, 1.0, 0.2, material,
                    )));
                } else if chose_mat < 0.95 {
                    // metal
                    let albedo = Vec3::random(0.2, 1.0);
                    let fuzz = random_between(0.0, 0.5);
                    let material = Material::Metal(Metal::new(albedo, fuzz));
                    hittable_list.add(Figures::Sphere(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Material::Dielectric(Dielectric::new(1.5));
                    hittable_list.add(Figures::Sphere(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material = Material::Dielectric(Dielectric::new(1.5));
    hittable_list.add(Figures::Sphere(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Lambertial(Lambertian::new(Vec3::new(0.4, 0.2, 0.1)));
    hittable_list.add(Figures::Sphere(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    let material = Material::Metal(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0));
    hittable_list.add(Figures::Sphere(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        material,
    )));

    hittable_list
}
