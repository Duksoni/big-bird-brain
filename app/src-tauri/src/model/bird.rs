use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{BirdPower, BirdResource, Habitat, NestType, PowerEffect, PowerTrigger, Resource};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Bird {
    id: i64,
    name: String,
    tier: String,
    cost: Vec<BirdResource>,
    habitats: Vec<Habitat>,
    power: Option<BirdPower>,
    wingspan: u16, // in cm
    victory_points: u8,
    nest_type: NestType,
    egg_limit: u8,
    is_flocking: bool,
    is_predator: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayedBird {
    bird: Bird,
    laid_eggs: u8,
    tucked_cards: u8,
    cached_resources: u8,
}

#[derive(Serialize)]
pub struct BirdDisplay {
    #[serde(flatten)]
    bird: PlayedBird,
    scientific_name: String,
    power_text: Option<String>,
}

impl Bird {
    pub fn new(
        id: i64,
        name: String,
        tier: String,
        cost: Vec<BirdResource>,
        habitats: Vec<Habitat>,
        power: Option<BirdPower>,
        wingspan: u16,
        victory_points: u8,
        nest_type: NestType,
        egg_limit: u8,
        is_flocking: bool,
        is_predator: bool,
    ) -> Self {
        Self {
            id,
            name,
            tier,
            cost,
            habitats,
            power,
            wingspan,
            victory_points,
            nest_type,
            egg_limit,
            is_flocking,
            is_predator,
        }
    }

    pub fn has_power_trigger(&self, trigger: PowerTrigger) -> bool {
        if let Some(power) = &self.power {
            power.0 == trigger
        } else {
            false
        }
    }

    pub fn has_power_effect(&self, effect_matches: impl Fn(&PowerEffect) -> bool) -> bool {
        if let Some(power) = &self.power {
            effect_matches(&power.1)
        } else {
            false
        }
    }

    pub fn food_cost_value(&self) -> u8 {
        self.cost.len() as u8
    }

    pub fn is_brown_power(&self) -> bool {
        self.has_power_trigger(PowerTrigger::WhenActivated)
    }

    pub fn is_pink_power(&self) -> bool {
        self.has_power_trigger(PowerTrigger::OnceBetweenTurns)
    }

    pub fn is_white_power(&self) -> bool {
        self.has_power_trigger(PowerTrigger::WhenPlayed)
    }

    pub fn is_card_draw_engine(&self) -> bool {
        self.has_power_effect(|effect| matches!(effect, PowerEffect::DrawCards(_)))
            && self.habitats.contains(&Habitat::Wetland)
    }

    pub fn is_food_engine(&self) -> bool {
        self.has_power_effect(|effect| matches!(effect, PowerEffect::GainResource(_)))
    }

    pub fn is_raven_engine(&self) -> bool {
        self.has_power_effect(|effect| matches!(effect, PowerEffect::DiscardToGain(_))) // Simplified check for egg->food
    }

    pub fn can_be_played_with(&self, available_food: &[Resource]) -> bool {
        let mut available_counts: HashMap<Resource, u8> = HashMap::new();
        for &resource in available_food {
            *available_counts.entry(resource).or_insert(0) += 1;
        }

        let mut wild_requirements = 0;

        for cost in self.cost() {
            match cost {
                BirdResource::Standard(required) => {
                    let count = available_counts.entry(*required).or_insert(0);
                    if *count == 0 {
                        return false;
                    }
                    *count -= 1;
                }
                BirdResource::Wild => {
                    wild_requirements += 1;
                }
            }
        }

        let remaining_food: u8 = available_counts.values().copied().sum();
        remaining_food >= wild_requirements
    }
}

impl PlayedBird {
    pub fn new(bird: Bird) -> Self {
        Self {
            bird,
            laid_eggs: 0,
            tucked_cards: 0,
            cached_resources: 0,
        }
    }

    pub fn add_eggs(&mut self, amount: u8) {
        if self.bird.egg_limit == 0 {
            return;
        };
        let new_count = self.laid_eggs + amount;
        if new_count <= self.bird.egg_limit {
            self.laid_eggs = new_count;
        }
    }

    pub fn remove_eggs(&mut self, amount: u8) {
        if self.bird.egg_limit == 0 {
            return;
        };
        self.laid_eggs = self.laid_eggs.saturating_sub(amount);
    }

    pub fn tuck_card(&mut self) {
        self.tucked_cards += 1;
    }

    pub fn undo_tuck_card(&mut self) {
        self.tucked_cards = self.tucked_cards.saturating_sub(1);
    }

    pub fn cache_resource(&mut self) {
        self.cached_resources += 1;
    }

    pub fn undo_cache_resource(&mut self) {
        self.cached_resources = self.cached_resources.saturating_sub(1);
    }

    pub fn sum_points(&self) -> u16 {
        self.bird.victory_points as u16
            + self.laid_eggs as u16
            + self.tucked_cards as u16
            + self.cached_resources as u16
    }
}

impl Bird {
    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn victory_points(&self) -> u8 {
        self.victory_points
    }

    pub fn nest_type(&self) -> NestType {
        self.nest_type
    }

    pub fn egg_limit(&self) -> u8 {
        self.egg_limit
    }

    pub fn wingspan(&self) -> u16 {
        self.wingspan
    }

    pub fn cost(&self) -> &[BirdResource] {
        &self.cost
    }

    pub fn habitats(&self) -> &[Habitat] {
        &self.habitats
    }

    pub fn power(&self) -> Option<&BirdPower> {
        self.power.as_ref()
    }
    pub fn tier(&self) -> &str {
        &self.tier
    }

    pub fn is_flocking(&self) -> bool {
        self.is_flocking
    }

    pub fn is_predator(&self) -> bool {
        self.is_predator
    }
}

impl PlayedBird {
    pub fn bird(&self) -> &Bird {
        &self.bird
    }

    pub fn laid_eggs(&self) -> u8 {
        self.laid_eggs
    }

    pub fn tucked_cards(&self) -> u8 {
        self.tucked_cards
    }

    pub fn cached_resources(&self) -> u8 {
        self.cached_resources
    }
}
