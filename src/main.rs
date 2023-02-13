pub mod libs;

use libs::ImageGenerator;

fn main() {
    let image_generator = ImageGenerator::default();

    image_generator.generate();
}
