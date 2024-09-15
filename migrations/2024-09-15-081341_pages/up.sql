DROP TABLE IF EXISTS pages;

CREATE TABLE IF NOT EXISTS pages (
    title TEXT PRIMARY KEY,
    url TEXT NOT NULL UNIQUE,
    language TEXT NOT NULL DEFAULT 'en' CHECK (language IN ('en', 'da')),
    last_updated TIMESTAMP,
    content TEXT NOT NULL
);