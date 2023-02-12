use std::fs::File;
use std::io::Write;
use crate::libs::vec3::Vec3;

const MAX_VALUE: f64 = 255.99;

pub fn write_color(file: &mut File, pixel_color: Vec3) {
    let r: u32 = (MAX_VALUE * pixel_color.x()) as u32;
    let g: u32 = (MAX_VALUE * pixel_color.y()) as u32;
    let b: u32 = (MAX_VALUE * pixel_color.z()) as u32;

    file.write_fmt(format_args!("{} {} {}\n", r, g, b)).expect("Cant write to file");
}