-- Add migration script here

CREATE TABLE "user" (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    avatar_url VARCHAR NOT NULL
);