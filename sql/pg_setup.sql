DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL
);

INSERT INTO users (email) VALUES ('foo@mail.com'), ('bar@mail.com'), ('foobar@mail.com');