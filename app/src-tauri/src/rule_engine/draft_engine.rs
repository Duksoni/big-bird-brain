use rust_rule_engine::{Facts, KnowledgeBase, engine::engine::RustRuleEngine};

use super::Result;
use crate::{
    error::AppError,
    model::{Bird, BirdFeederResource, DraftCombination, DraftSetup, DraftSuggestion, Resource},
};

pub struct DraftEngine {
    engine: Box<RustRuleEngine>,
}

impl DraftEngine {
    pub fn new() -> Result<Self> {
        let draft_kb = Self::load_draft_rules()?;
        let engine = RustRuleEngine::new(draft_kb);
        let mut instance = Self {
            engine: Box::new(engine),
        };
        instance.register_handlers();
        instance.register_functions();
        Ok(instance)
    }

    /// Evaluates all possible draft choices and returns top 5 recommendations.
    pub fn evaluate(&mut self, setup: DraftSetup) -> Result<Vec<DraftSuggestion>> {
        let combinations = generate_draft_combinations(setup);

        let mut evaluated_combos = Vec::new();
        for mut combo in combinations {
            self.evaluate_combination(&mut combo)?;
            evaluated_combos.push(combo);
        }

        // Sort by score descending
        evaluated_combos.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Take top 5 and convert to Suggestions
        let recommendations = evaluated_combos
            .into_iter()
            .take(5)
            .map(|c| c.into())
            .collect();

        Ok(recommendations)
    }

    pub fn evaluate_combination(&mut self, combination: &mut DraftCombination) -> Result<()> {
        let mut facts = Facts::new();

        // 1. Derive additional facts and add bird facts
        self.derive_combination_facts(&mut facts, combination)?;

        // 2. Add the derived combination as the DraftCombination fact
        facts.add("DraftCombination", &*combination)?;

        // 3. Execute engine
        self.engine.reset_no_loop_tracking();
        self.engine.clear_agenda_focus();
        self.engine.execute(&facts)?;

        // 4. Retrieve updated combination from engine and update the reference
        let updated = self.extract_result(&facts)?;
        *combination = updated;

        Ok(())
    }

    fn derive_combination_facts(
        &self,
        facts: &mut Facts,
        combination: &mut DraftCombination,
    ) -> Result<()> {
        let (bonus_match, cheap, expensive, playable) =
            self.derive_and_add_bird_facts(facts, combination)?;

        combination.bonus_match_count = bonus_match;
        combination.cheap_bird_count = cheap;
        combination.expensive_bird_count = expensive;
        combination.playable_bird_count = playable;
        combination.has_turn1_playable_bird = playable > 0;

        // Check if birdfeeder has the resources to support all bird costs
        let feeder = combination.bird_feeder.clone();
        combination.required_food_available = combination
            .kept_birds
            .iter()
            .all(|bird| bird.can_be_played_with(&feeder));

        Ok(())
    }

    fn derive_and_add_bird_facts(
        &self,
        facts: &mut Facts,
        combination: &mut DraftCombination,
    ) -> Result<(u8, u8, u8, u8)> {
        let mut bonus_match_count: u8 = 0;
        let mut cheap_bird_count: u8 = 0;
        let mut expensive_bird_count: u8 = 0;
        let mut playable_bird_count: u8 = 0;

        for bird in &combination.kept_birds {
            let mut bird_val =
                serde_json::to_value(bird).map_err(|e| AppError::RuleEngineError(e.to_string()))?;

            let matches_bonus = combination.bonus_card.matches_bird(bird);
            let is_playable = bird.can_be_played_with(&combination.kept_food);
            let food_cost = bird.food_cost_value();

            bird_val["matches_bonus"] = serde_json::Value::Bool(matches_bonus);
            bird_val["food_cost"] = serde_json::Value::Number(food_cost.into());
            bird_val["is_playable"] = serde_json::Value::Bool(is_playable);

            if matches_bonus {
                bonus_match_count += 1;
            }
            if food_cost <= 1 {
                cheap_bird_count += 1;
            }
            if food_cost >= 3 {
                expensive_bird_count += 1;
            }
            if is_playable {
                playable_bird_count += 1;
            }

            if bird.is_card_draw_engine() {
                combination.has_wetlands_card_draw_bird = true;
            }
            if bird.is_food_engine() {
                combination.has_food_generation_bird = true;
            }
            if bird.is_raven_engine() {
                combination.has_egg_to_food_bird = true;
            }
            if bird.is_brown_power() || bird.is_pink_power() {
                combination.has_brown_or_pink_power_bird = true;
            }
            if bird.tier().to_lowercase() == "a" {
                combination.has_a_tier_bird = true;
            }
            if bird.tier().to_lowercase() == "s" {
                combination.has_s_tier_bird = true;
            }

            facts.add("Bird", bird_val)?;
        }

        Ok((
            bonus_match_count,
            cheap_bird_count,
            expensive_bird_count,
            playable_bird_count,
        ))
    }

    fn extract_result(&self, facts: &Facts) -> Result<DraftCombination> {
        let updated_combo_rule_val = facts
            .get("DraftCombination")
            .ok_or_else(|| AppError::RuleEngineError("DraftCombination fact missing".into()))?;

        let updated_combo_val = value_to_json(updated_combo_rule_val);

        let updated_combo: DraftCombination = serde_json::from_value(updated_combo_val.clone())
            .map_err(|e| {
                AppError::RuleEngineError(format!(
                    "Deserialization error: {}, JSON: {}",
                    e, updated_combo_val
                ))
            })?;

        Ok(updated_combo)
    }

    fn load_draft_rules() -> Result<KnowledgeBase> {
        let kb = KnowledgeBase::new("Draft Rules");

        let paths = [
            "rules/draft-phase/level1.grl",
            "rules/draft-phase/level2.grl",
            "rules/draft-phase/level3.grl",
        ];

        for path in &paths {
            let content = std::fs::read_to_string(path).map_err(|e| {
                AppError::RuleEngineError(format!("Failed to read rule file {}: {}", path, e))
            })?;
            kb.add_rules_from_grl(&content)?;
        }

        Ok(kb)
    }

    fn register_handlers(&mut self) {
        use rust_rule_engine::Value;

        // Custom action: add_fact("NAME")
        self.engine
            .register_action_handler("add_fact", |args, facts| {
                let fact = args
                    .get("0")
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();
                if fact.is_empty() {
                    return Ok(());
                }

                Self::mutate_combo(facts, |combo| {
                    if let Some(Value::Array(arr)) = combo.get_mut("facts") {
                        let val = Value::String(fact);
                        if !arr.contains(&val) {
                            arr.push(val);
                        }
                    }
                });
                Ok(())
            });

        // Custom action: add_derived_fact("NAME")
        self.engine
            .register_action_handler("add_derived_fact", |args, facts| {
                let fact = args
                    .get("0")
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();
                if fact.is_empty() {
                    return Ok(());
                }

                Self::mutate_combo(facts, |combo| {
                    if let Some(Value::Array(arr)) = combo.get_mut("derived_facts") {
                        let val = Value::String(fact);
                        if !arr.contains(&val) {
                            arr.push(val);
                        }
                    }
                });
                Ok(())
            });

        // Custom action: add_score(value)
        self.engine
            .register_action_handler("add_score", |args, facts| {
                let score = args.get("0").and_then(|v| v.to_number()).unwrap_or(0.0);
                Self::mutate_combo(facts, |combo| {
                    let current_score = combo
                        .get("score")
                        .and_then(|v| v.to_number())
                        .unwrap_or(0.0);
                    combo.insert("score".to_string(), Value::Number(current_score + score));
                });
                Ok(())
            });

        // Custom action: add_tag("TAG")
        self.engine
            .register_action_handler("add_tag", |args, facts| {
                let tag = args
                    .get("0")
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();
                if tag.is_empty() {
                    return Ok(());
                }

                Self::mutate_combo(facts, |combo| {
                    if let Some(Value::Array(arr)) = combo.get_mut("tags") {
                        let val = Value::String(tag);
                        if !arr.contains(&val) {
                            arr.push(val);
                        }
                    }
                });
                Ok(())
            });

        // Custom action: add_reason("REASON")
        self.engine
            .register_action_handler("add_reason", |args, facts| {
                let reason = args
                    .get("0")
                    .and_then(|v| v.as_string())
                    .unwrap_or_default();
                if reason.is_empty() {
                    return Ok(());
                }

                Self::mutate_combo(facts, |combo| {
                    if let Some(Value::Array(arr)) = combo.get_mut("reasons") {
                        arr.push(Value::String(reason));
                    }
                });
                Ok(())
            });
    }

    fn register_functions(&mut self) {
        use rust_rule_engine::Value;

        // Custom function: has_fact("NAME") -> bool
        self.engine.register_function("has_fact", |args, facts| {
            let fact_name_raw = args.first().and_then(|v| v.as_string()).unwrap_or_default();
            let fact_name = fact_name_raw.trim_matches('"').trim_matches('\'');

            let mut result = false;
            if let Some(Value::Object(combo)) = facts.get("DraftCombination")
                && let Some(Value::Array(arr)) = combo.get("facts")
            {
                result = arr.contains(&Value::String(fact_name.to_string()));
            }
            Ok(Value::Boolean(result))
        });

        // Custom function: has_derived_fact("NAME") -> bool
        self.engine
            .register_function("has_derived_fact", |args, facts| {
                let fact_name_raw = args.first().and_then(|v| v.as_string()).unwrap_or_default();
                let fact_name = fact_name_raw.trim_matches('"').trim_matches('\'');

                let mut result = false;
                if let Some(Value::Object(combo)) = facts.get("DraftCombination")
                    && let Some(Value::Array(arr)) = combo.get("derived_facts")
                {
                    result = arr.contains(&Value::String(fact_name.to_string()));
                }
                Ok(Value::Boolean(result))
            });
    }

    fn mutate_combo<F>(facts: &Facts, mutator: F)
    where
        F: FnOnce(&mut std::collections::HashMap<String, rust_rule_engine::Value>),
    {
        use rust_rule_engine::Value;
        if let Some(Value::Object(mut combo)) = facts.get("DraftCombination") {
            mutator(&mut combo);
            facts.set("DraftCombination", Value::Object(combo));
        }
    }
}

pub fn generate_draft_combinations(setup: DraftSetup) -> Vec<DraftCombination> {
    let mut combinations = Vec::new();

    let starting_food = vec![
        Resource::Invertebrate,
        Resource::Seed,
        Resource::Fish,
        Resource::Fruit,
        Resource::Rodent,
    ];

    let feeder_supported: Vec<Resource> = setup
        .birdfeeder
        .iter()
        .flat_map(|die| match die {
            BirdFeederResource::Standard(r) => vec![*r],
            BirdFeederResource::InvertebrateSeed => vec![Resource::Invertebrate, Resource::Seed],
        })
        .collect();

    for bird_bits in 0..(1u32 << setup.hand_birds.len()) {
        let kept_birds: Vec<Bird> = setup
            .hand_birds
            .iter()
            .enumerate()
            .filter_map(|(idx, bird)| {
                if bird_bits & (1 << idx) != 0 {
                    Some(bird.clone())
                } else {
                    None
                }
            })
            .collect();

        let needed_food_count = 5 - kept_birds.len();

        for bonus_card in &setup.bonus_cards {
            let food_options = get_combinations(&starting_food, needed_food_count);

            for kept_food in food_options {
                combinations.push(DraftCombination::new(
                    kept_birds.clone(),
                    kept_food,
                    bonus_card.clone(),
                    feeder_supported.clone(),
                ));
            }
        }
    }
    combinations
}

fn get_combinations<T: Clone>(items: &[T], k: usize) -> Vec<Vec<T>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.is_empty() {
        return vec![];
    }

    let mut result = Vec::new();
    for mut combo in get_combinations(&items[1..], k - 1) {
        combo.insert(0, items[0].clone());
        result.push(combo);
    }
    result.extend(get_combinations(&items[1..], k));

    result
}

/// For debugging
fn value_to_json(v: rust_rule_engine::Value) -> serde_json::Value {
    use rust_rule_engine::Value;
    match v {
        Value::String(s) => serde_json::Value::String(s),
        Value::Number(n) => serde_json::Number::from_f64(n)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::Integer(i) => serde_json::Value::Number(i.into()),
        Value::Boolean(b) => serde_json::Value::Bool(b),
        Value::Array(arr) => serde_json::Value::Array(arr.into_iter().map(value_to_json).collect()),
        Value::Object(obj) => {
            let mut map = serde_json::Map::new();
            for (k, v) in obj {
                map.insert(k, value_to_json(v));
            }
            serde_json::Value::Object(map)
        }
        Value::Null => serde_json::Value::Null,
        Value::Expression(e) => serde_json::Value::String(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Bird, BirdFeederResource, Bonus, Resource};

    #[test]
    fn test_facts_accumulation() {
        let mut engine = DraftEngine::new().expect("Failed to initialize DraftEngine");

        let bird: Bird = serde_json::from_value(serde_json::json!({
            "id": 111,
            "name": "Mourning Dove",
            "tier": "a",
            "victory_points": 0,
            "nest_type": "platform",
            "egg_limit": 5,
            "wingspan": 46,
            "power_trigger": null,
            "power": null,
            "cost": [{"Standard": "Seed"}],
            "habitats": ["forest", "grassland", "wetland"],
            "powers": [],
            "total_food_cost": 1,
            "is_predator": false,
            "is_flocking": false,
            "has_bonus_card_power": false
        }))
        .expect("Failed to create mock bird");

        let bonus: Bonus = serde_json::from_value(serde_json::json!({
            "id": 1,
            "name": "Platform Builder",
            "description": "Birds with platform nests",
            "scoring": { "type": "perBird", "value": 2 },
            "condition": { "type": "nestTypeCount", "value": "platform" }
        }))
        .expect("Failed to create mock bonus");

        let mut combo = DraftCombination::new(
            vec![bird],
            vec![Resource::Seed],
            bonus,
            vec![Resource::Seed],
        );

        let result = engine.evaluate_combination(&mut combo);
        assert!(result.is_ok(), "Evaluation failed: {:?}", result.err());

        let evaluated = combo;

        println!("\n=== ACCUMULATION ===");
        println!("cheap_bird_count: {}", evaluated.cheap_bird_count);
        println!("bonus_match_count: {}", evaluated.bonus_match_count);
        println!(
            "has_turn1_playable_bird: {}",
            evaluated.has_turn1_playable_bird
        );
        println!("has_a_tier_bird: {}", evaluated.has_a_tier_bird);
        println!(
            "required_food_available: {}",
            evaluated.required_food_available
        );
        println!(
            "\nFacts ({} total): {:?}",
            evaluated.facts.len(),
            evaluated.facts
        );
        println!(
            "Derived Facts ({} total): {:?}",
            evaluated.derived_facts.len(),
            evaluated.derived_facts
        );
        println!("Score: {}", evaluated.score);
        println!("Tags: {:?}", evaluated.tags);
        println!("Reasons: {:?}", evaluated.reasons);

        // Assertions
        assert!(evaluated.cheap_bird_count > 0, "Should detect cheap bird");
        assert!(evaluated.bonus_match_count > 0, "Should match bonus");
        assert!(
            evaluated.has_turn1_playable_bird,
            "Should be playable on turn 1"
        );
        assert!(evaluated.has_a_tier_bird, "Should detect A-tier bird");
        assert!(
            evaluated.required_food_available,
            "Birdfeeder should support costs"
        );

        assert!(
            evaluated.facts.contains(&"CHEAP_BIRD".to_string()),
            "CHEAP_BIRD should be in facts, but got: {:?}",
            evaluated.facts
        );
        assert!(
            evaluated.facts.contains(&"HIGH_TIER_BIRD".to_string()),
            "HIGH_TIER_BIRD should be in facts, but got: {:?}",
            evaluated.facts
        );
    }

    #[test]
    fn test_combination_generation() {
        let setup = DraftSetup {
            hand_birds: vec![
                mock_bird(1, "C"),
                mock_bird(2, "C"),
                mock_bird(3, "C"),
                mock_bird(4, "C"),
                mock_bird(5, "C"),
            ],
            bonus_cards: vec![mock_bonus1(1), mock_bonus2(2)],
            birdfeeder: vec![BirdFeederResource::Standard(Resource::Seed); 5],
        };

        let combos = generate_draft_combinations(setup);
        assert_eq!(combos.len(), 504);

        let combo_3_birds = combos.iter().find(|c| c.kept_birds.len() == 3).unwrap();
        assert_eq!(combo_3_birds.kept_food.len(), 2);
    }

    #[test]
    fn test_end_to_end_evaluation() {
        let mut engine = DraftEngine::new().expect("Failed to initialize DraftEngine");

        let setup = DraftSetup {
            hand_birds: vec![
                mock_bird(1, "s"),
                mock_bird(2, "c"),
                mock_bird(3, "c"),
                mock_bird(4, "c"),
                mock_bird(5, "c"),
            ],
            bonus_cards: vec![mock_bonus1(1), mock_bonus2(2)],
            birdfeeder: vec![BirdFeederResource::Standard(Resource::Seed); 5],
        };

        let suggestions = engine.evaluate(setup).expect("Evaluation failed");

        assert_eq!(suggestions.len(), 5);
        let top = &suggestions[0];
        assert!(top.score > 0.0);
    }

    fn mock_bird(id: i64, tier: &str) -> Bird {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "name": format!("Bird {}", id),
            "tier": tier,
            "victory_points": 2,
            "nest_type": "platform",
            "egg_limit": 3,
            "wingspan": 20,
            "cost": [{"Standard": "Seed"}],
            "habitats": ["grassland"],
            "powers": [],
            "is_predator": false,
            "is_flocking": false
        }))
        .unwrap()
    }

    fn mock_bonus1(id: i64) -> Bonus {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "scoring": {
                "type": "perBird",
                "value": 2
            },
            "condition": {
                "type": "nestTypeCount",
                "value": "platform"
            }
        }))
        .unwrap()
    }

    fn mock_bonus2(id: i64) -> Bonus {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "scoring": {
                "type": "threshold",
                "value": {
                    "low_threshold": 2,
                    "high_threshold": 4,
                    "low_reward_points": 3,
                    "high_reward_points": 7,
                }
            },
            "condition": {
                "type": "miscellaneous",
                "value": "anatomist"
            }
        }))
        .unwrap()
    }
}
