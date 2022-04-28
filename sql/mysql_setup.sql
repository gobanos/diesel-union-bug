DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id INTEGER AUTO_INCREMENT PRIMARY KEY ,
    email VARCHAR(255) NOT NULL
);

INSERT INTO users (email) VALUES ('foo@mail.com'), ('bar@mail.com'), ('foobar@mail.com');