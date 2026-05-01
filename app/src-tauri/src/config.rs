use anyhow::{Context, Result};
use std::env::var;

pub struct Environment {
    pub database_url: String,
}

impl Environment {
    fn new(database_url: String) -> Self {
        Self { database_url }
    }

    pub fn load() -> Result<Environment> {
        let database_url = var("DATABASE_URL").context("Database URL not set")?;

        Ok(Environment::new(database_url))
    }
}
