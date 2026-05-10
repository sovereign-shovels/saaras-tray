use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttResult {
    pub text: String,
    pub confidence: Option<f32>,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SaarasResponse {
    transcript: String,
    #[serde(rename = "language_code")]
    language_code: Option<String>,
}

#[async_trait::async_trait]
pub trait SttProvider: Send + Sync {
    async fn transcribe(&self, audio_path: &Path) -> Result<SttResult, String>;
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

    async fn transcribe(&self, audio_path: &Path) -> Result<SttResult, String> {
        let api_key = self.api_key.as_ref().ok_or("SAARAS_API_KEY not set")?;

        let file_bytes = tokio::fs::read(audio_path)
            .await
            .map_err(|e| format!("Failed to read audio file: {}", e))?;

        let file_name = audio_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("audio.wav")
            .to_string();

        let part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("audio/wav")
            .map_err(|e| e.to_string())?;

        let form = reqwest::multipart::Form::new()
            .part("file", part)
            .text("model", "saaras:v3")
            .text("language_code", self.language.clone());

        let client = reqwest::Client::new();
        let response = client
            .post(&self.endpoint)
            .header("api-subscription-key", api_key)
            .multipart(form)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(format!("Saaras API error ({}): {}", status, body));
        }

        let parsed: SaarasResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(SttResult {
            text: parsed.transcript,
            confidence: None,
            language: parsed.language_code,
        })
    }
}

pub struct LocalWhisperProvider;

#[async_trait::async_trait]
impl SttProvider for LocalWhisperProvider {
    fn name(&self) -> &str {
        "local-whisper"
    }

    async fn transcribe(&self, _audio_path: &Path) -> Result<SttResult, String> {
        Ok(SttResult {
            text: "[Placeholder — local Whisper fallback in v0.5]".into(),
            confidence: None,
            language: Some("auto".into()),
        })
    }
}

pub type SharedProvider = Arc<Mutex<Box<dyn SttProvider>>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_whisper_provider() {
        let provider = LocalWhisperProvider;
        assert_eq!(provider.name(), "local-whisper");

        let result = provider.transcribe(Path::new("/dev/null")).await.unwrap();
        assert!(result.text.contains("Placeholder"));
        assert_eq!(result.language, Some("auto".into()));
    }

    #[test]
    fn test_saaras_provider_name() {
        let provider = SaarasProvider::new(
            "https://api.sarvam.ai/speech-to-text".into(),
            Some("dummy".into()),
            "hi-IN".into(),
            true,
        );
        assert_eq!(provider.name(), "saaras-v3");
    }
}
