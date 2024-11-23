use chrono::{Utc};
use std::fs;
use std::path::Path;
mod convertion;
mod model;
use crate::model::image::Image;
use crate::convertion::schema::images;
use diesel::prelude::*;
use diesel::insert_into;
mod service;
use crate::service::image_service::establish_connection;
use crate::service::image_service::run_main_logic;

fn main() {
    if let Err(e) = run() {
        eprintln!("An error occurred: {:?}", e);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let _image = run_main_logic("ppsteam.jpg");

    let image_path = "ppsteam.jpg";

    if !Path::new(image_path).exists() {
        panic!("Le fichier image n'existe pas à l'emplacement spécifié !");
    }

    let file_content = match fs::read(image_path) {
        Ok(content) => content,
        Err(err) => panic!("Erreur lors de la lecture du fichier image : {:?}", err),
    };

    let mut connection = establish_connection();

    let image1 = Image {
        id: 2,
        filepath: image_path.to_string(),
        file_content: file_content.clone(),
        created_at: Utc::now().naive_utc(),
    };

    let image2 = Image {
        id: 3,
        filepath: image_path.to_string(),
        file_content: file_content.clone(),
        created_at: Utc::now().naive_utc() - chrono::Duration::days(2),
    };

    let image3 = Image {
        id: 4,
        filepath: image_path.to_string(),
        file_content: file_content.clone(),
        created_at: Utc::now().naive_utc() - chrono::Duration::days(5),
    };

    insert_into(images::table)
        .values(&[image1, image2, image3])
        .execute(&mut connection)
        .expect("Error inserting images into database");

    let all_images: Vec<Image> = images::table
        .order(images::created_at.desc())
        .load::<Image>(&mut connection)
        .expect("Error loading images");

    println!("All images in the database (sorted by date):");
    for image in &all_images {
        println!("ID: {}, Date: {}", image.id, image.created_at);
    }

    let current_date = Utc::now().naive_utc().date();
    let start_of_day = current_date.and_hms_opt(0, 0, 0).unwrap();
    let end_of_day = current_date.and_hms_opt(23, 59, 59).unwrap();

    let filtered_images: Vec<Image> = images::table
        .filter(images::created_at.ge(start_of_day))
        .filter(images::created_at.le(end_of_day))
        .load::<Image>(&mut connection)
        .expect("Error loading filtered images");

    println!("\nImages created on 16-11-2024:");
    for image in filtered_images {
        println!("ID: {}, Date: {}", image.id, image.created_at);
    }

    Ok(())
}
