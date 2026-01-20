use anyhow::{Context, Result};
use spin_sdk::variables;

pub struct Config {
    pub ollama_api_url: String,
    pub model: String,
    pub temperature: f32,
}

impl Config {
    pub fn try_load() -> Result<Self> {
        Ok(Config {
            ollama_api_url: variables::get("ollama_api_url")
                .with_context(|| "Error loading ollama_api_url")?,
            model: variables::get("model").with_context(|| "Error loading model")?,
            temperature: variables::get("temperature")
                .with_context(|| "Error loading temperature")?
                .parse()
                .context("temperature must be a valid f32")?,
        })
    }
}
