DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    email VARCHAR NOT NULL
);

INSERT INTO users (email) VALUES ('foo@mail.com'), ('bar@mail.com'), ('foobar@mail.com');