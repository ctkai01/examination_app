-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR UNIQUE NOT NULL,
  password VARCHAR NOT NULL,
  token VARCHAR NOT NULL,
  avatar TEXT,
  full_name VARCHAR,
  bio TEXT,
  dob TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT current_timestamp
)
