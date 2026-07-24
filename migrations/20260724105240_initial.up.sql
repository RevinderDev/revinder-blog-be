-- Add up migration script here
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL,
  is_activated BOOLEAN NOT NULL DEFAULT 0 CHECK (is_activated IN (0, 1))
);
