DROP TABLE IF EXISTS users;

CREATE TABLE IF NOT EXISTS users (
  id SERIAL PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL UNIQUE,
  password TEXT NOT NULL
);

-- Create a default user, The password is 'password' (BCrypt hashed)
INSERT INTO users (username, email, password) 
    VALUES ('admin', 'keamonk1@stud.kea.dk', ' $2a$12$Xm.FSBCfvQfGbEC.S5mY2Oc6hWM6B4bCyAWS1rc2BVcoS0rB7PxIC ');
