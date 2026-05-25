PRAGMA foreign_keys = OFF;

DROP INDEX IF EXISTS idx_bird_costs_bird_id;

DROP INDEX IF EXISTS idx_bird_habitats_bird_id;

DROP INDEX IF EXISTS idx_bonuses_name;

DROP INDEX IF EXISTS idx_birds_tier;

DROP INDEX IF EXISTS idx_birds_name;

DROP TABLE IF EXISTS games;

DROP TABLE IF EXISTS goals;

DROP TABLE IF EXISTS bird_bonus_qualifications;

DROP TABLE IF EXISTS bird_costs;

DROP TABLE IF EXISTS bird_habitats;

DROP TABLE IF EXISTS bonus_details;

DROP TABLE IF EXISTS bird_details;

DROP TABLE IF EXISTS bonuses;

DROP TABLE IF EXISTS birds;