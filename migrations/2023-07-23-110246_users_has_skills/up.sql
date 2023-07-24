-- Your SQL goes here
-- Créer le type ENUM Level avec des valeurs en anglais
CREATE TYPE Level AS ENUM ('beginner', 'intermediate', 'expert');

-- Créer la table users_has_skills
CREATE TABLE users_has_skills (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    level Level NOT NULL
);