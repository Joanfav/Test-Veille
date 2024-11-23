use diesel::prelude::*;
use chrono::NaiveDateTime;
use schema::images;
use crate::convertion::schema;

#[derive(Queryable, Insertable, Debug, Clone)]
#[diesel(table_name = images)]
pub struct Image {
    pub id: i32,
    pub filepath: String,
    pub file_content: Vec<u8>,
    pub created_at: NaiveDateTime,
}
