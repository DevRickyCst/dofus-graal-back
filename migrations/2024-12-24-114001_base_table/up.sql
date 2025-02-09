-- Relative path: migrations/2024-12-24-114001_base_table/up.sql
-- Création de la table item_categories
CREATE TABLE item_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) UNIQUE NOT NULL
);

-- Création de la table item_types
CREATE TABLE item_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) UNIQUE NOT NULL
);

-- Création de la table image_urls
CREATE TABLE image_urls (
    id SERIAL PRIMARY KEY,
    icon VARCHAR(200) NOT NULL,
    sd VARCHAR(200) NOT NULL,
    hq VARCHAR(200),
    hd VARCHAR(200)
);
CREATE TABLE ranges (
    id SERIAL PRIMARY KEY,
    min INTEGER NOT NULL,
    max INTEGER NOT NULL
);
-- Création de la table elements
CREATE TABLE elements (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL
);

-- Création de la table servers
CREATE TABLE servers (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL,
    category VARCHAR(100) NOT NULL
);

-- Création de la table classes
CREATE TABLE dofus_classes (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL DEFAULT 'random'
);

