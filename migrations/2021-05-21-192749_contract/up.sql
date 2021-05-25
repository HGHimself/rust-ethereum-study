-- Your SQL goes here
CREATE TABLE contract (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  owner VARCHAR NOT NULL,
  address VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,
  active BOOLEAN NOT NULL
)
