use rust_rule_engine::{
    KnowledgeBase, backward::BackwardEngine, engine::engine::RustRuleEngine,
    streaming::StreamRuleEngine,
};

use super::{GameState, Result};
use crate::model::MoveSuggestion;

pub struct MainEngine {
    forward_chaining_engine: Box<RustRuleEngine>,
    backward_chaining_engine: Box<BackwardEngine>,
    cep_engine: Box<StreamRuleEngine>,
}

impl MainEngine {
    pub fn new() -> Result<Self> {
        let kb_forward = Self::load_forward_chaining_rules()?;
        let kb_backward = Self::load_backward_chaining_rules()?;

        Ok(Self {
            forward_chaining_engine: Box::new(RustRuleEngine::new(kb_forward)),
            backward_chaining_engine: Box::new(BackwardEngine::new(kb_backward)),
            cep_engine: Box::new(StreamRuleEngine::new()),
        })
    }

    pub fn evaluate(&mut self, game_state: &GameState) -> Result<Vec<MoveSuggestion>> {
        todo!()
    }

    fn load_forward_chaining_rules() -> Result<KnowledgeBase> {
        todo!()
    }

    fn load_backward_chaining_rules() -> Result<KnowledgeBase> {
        todo!()
    }
}
