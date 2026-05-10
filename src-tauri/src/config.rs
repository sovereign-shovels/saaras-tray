use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub provider_name: Option<String>,
    pub endpoint: Option<String>,
    pub api_key_env_var: Option<String>,
    pub model: Option<String>,
    pub language: Option<String>,
    pub codemix: Option<bool>,
    pub hotkey: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct TomlConfig {
    provider: Option<Config>,
}

impl Config {
    pub fn load(app_name: &str) -> Self {
        let prefix = app_name.to_uppercase().replace("-", "_");
        let mut merged = Self::default();

        if let Some(path) = Self::path(app_name) {
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(toml) = toml::from_str::<TomlConfig>(&content) {
                    if let Some(p) = toml.provider {
                        merged.provider_name = p.provider_name.or(merged.provider_name);
                        merged.endpoint = p.endpoint.or(merged.endpoint);
                        merged.model = p.model.or(merged.model);
                        merged.api_key_env_var = p.api_key_env_var.or(merged.api_key_env_var);
                        merged.language = p.language.or(merged.language);
                        merged.codemix = p.codemix.or(merged.codemix);
                        merged.hotkey = p.hotkey.or(merged.hotkey);
                    }
                }
            }
        }

        let env = |k: &str| env::var(format!("{}_{}", prefix, k)).ok();
        merged.provider_name = env("NAME").or(merged.provider_name);
        merged.endpoint = env("ENDPOINT").or(merged.endpoint);
        merged.model = env("MODEL").or(merged.model);
        merged.api_key_env_var = env("API_KEY_ENV").or(merged.api_key_env_var);
        merged.language = env("LANGUAGE").or(merged.language);
        if let Ok(v) = env::var(format!("{}_CODEMIX", prefix)) {
            merged.codemix = Some(v == "true" || v == "1");
        }
        merged.hotkey = env("HOTKEY").or(merged.hotkey);

        // Defaults
        if merged.endpoint.is_none() {
            merged.endpoint = Some("wss://api.sarvam.ai/v1/speech-to-text/stream".into());
        }
        if merged.language.is_none() {
            merged.language = Some("hi-IN".into());
        }
        if merged.codemix.is_none() {
            merged.codemix = Some(true);
        }
        if merged.hotkey.is_none() {
            merged.hotkey = Some("CmdOrCtrl+Shift+S".into());
        }

        merged
    }

    pub fn path(app_name: &str) -> Option<PathBuf> {
        let base = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .ok()
            .or_else(|| env::var("HOME").map(PathBuf::from).ok())?;
        Some(base.join(".config").join(app_name).join("config.toml"))
    }

    pub fn api_key(&self) -> Option<String> {
        self.api_key_env_var.as_ref().and_then(|k| env::var(k).ok())
    }
}
