CREATE TABLE images (
                        id SERIAL PRIMARY KEY,
                        filepath VARCHAR NOT NULL,
                        file_content bytea NOT NULL,
                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
