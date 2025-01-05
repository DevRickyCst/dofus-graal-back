-- Création de la table items
CREATE TABLE items (
    ankama_id INTEGER PRIMARY KEY,
    category_id INTEGER NOT NULL,
    type_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    level INTEGER NOT NULL CHECK (level >= 1 AND level <= 200),
    pods INTEGER,
    image_urls_id INTEGER UNIQUE NOT NULL,
    ap_cost INTEGER,
    max_cast_per_turn INTEGER,
    is_weapon BOOLEAN DEFAULT FALSE NOT NULL,
    is_two_handed BOOLEAN,
    critical_hit_probability INTEGER,
    critical_hit_bonus INTEGER,
    FOREIGN KEY (category_id) REFERENCES item_categories(id) ON DELETE CASCADE,
    FOREIGN KEY (type_id) REFERENCES item_types(id) ON DELETE CASCADE,
    FOREIGN KEY (image_urls_id) REFERENCES image_urls(id) ON DELETE CASCADE
);

-- Création de la table ranges
CREATE TABLE ranges (
    id INTEGER PRIMARY KEY,
    ankama_id INTEGER NOT NULL,
    min INTEGER NOT NULL,
    max INTEGER NOT NULL,
    FOREIGN KEY (ankama_id) REFERENCES items(ankama_id) ON DELETE CASCADE
);

-- Création de la table characters
CREATE TABLE characters (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    level INTEGER DEFAULT 200 NOT NULL,
    server_id INTEGER NOT NULL,
    dofus_classes_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES servers(id) ON DELETE RESTRICT,
    FOREIGN KEY (dofus_classes_id) REFERENCES dofus_classes(id) ON DELETE CASCADE
);

-- Création de la table set_caracteristiques
CREATE TABLE caracteristiques (
    id SERIAL PRIMARY KEY,
    vitalite INTEGER DEFAULT 0 NOT NULL,
    sagesse INTEGER DEFAULT 0 NOT NULL,
    agilite INTEGER DEFAULT 0 NOT NULL,
    intelligence INTEGER DEFAULT 0 NOT NULL,
    chance INTEGER DEFAULT 0 NOT NULL,
    force INTEGER DEFAULT 0 NOT NULL
);

-- Création de la table set_stuffs
CREATE TABLE stuffs (
    id SERIAL PRIMARY KEY,
    chapeau_id INTEGER,
    collier_id INTEGER,
    anneau_1_id INTEGER,
    anneau_2_id INTEGER,
    ceinture_id INTEGER,
    botte_id INTEGER,
    bouclier_id INTEGER,
    arme_id INTEGER,
    FOREIGN KEY (chapeau_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (collier_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (anneau_1_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (anneau_2_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (ceinture_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (botte_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (bouclier_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (arme_id) REFERENCES items(ankama_id) ON DELETE CASCADE
);

-- Création de la table sets
CREATE TABLE sets (
    id SERIAL PRIMARY KEY,
    character_id INTEGER NOT NULL,
    caracteristique_id INTEGER NOT NULL,
    stuff_id INTEGER NOT NULL,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (caracteristique_id) REFERENCES caracteristiques(id) ON DELETE CASCADE,
    FOREIGN KEY (stuff_id) REFERENCES stuffs(id) ON DELETE CASCADE
);
