-- Création de la table item_categories
CREATE TABLE item_categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL
);

-- Création de la table item_types
CREATE TABLE item_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL
);

-- Création de la table image_urls
CREATE TABLE image_urls (
    id SERIAL PRIMARY KEY,
    icon VARCHAR(200) NOT NULL,
    sd VARCHAR(200) NOT NULL,
    hq VARCHAR(200) NOT NULL,
    hd VARCHAR(200) NOT NULL
);

-- Création de la table elements
CREATE TABLE elements (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL
);

-- Création de la table recipe_singles
CREATE TABLE recipe_singles (
    id SERIAL PRIMARY KEY,
    ankama_id INTEGER NOT NULL,
    item_subtype VARCHAR(200) NOT NULL,
    quantity INTEGER DEFAULT 1 NOT NULL
);

-- Création de la table effect_singles
CREATE TABLE effect_singles (
    id SERIAL PRIMARY KEY,
    int_minimum INTEGER DEFAULT 0 NOT NULL,
    int_maximum INTEGER DEFAULT 0 NOT NULL,
    element_id INTEGER NOT NULL,
    ignore_int_min BOOLEAN DEFAULT FALSE NOT NULL,
    ignore_int_max BOOLEAN DEFAULT FALSE NOT NULL,
    formatted VARCHAR(200) NOT NULL,
    FOREIGN KEY (element_id) REFERENCES elements(id) ON DELETE CASCADE
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
