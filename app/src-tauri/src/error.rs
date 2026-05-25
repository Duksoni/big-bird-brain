use rust_rule_engine::RuleEngineError;
use serde::{Serialize, ser::Serializer};
use sqlx::Error as SqlxError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Rule engine error: {0}")]
    RuleEngineError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] SqlxError),
    #[error("Rust rule engine error: {0}")]
    RustRuleEngineError(#[from] RuleEngineError),
}

pub type Result<T> = std::result::Result<T, AppError>;

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
