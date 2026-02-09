require 'sqlite3'

db = SQLite3::Database.new 'whoknows.db'

schema = <<~SQL
  DROP TABLE IF EXISTS users;

  CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL
  );

  INSERT INTO users (username, email, password)
  VALUES ('admin', 'keamonk1@stud.kea.dk', '5f4dcc3b5aa765d61d8327deb882cf99');

  CREATE TABLE IF NOT EXISTS pages (
    title TEXT PRIMARY KEY UNIQUE,
    url TEXT NOT NULL UNIQUE,
    language TEXT NOT NULL CHECK(language IN ('en', 'da')) DEFAULT 'en',
    last_updated TIMESTAMP,
    content TEXT NOT NULL
  );
SQL

db.execute_batch(schema)