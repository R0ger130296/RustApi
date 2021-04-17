-- Your SQL goes here

CREATE TABLE tareas (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
)