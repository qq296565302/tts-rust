use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub api_base: String,
    pub model: String,
    pub output_dir: PathBuf,
    pub default_voice: String,
}

impl Default for Config {
    fn default() -> Self {
        let output_dir = dirs::document_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tts_output");
        
        Self {
            api_key: String::new(),
            api_base: "https://api.xiaomimimo.com/v1".to_string(),
            model: "mimo-v2-tts".to_string(),
            output_dir,
            default_voice: "default_zh".to_string(),
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("tts_tool")
            .join("config.json")
    }

    pub fn load() -> Self {
        let mut config = Self::default();
        
        if let Ok(content) = fs::read_to_string(Self::config_path()) {
            if let Ok(loaded) = serde_json::from_str::<Config>(&content) {
                config = loaded;
            }
        }
        
        let _ = dotenv::dotenv();
        
        if let Ok(key) = std::env::var("TTS_API_KEY") {
            if !key.is_empty() {
                config.api_key = key;
            }
        }
        if let Ok(base) = std::env::var("TTS_BASE_URL") {
            if !base.is_empty() {
                config.api_base = base;
            }
        }
        if let Ok(model) = std::env::var("TTS_MODEL") {
            if !model.is_empty() {
                config.model = model;
            }
        }
        
        config
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    pub fn ensure_output_dir(&self) -> Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        Ok(())
    }
}
