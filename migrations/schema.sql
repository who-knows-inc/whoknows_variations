DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);

-- Create a default user, The password is 'password' (MD5 hashed)
INSERT INTO users (username, email, password) 
    VALUES ('admin', 'keamonk1@stud.kea.dk', '5f4dcc3b5aa765d61d8327deb882cf99');


DROP TABLE IF EXISTS pages;

CREATE TABLE IF NOT EXISTS pages (
    title TEXT PRIMARY KEY,
    url TEXT NOT NULL UNIQUE,
    language TEXT NOT NULL DEFAULT 'en' CHECK (language IN ('en', 'da')),
    last_updated TIMESTAMP,
    content TEXT NOT NULL
);
