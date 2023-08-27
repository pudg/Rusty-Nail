-- Your SQL goes here
CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    drink_name VARCHAR NOT NULL,
    ingredients json NOT NULL,
    instructions TEXT NOT NULL
)