use crate::model::image::Image;
use crate::convertion::schema::images;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::sql_query;
use dotenv::dotenv;
use std::fs;
use std::env;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL doit être défini dans .env");

    PgConnection::establish(&database_url)
        .expect("Erreur de connexion à la base de données")
}
fn recreate_images_table(connection: &mut PgConnection) {
    let drop_query = "DROP TABLE IF EXISTS images;";
    sql_query(drop_query)
        .execute(connection)
        .expect("Erreur lors de la suppression de la table `images`");


    let create_table_query = r#"
        CREATE TABLE images (
            id SERIAL PRIMARY KEY,
            filepath VARCHAR NOT NULL,
            file_content BYTEA NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
    "#;
    sql_query(create_table_query)
        .execute(connection)
        .expect("Erreur lors de la création de la table `images`");

    println!("Table `images` recréée avec succès !");
}

pub fn run_main_logic(path_file: &str) -> Image {
    dotenv().ok();

    let mut connection = establish_connection();

    recreate_images_table(&mut connection);

    let image_path = path_file;

    if !fs::metadata(image_path).is_ok() {
        panic!("Le fichier image n'existe pas à l'emplacement spécifié !");
    }

    let file_content = match fs::read(image_path) {
        Ok(content) => content,
        Err(err) => panic!("Erreur lors de la lecture du fichier image : {:?}", err),
    };

    let current_time = chrono::Utc::now().naive_utc();
    let new_image = Image {
        id: 1,
        filepath: image_path.to_string(),
        file_content: file_content.clone(),
        created_at: current_time,
    };

    diesel::insert_into(images::table)
        .values(&new_image)
        .execute(&mut connection)
        .expect("Erreur lors de l'insertion dans `images`");

    println!("Image insérée avec succès !");

    let retrieved_image: Image = images::table
        .filter(images::id.eq(new_image.id))
        .first(&mut connection)
        .expect("Erreur lors de la récupération de l'image depuis `images`");

    let is_content_equal = file_content == retrieved_image.file_content;
    println!("Le contenu du fichier correspond-il ? {}", is_content_equal);

    assert!(is_content_equal, "Le contenu du fichier ne correspond pas !");

    return new_image;
}

