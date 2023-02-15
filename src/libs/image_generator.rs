use std::fs::File;
use std::io::Write;
use fastrand;
use crate::libs::camera::Camera;
use crate::libs::color::{ray_color, write_color};
use crate::libs::figures::hittable_list::HittableList;
use crate::libs::figures::sphere::Sphere;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH:u32 = 400;
const VIEWPORT_HEIGHT:f64 = 2.0;
const FOCAL_LENGTH:f64 = 1.0;
const SAMPLES_PER_PIXEL: u32 = 100;

pub struct ImageGenerator {
    image_width: u32,
    image_height: u32,
}

impl Default for ImageGenerator {
    fn default() -> Self {
        let image_height = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
        ImageGenerator{image_height, image_width: IMAGE_WIDTH}
    }
}

impl ImageGenerator {
    pub fn generate(&self) {
        let output = "output.ppm";

        let viewport_width = ASPECT_RATIO * VIEWPORT_HEIGHT;

        let camera = Camera::new(viewport_width, VIEWPORT_HEIGHT, FOCAL_LENGTH);

        let mut file = File::create(output).expect("Unable to create file");

        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.image_width, self.image_height)).expect("Can't create file");

        let mut hittable_list = HittableList::new(None);
        hittable_list.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        hittable_list.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));


        for y in (0..self.image_height).rev() {
            for x in 0..self.image_width {
                let mut pixel_color = Vec3::null();

                for _ in 0..SAMPLES_PER_PIXEL {

                    let u = (x as f64 + fastrand::f64()) / (self.image_width as f64 - 1.0);
                    let v = (y as f64 + fastrand::f64()) / (self.image_height as f64 - 1.0);

                    let ray = camera.get_ray(u, v);

                    pixel_color += ray_color(&ray, &hittable_list);

                }

                write_color(&mut file, pixel_color, SAMPLES_PER_PIXEL);
            }
        }
    }
}