-- Table: ItemCategory
CREATE TABLE ItemCategory (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL
);

-- Table: Itemtype
CREATE TABLE Itemtype (
    id SERIAL PRIMARY KEY,
    name VARCHAR(20) NOT NULL
);

-- Table: ImageUrls
CREATE TABLE ImageUrls (
    id SERIAL PRIMARY KEY,
    icon VARCHAR(200) NOT NULL,
    sd VARCHAR(200) NOT NULL,
    hq VARCHAR(200) NOT NULL,
    hd VARCHAR(200) NOT NULL
);

-- Table: RecipeSingle
CREATE TABLE RecipeSingle (
    id SERIAL PRIMARY KEY,
    item_ankama_id INTEGER NOT NULL,
    item_subtype VARCHAR(200) NOT NULL,
    quantity INTEGER DEFAULT 1
);

-- Table: Element
CREATE TABLE Element (
    id SERIAL PRIMARY KEY,
    name VARCHAR(200) NOT NULL
);

-- Table: EffectSingle
CREATE TABLE EffectSingle (
    id SERIAL PRIMARY KEY,
    int_minimum INTEGER DEFAULT 0,
    int_maximum INTEGER DEFAULT 0,
    element_id INTEGER NOT NULL,
    ignore_int_min BOOLEAN DEFAULT FALSE,
    ignore_int_max BOOLEAN DEFAULT FALSE,
    formatted VARCHAR(200) NOT NULL,
    FOREIGN KEY (element_id) REFERENCES Element(id)
);

-- Table: Item
CREATE TABLE Item (
    ankama_id INTEGER PRIMARY KEY,
    category VARCHAR(20) NOT NULL,
    type_id INTEGER NOT NULL,
    name VARCHAR(100) NOT NULL,
    description VARCHAR(1000) NOT NULL,
    level INTEGER CHECK(level >= 1 AND level <= 200),
    pods INTEGER DEFAULT NULL,
    image_urls_id INTEGER UNIQUE NOT NULL,
    ap_cost INTEGER DEFAULT NULL,
    max_cast_per_turn INTEGER DEFAULT NULL,
    is_weapon BOOLEAN DEFAULT FALSE,
    is_two_handed BOOLEAN DEFAULT NULL,
    critical_hit_probability INTEGER DEFAULT NULL,
    critical_hit_bonus INTEGER DEFAULT NULL,
    FOREIGN KEY (type_id) REFERENCES Itemtype(id),
    FOREIGN KEY (image_urls_id) REFERENCES ImageUrls(id)
);

-- Table: Range
CREATE TABLE Range (
    item_id INTEGER PRIMARY KEY,
    min INTEGER NOT NULL,
    max INTEGER NOT NULL,
    FOREIGN KEY (item_id) REFERENCES Item(ankama_id)
);

-- Table: Server
CREATE TABLE Server (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE NOT NULL
);

-- Table: CharacterClass
CREATE TABLE CharacterClass (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) UNIQUE DEFAULT 'random',
    logo_url VARCHAR(100) DEFAULT 'random'
);

-- Table: Character
CREATE TABLE Character (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    level INTEGER DEFAULT 200,
    server_id INTEGER DEFAULT NULL,
    character_class_id INTEGER DEFAULT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (server_id) REFERENCES Server(id),
    FOREIGN KEY (character_class_id) REFERENCES CharacterClass(id),
    FOREIGN KEY (user_id) REFERENCES auth_user(id)
);

-- Table: SetCaracteristique
CREATE TABLE SetCaracteristique (
    id SERIAL PRIMARY KEY,
    vitalite INTEGER DEFAULT 0,
    sagesse INTEGER DEFAULT 0,
    agilite INTEGER DEFAULT 0,
    intelligence INTEGER DEFAULT 0,
    chance INTEGER DEFAULT 0,
    force INTEGER DEFAULT 0
);

-- Table: SetStuff
CREATE TABLE SetStuff (
    id SERIAL PRIMARY KEY,
    chapeau_id INTEGER DEFAULT NULL,
    collier_id INTEGER DEFAULT NULL,
    anneau_1_id INTEGER DEFAULT NULL,
    anneau_2_id INTEGER DEFAULT NULL,
    ceinture_id INTEGER DEFAULT NULL,
    botte_id INTEGER DEFAULT NULL,
    bouclier_id INTEGER DEFAULT NULL,
    arme_id INTEGER DEFAULT NULL,
    FOREIGN KEY (chapeau_id) REFERENCES Item(ankama_id),
    FOREIGN KEY (collier_id) REFERENCES Item(ankama_id),
    FOREIGN KEY (anneau_1_id) REFERENCES Item(ankama_id),
    FOREIGN KEY (anneau_2_id) REFERENCES Item(ankama_id),
    FOREIGN KEY (ceinture_id) REFERENCES Item(ankama_id),
    FOREIGN KEY (botte_id) REFERENCES Item(ankama_id),
    FOREIGN KEY (bouclier_id) REFERENCES Item(ankama_id),
    FOREIGN KEY (arme_id) REFERENCES Item(ankama_id)
);

-- Table: Set
CREATE TABLE Set (
    id SERIAL PRIMARY KEY,
    character_id INTEGER NOT NULL,
    raw_caracteristique_id INTEGER DEFAULT NULL,
    parcho_caracteristique_id INTEGER DEFAULT NULL,
    stuff_id INTEGER DEFAULT NULL,
    FOREIGN KEY (character_id) REFERENCES Character(id),
    FOREIGN KEY (raw_caracteristique_id) REFERENCES SetCaracteristique(id),
    FOREIGN KEY (parcho_caracteristique_id) REFERENCES SetCaracteristique(id),
    FOREIGN KEY (stuff_id) REFERENCES SetStuff(id)
);
