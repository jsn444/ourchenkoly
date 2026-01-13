// audio/transcription/openai_whisper_provider.rs
//
// OpenAI Whisper API transcription provider for fast cloud-based transcription.

use super::provider::{TranscriptionError, TranscriptionProvider, TranscriptResult};
use async_trait::async_trait;
use log::{debug, error, info};
use reqwest::multipart;
use std::sync::Arc;
use tokio::sync::RwLock;

/// OpenAI Whisper API transcription provider
pub struct OpenAIWhisperProvider {
    api_key: Arc<RwLock<Option<String>>>,
    client: reqwest::Client,
}

impl OpenAIWhisperProvider {
    pub fn new() -> Self {
        Self {
            api_key: Arc::new(RwLock::new(None)),
            client: reqwest::Client::new(),
        }
    }

    pub async fn set_api_key(&self, key: String) {
        let mut api_key = self.api_key.write().await;
        *api_key = Some(key);
        info!("OpenAI Whisper API key configured");
    }

    pub async fn has_api_key(&self) -> bool {
        let api_key = self.api_key.read().await;
        api_key.is_some() && !api_key.as_ref().unwrap().is_empty()
    }

    /// Convert f32 audio samples to WAV bytes
    fn samples_to_wav(samples: &[f32], sample_rate: u32) -> Vec<u8> {
        let num_samples = samples.len();
        let byte_rate = sample_rate * 2; // 16-bit mono
        let block_align = 2u16;
        let bits_per_sample = 16u16;
        let data_size = (num_samples * 2) as u32;
        let file_size = 36 + data_size;

        let mut wav = Vec::with_capacity(44 + num_samples * 2);

        // RIFF header
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&file_size.to_le_bytes());
        wav.extend_from_slice(b"WAVE");

        // fmt chunk
        wav.extend_from_slice(b"fmt ");
        wav.extend_from_slice(&16u32.to_le_bytes()); // chunk size
        wav.extend_from_slice(&1u16.to_le_bytes()); // audio format (PCM)
        wav.extend_from_slice(&1u16.to_le_bytes()); // num channels (mono)
        wav.extend_from_slice(&sample_rate.to_le_bytes()); // sample rate
        wav.extend_from_slice(&byte_rate.to_le_bytes()); // byte rate
        wav.extend_from_slice(&block_align.to_le_bytes()); // block align
        wav.extend_from_slice(&bits_per_sample.to_le_bytes()); // bits per sample

        // data chunk
        wav.extend_from_slice(b"data");
        wav.extend_from_slice(&data_size.to_le_bytes());

        // Convert f32 samples to i16
        for &sample in samples {
            let clamped = sample.clamp(-1.0, 1.0);
            let scaled = (clamped * 32767.0) as i16;
            wav.extend_from_slice(&scaled.to_le_bytes());
        }

        wav
    }
}

#[async_trait]
impl TranscriptionProvider for OpenAIWhisperProvider {
    async fn transcribe(
        &self,
        audio: Vec<f32>,
        language: Option<String>,
    ) -> std::result::Result<TranscriptResult, TranscriptionError> {
        let api_key = {
            let key = self.api_key.read().await;
            match key.as_ref() {
                Some(k) if !k.is_empty() => k.clone(),
                _ => {
                    return Err(TranscriptionError::EngineFailed(
                        "OpenAI API key not configured".to_string(),
                    ))
                }
            }
        };

        // Check minimum audio length (at least 0.1 seconds at 16kHz)
        let minimum_samples = 1600;
        if audio.len() < minimum_samples {
            return Err(TranscriptionError::AudioTooShort {
                samples: audio.len(),
                minimum: minimum_samples,
            });
        }

        debug!(
            "OpenAI Whisper: Transcribing {} samples ({:.1}s)",
            audio.len(),
            audio.len() as f32 / 16000.0
        );

        // Convert to WAV (OpenAI expects 16kHz audio)
        let wav_data = Self::samples_to_wav(&audio, 16000);

        // Create multipart form
        let file_part = multipart::Part::bytes(wav_data)
            .file_name("audio.wav")
            .mime_str("audio/wav")
            .map_err(|e| TranscriptionError::EngineFailed(format!("Failed to create form: {}", e)))?;

        let mut form = multipart::Form::new()
            .part("file", file_part)
            .text("model", "whisper-1");

        // Add language hint if provided
        if let Some(lang) = language {
            form = form.text("language", lang);
        }

        // Send request to OpenAI
        let response = self
            .client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .header("Authorization", format!("Bearer {}", api_key))
            .multipart(form)
            .send()
            .await
            .map_err(|e| TranscriptionError::EngineFailed(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_body = response.text().await.unwrap_or_default();
            error!("OpenAI Whisper API error {}: {}", status, error_body);
            return Err(TranscriptionError::EngineFailed(format!(
                "API error {}: {}",
                status, error_body
            )));
        }

        // Parse response
        let result: serde_json::Value = response
            .json()
            .await
            .map_err(|e| TranscriptionError::EngineFailed(format!("Failed to parse response: {}", e)))?;

        let text = result["text"]
            .as_str()
            .unwrap_or("")
            .trim()
            .to_string();

        info!(
            "OpenAI Whisper: Transcribed {} chars from {:.1}s audio",
            text.len(),
            audio.len() as f32 / 16000.0
        );

        Ok(TranscriptResult {
            text,
            confidence: Some(1.0), // OpenAI doesn't return confidence, assume high
            is_partial: false,
        })
    }

    async fn is_model_loaded(&self) -> bool {
        // Cloud API is always "loaded" if we have an API key
        self.has_api_key().await
    }

    async fn get_current_model(&self) -> Option<String> {
        if self.has_api_key().await {
            Some("whisper-1 (OpenAI API)".to_string())
        } else {
            None
        }
    }

    fn provider_name(&self) -> &'static str {
        "OpenAI Whisper"
    }
}
