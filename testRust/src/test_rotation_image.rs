mod convertion;
mod service;
use crate::service::image_service::run_main_logic;
mod model;
use image::{DynamicImage};

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {:?}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let _image = run_main_logic("ppsteam.jpg");

    let image = image::open("ppsteam.jpg")?;
    println!("Image loaded: {}x{}", image.width(), image.height());


    let rotated_image = rotate_image(&image);
    rotated_image.save("image_rotated.jpg")?;
    println!("Image rotated and saved as 'image_rotated.jpg'");

    Ok(())
}

fn rotate_image(image: &DynamicImage) -> DynamicImage {
    image.rotate90()
}
