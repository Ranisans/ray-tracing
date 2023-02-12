use std::fs::File;
use std::io::Write;
use crate::libs::color::write_color;
use crate::libs::vec3::Vec3;


pub struct ImageGenerator {
    image_width: u32,
    image_height: u32,
}

impl ImageGenerator {
    pub fn new (w: u32, h: u32) -> Self {
        ImageGenerator{image_height: h, image_width: w}
    }

    pub fn generate(&self) {
        let output = "output.ppm";

        let mut file = File::create(output).expect("Unable to create file");

        file.write_fmt(format_args!("P3\n{} {}\n255\n", self.image_width, self.image_height)).expect("Can't create file");

        for y in (0..self.image_height).rev() {
            for x in 0..self.image_width {
                let r: f64 = x as f64 / (self.image_width as f64 - 1.0);
                let g: f64 = y as f64 / (self.image_height as f64 - 1.0);
                let b : f64 = 0.25;

                let color = Vec3::new(r, g, b);
                
                write_color(&mut file, color)
            }
        }
    }
}