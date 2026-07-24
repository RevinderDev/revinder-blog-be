-- Add up migration script here
CREATE TABLE users (
  id INTEGER PRIMARY KEY,
  email varchar(255) NOT NULL,
  password varchar(255) NOT NULL,
  is_activated INTEGER NOT NULL DEFAULT 0 CHECK (is_activated IN (0, 1))
);
