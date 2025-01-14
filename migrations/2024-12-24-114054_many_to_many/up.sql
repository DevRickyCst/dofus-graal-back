-- Relative path: migrations/2024-12-24-114054_many_to_many/up.sql
-- Création de la table intermédiaire item_effects
CREATE TABLE item_effects (
    ankama_id INTEGER NOT NULL,
    effect_id INTEGER NOT NULL,
    PRIMARY KEY (ankama_id, effect_id),
    FOREIGN KEY (ankama_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (effect_id) REFERENCES effect_singles(id) ON DELETE CASCADE
);

-- Création de la table intermédiaire item_recipes
CREATE TABLE item_recipes (
    ankama_id INTEGER NOT NULL,
    recipe_id INTEGER NOT NULL,
    PRIMARY KEY (ankama_id, recipe_id),
    FOREIGN KEY (ankama_id) REFERENCES items(ankama_id) ON DELETE CASCADE,
    FOREIGN KEY (recipe_id) REFERENCES recipe_singles(id) ON DELETE CASCADE
);
