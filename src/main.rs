pub mod libs;

use libs::ImageGenerator;
use std::time::Instant;

fn main() {
    let image_generator = ImageGenerator::default();

    let start = Instant::now();

    image_generator.generate();

    println!("Frame time: {}ms", start.elapsed().as_millis());
}
