-- Your SQL goes here
CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    author INT NOT NULL,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    created TIMESTAMP NOT NULL,
    modified TIMESTAMP
);
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    author INT NOT NULL,
    post INT NOT NULL,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    created TIMESTAMP NOT NULL,
    modified TIMESTAMP
);