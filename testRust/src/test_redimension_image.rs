mod model;
mod convertion;
mod service;
use crate::service::image_service::run_main_logic;
use image::{imageops::FilterType};

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {:?}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let _image = run_main_logic("ppsteam.jpg");

    let image = image::open("ppsteam.jpg")?;
    println!("Image before resizing: {}x{}", image.width(), image.height());

    let resized_image = image.resize_exact(1000, 1000, FilterType::Lanczos3);
    println!("Image after resizing: {}x{}", resized_image.width(), resized_image.height());

    resized_image.save("ppsteam_resized.jpg")?;
    Ok(())
}
