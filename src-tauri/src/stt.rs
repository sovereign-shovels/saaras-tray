use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttResult {
    pub text: String,
    pub confidence: Option<f32>,
    pub language: Option<String>,
}

#[async_trait::async_trait]
pub trait SttProvider: Send + Sync {
    async fn transcribe(&self, audio_path: &std::path::Path) -> Result<SttResult, String>;
    fn name(&self) -> &str;
}

pub struct SaarasProvider {
    endpoint: String,
    api_key: Option<String>,
    language: String,
    codemix: bool,
}

impl SaarasProvider {
    pub fn new(endpoint: String, api_key: Option<String>, language: String, codemix: bool) -> Self {
        Self { endpoint, api_key, language, codemix }
    }
}

#[async_trait::async_trait]
impl SttProvider for SaarasProvider {
    fn name(&self) -> &str {
        "saaras-v3"
    }

    async fn transcribe(&self, audio_path: &std::path::Path) -> Result<SttResult, String> {
        // TODO: Implement Saaras v3 WebSocket streaming
        // For v0.1 MVP, return placeholder to verify architecture
        let _ = audio_path;
        let _ = &self.endpoint;
        Ok(SttResult {
            text: "[Placeholder transcription — Saaras v3 integration in progress]".into(),
            confidence: None,
            language: Some(self.language.clone()),
        })
    }
}

pub struct LocalWhisperProvider;

#[async_trait::async_trait]
impl SttProvider for LocalWhisperProvider {
    fn name(&self) -> &str {
        "local-whisper"
    }

    async fn transcribe(&self, audio_path: &std::path::Path) -> Result<SttResult, String> {
        let _ = audio_path;
        Ok(SttResult {
            text: "[Placeholder — local Whisper fallback in v0.5]".into(),
            confidence: None,
            language: Some("auto".into()),
        })
    }
}

pub type SharedProvider = Arc<Mutex<Box<dyn SttProvider>>>;
