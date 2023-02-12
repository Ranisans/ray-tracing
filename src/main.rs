pub mod libs;

use libs::ImageGenerator;

fn main() {
    let image_generator = ImageGenerator::new(256, 266);

    image_generator.generate();
}
