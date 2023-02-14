use std::fs::File;
use std::io::Write;
use crate::libs::color::{ray_color, write_color};
use crate::libs::figures::hittable_list::HittableList;
use crate::libs::figures::sphere::Sphere;
use crate::libs::ray::Ray;
use crate::libs::vec3::Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

pub struct ImageGenerator {
    image_width: u32,
    image_height: u32,
}

impl Default for ImageGenerator {
    fn default() -> Self {
        let image_width = 400;
        let image_height = (image_width as f64 / ASPECT_RATIO) as u32;
        ImageGenerator{image_height, image_width}
    }
}

impl ImageGenerator {
    pub fn generate(&self) {
        let output = "output.ppm";

        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::null();
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let left_lower_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        let mut file = File::create(output).expect("Unable to create file");

        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.image_width, self.image_height)).expect("Can't create file");

        let mut hittable_list = HittableList::new(None);
        hittable_list.add(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
        hittable_list.add(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

        let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

        for y in (0..self.image_height).rev() {
            for x in 0..self.image_width {
                let u = x as f64 / (self.image_width as f64 - 1.0);
                let v = y as f64 / (self.image_height as f64 - 1.0);

                let ray = Ray::new(origin, left_lower_corner + u * horizontal + v * vertical - origin);

                let pixel_color = ray_color(&ray, &hittable_list);

                write_color(&mut file, pixel_color)
            }
        }
    }
}