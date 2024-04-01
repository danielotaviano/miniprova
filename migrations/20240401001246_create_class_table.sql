-- Add migration script here


CREATE TABLE class (
    id VARCHAR PRIMARY KEY,
    name VARCHAR NOT NULL,
    code VARCHAR NOT NULL,
    description TEXT NOT NULL,
    user_id VARCHAR NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "user"(id),
    UNIQUE (code)
);