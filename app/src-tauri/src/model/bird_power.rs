use serde::{Deserialize, Serialize};

use super::{NestType, Resource};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub struct BirdPower(pub PowerTrigger, pub PowerEffect);

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PowerTrigger {
    WhenActivated,    // Brown effect
    WhenPlayed,       // White effect
    OnceBetweenTurns, // Pink effect
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum PowerEffect {
    DrawCards(u8),
    GainResource(Resource),
    LayEggs(u8),
    TuckCard(u8),
    CacheResource(Resource),
    TradeResource(Resource), // "wild"

    DiscardToDraw(u8),
    DiscardToGain(Resource),
    DiscardToTuck(u8),

    AllPlayersDrawCards(u8),
    AllPlayersGainResource(Resource),
    AllPlayersLayEggs(u8),
    AllPlayersGainResourceFromFeeder(u8),
    AllPlayersLayEggsOnNest(NestType),

    TuckIfWingspanLess(u16),
    CacheIfDieMatches(Resource),
    GainAllResourceFromFeeder(Resource),
    GainResourceFromFeeder(u8),
    GainResourceIfFewest(u8),
    GainResourceOnPredatorSuccess(u8),
    CacheOnPlayerAction(Resource),
    DrawCardsIfFewest(u8),

    RepeatPredatorPower(bool),
    RepeatHabitatPower(bool),
    MoveBird(bool),
    PlayAdditionalBird(bool),

    DrawFaceUpCards(u8),
    DrawBonusCard(u8),
}
