-- Initial migration: add project table
CREATE TABLE projects (
    id              SERIAL PRIMARY KEY,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL,
    date_created    DATE NOT NULL
);
