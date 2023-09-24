-- Initial migration: add project table
CREATE TABLE projects (
    id              SERIAL PRIMARY KEY,
    name            TEXT NOT NULL,
    description     TEXT NOT NULL,
    image_url       TEXT,
    pinned          BOOL NOT NULL DEFAULT false,
    date_created    DATE NOT NULL DEFAULT NOW()
);
