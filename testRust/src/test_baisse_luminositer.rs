mod model;
mod convertion;
mod service;
use crate::service::image_service::run_main_logic;
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

    let dimmed_image = reduce_brightness(&image, -99);
    dimmed_image.save("ImageBrightness.jpg")?;
    println!("Image with reduced brightness");

    Ok(())
}

fn reduce_brightness(image: &DynamicImage, value: i32) -> DynamicImage {
    image.brighten(value)
}
