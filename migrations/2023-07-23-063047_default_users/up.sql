-- Your SQL goes here
INSERT INTO users (firstname, lastname, email, bio, password, role)
VALUES ('aurélien', 'lheureux', 'aurelien@alixir.fr', 'Créateur de alixir.fr', crypt('123456789', 'password'), 'ADMIN');