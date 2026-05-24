use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Resource {
    Invertebrate,
    Seed,
    Fish,
    Fruit,
    Rodent,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirdResource {
    Standard(Resource),
    Wild,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirdFeederResource {
    Standard(Resource),
    InvertebrateSeed,
}

/// Expand BirdFeederResource variants into individual Resource options.
/// If a die is InvertebrateSeed, it expands into both Invertebrate and Seed options for combination generation.
pub fn expand_birdfeeder_resources(birdfeeder: &[BirdFeederResource]) -> Vec<Vec<Resource>> {
    birdfeeder.iter().fold(vec![vec![]], |acc, die| {
        let options = match die {
            BirdFeederResource::Standard(r) => vec![*r],
            BirdFeederResource::InvertebrateSeed => vec![Resource::Invertebrate, Resource::Seed],
        };

        acc.into_iter()
            .flat_map(|combo| {
                options.iter().map(move |&opt| {
                    let mut next = combo.clone();
                    next.push(opt);
                    next
                })
            })
            .collect()
    })
}
