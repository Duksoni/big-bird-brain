use serde::{Deserialize, Serialize};

use super::{Bird, Bonus, Resource};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DraftCombination {
    pub kept_birds: Vec<Bird>,
    pub kept_food: Vec<Resource>,
    pub bonus_card: Bonus,
    pub bird_feeder: Vec<Resource>,

    // Counts (from accumulate rules)
    pub playable_bird_count: u8,
    pub cheap_bird_count: u8,     // birds with food_cost <= 1
    pub expensive_bird_count: u8, // birds with food_cost >= 3
    pub bonus_match_count: u8,    // birds matching bonus condition

    // Availability checks
    pub has_turn1_playable_bird: bool, // can play at least one bird with kept_food
    pub required_food_available: bool, // birdfeeder supports all bird costs

    pub has_wetlands_card_draw_bird: bool,
    pub has_food_generation_bird: bool,
    pub has_egg_to_food_bird: bool, // raven-style engine
    pub has_s_tier_bird: bool,
    pub has_a_tier_bird: bool,
    pub has_brown_or_pink_power_bird: bool,
    pub playable_bird_count_gt_zero: bool, // inverted for DEAD_OPENING detection

    pub facts: Vec<String>, // Level 1 facts: "CHEAP_BIRD", "CARD_ENGINE", etc.
    pub derived_facts: Vec<String>, // Level 2 facts: "HIGH_TEMPO", "WETLANDS_CYCLING"

    // Evaluation results
    pub score: f64,           // cumulative scoring
    pub tags: Vec<String>,    // classification tags
    pub reasons: Vec<String>, // explanation reasons
}

impl DraftCombination {
    pub fn new(
        kept_birds: Vec<Bird>,
        kept_food: Vec<Resource>,
        bonus_card: Bonus,
        bird_feeder: Vec<Resource>,
    ) -> Self {
        Self {
            kept_birds,
            kept_food,
            bonus_card,
            bird_feeder,
            playable_bird_count: Default::default(),
            cheap_bird_count: Default::default(),
            expensive_bird_count: Default::default(),
            bonus_match_count: Default::default(),
            has_turn1_playable_bird: Default::default(),
            required_food_available: Default::default(),
            has_wetlands_card_draw_bird: Default::default(),
            has_food_generation_bird: Default::default(),
            has_egg_to_food_bird: Default::default(),
            has_s_tier_bird: Default::default(),
            has_a_tier_bird: Default::default(),
            has_brown_or_pink_power_bird: Default::default(),
            playable_bird_count_gt_zero: Default::default(),
            facts: Default::default(),
            derived_facts: Default::default(),
            score: Default::default(),
            tags: Default::default(),
            reasons: Default::default(),
        }
    }
}
