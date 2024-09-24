-- Add migration script here
-- removes the default user and adds a new one with bcrypt hashed password
DELETE FROM users WHERE username = 'admin';
INSERT INTO users (username, email, password) 
    VALUES ('admin', 'keamonk1@stud.kea.dk', ' $2a$12$Xm.FSBCfvQfGbEC.S5mY2Oc6hWM6B4bCyAWS1rc2BVcoS0rB7PxIC ');
