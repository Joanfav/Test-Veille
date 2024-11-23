use diesel::table;

table! {
    images (id) {
        id -> Int4,
        filepath -> Varchar,
        file_content -> Bytea,
        created_at -> Timestamp,
    }
}
