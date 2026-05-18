PRAGMA foreign_keys = ON;

-- Table for Bonus Cards
CREATE TABLE
    IF NOT EXISTS bonuses (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        percent INTEGER,
        tier TEXT NOT NULL CHECK (tier IN ('s', 'a', 'b', 'c', 'd')),
        condition TEXT NOT NULL, -- in JSON
        low_threshold INTEGER,
        high_threshold INTEGER,
        low_reward_points INTEGER,
        high_reward_points INTEGER,
        per_bird INTEGER
    );

-- Table for Bonus UI Details
CREATE TABLE
    IF NOT EXISTS bonus_details (
        bonus_id INTEGER PRIMARY KEY,
        condition_text TEXT NOT NULL,
        explanatory_text TEXT,
        flavor_note TEXT,
        tier_explanation TEXT,
        FOREIGN KEY (bonus_id) REFERENCES bonuses (id) ON DELETE CASCADE
    );

-- Table for Birds
CREATE TABLE
    IF NOT EXISTS birds (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        tier TEXT NOT NULL CHECK (tier IN ('s', 'a', 'b', 'c', 'u')), -- 'U' stands for 'Underused'
        victory_points INTEGER NOT NULL,
        nest_type TEXT CHECK (
            nest_type IN ('platform', 'bowl', 'cavity', 'ground', 'star')
        ),
        egg_limit INTEGER NOT NULL,
        wingspan INTEGER NOT NULL,
        power_trigger TEXT CHECK (
            power_trigger in (
                'when_activated',
                'when_played',
                'once_between_turns'
            )
        ),
        power TEXT, -- JSON, e.g. {"type": "drawCards", "value": 1}
        total_food_cost INTEGER NOT NULL DEFAULT 0,
        is_predator BOOLEAN NOT NULL DEFAULT 0,
        is_flocking BOOLEAN NOT NULL DEFAULT 0,
        has_bonus_card_power BOOLEAN NOT NULL DEFAULT 0
    );

-- Table for Bird UI Details
CREATE TABLE
    IF NOT EXISTS bird_details (
        bird_id INTEGER PRIMARY KEY,
        scientific_name TEXT NOT NULL,
        power_text TEXT,
        note TEXT,
        FOREIGN KEY (bird_id) REFERENCES birds (id) ON DELETE CASCADE
    );

-- Join table for Bird Habitats
CREATE TABLE
    IF NOT EXISTS bird_habitats (
        bird_id INTEGER NOT NULL,
        habitat TEXT NOT NULL CHECK (habitat IN ('forest', 'grassland', 'wetland')),
        PRIMARY KEY (bird_id, habitat),
        FOREIGN KEY (bird_id) REFERENCES birds (id) ON DELETE CASCADE
    );

-- Join table for Bird Food Costs
CREATE TABLE
    IF NOT EXISTS bird_costs (
        bird_id INTEGER NOT NULL,
        resource TEXT CHECK (
            resource IN (
                'invertebrate',
                'seed',
                'fish',
                'fruit',
                'rodent',
                'wild'
            )
        ),
        amount INTEGER NOT NULL DEFAULT 1,
        PRIMARY KEY (bird_id, resource),
        FOREIGN KEY (bird_id) REFERENCES birds (id) ON DELETE CASCADE
    );

-- Join table mapping Birds to the Bonuses they satisfy
CREATE TABLE
    IF NOT EXISTS bird_bonus_qualifications (
        bird_id INTEGER NOT NULL,
        bonus_id INTEGER NOT NULL,
        PRIMARY KEY (bird_id, bonus_id),
        FOREIGN KEY (bird_id) REFERENCES birds (id) ON DELETE CASCADE,
        FOREIGN KEY (bonus_id) REFERENCES bonuses (id) ON DELETE CASCADE
    );

CREATE TABLE
    IF NOT EXISTS round_goals (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        condition TEXT NOT NULL, -- JSON logic
        explanation TEXT NOT NULL
    );

CREATE TABLE
    IF NOT EXISTS games (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        state TEXT NOT NULL, -- JSON
        created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
    );

-- Indexes
CREATE INDEX IF NOT EXISTS idx_birds_name ON birds (name);

CREATE INDEX IF NOT EXISTS idx_birds_tier ON birds (tier);

CREATE INDEX IF NOT EXISTS idx_bonuses_name ON bonuses (name);

CREATE INDEX IF NOT EXISTS idx_bird_habitats_bird_id ON bird_habitats (bird_id);

CREATE INDEX IF NOT EXISTS idx_bird_costs_bird_id ON bird_costs (bird_id);