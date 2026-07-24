-- Add up migration script here
CREATE TABLE users (
  id int PRIMARY KEY,
  email varchar(255) NOT NULL,
  password varchar(255) NOT NULL
);
