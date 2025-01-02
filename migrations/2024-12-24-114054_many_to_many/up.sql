-- Création de la table intermédiaire item_effects
CREATE TABLE item_effects (
    item_id INTEGER NOT NULL,
    effect_id INTEGER NOT NULL,
    PRIMARY KEY (item_id, effect_id),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (effect_id) REFERENCES effect_singles(id) ON DELETE CASCADE
);

-- Création de la table intermédiaire item_recipes
CREATE TABLE item_recipes (
    item_id INTEGER NOT NULL,
    recipe_id INTEGER NOT NULL,
    PRIMARY KEY (item_id, recipe_id),
    FOREIGN KEY (item_id) REFERENCES items(id) ON DELETE CASCADE,
    FOREIGN KEY (recipe_id) REFERENCES recipe_singles(id) ON DELETE CASCADE
);
