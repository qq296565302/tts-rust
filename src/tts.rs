use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Serialize)]
pub struct TtsRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub audio: AudioConfig,
}

#[derive(Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Clone, Serialize)]
pub struct AudioConfig {
    pub format: String,
    pub voice: String,
}

#[derive(Clone, Deserialize)]
pub struct TtsResponse {
    pub error: Option<ApiError>,
    pub choices: Option<Vec<Choice>>,
}

#[derive(Clone, Deserialize)]
pub struct ApiError {
    pub message: String,
}

#[derive(Clone, Deserialize)]
pub struct Choice {
    pub message: Option<ResponseMessage>,
}

#[derive(Clone, Deserialize)]
pub struct ResponseMessage {
    pub audio: Option<AudioData>,
}

#[derive(Clone, Deserialize)]
pub struct AudioData {
    pub data: Option<String>,
}

pub struct TtsClient {
    client: Client,
    api_key: String,
    api_base: String,
    model: String,
}

impl TtsClient {
    pub fn new(api_key: String, api_base: String, model: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_base,
            model,
        }
    }

    pub async fn synthesize(
        &self,
        text: &str,
        voice: &str,
    ) -> Result<Vec<u8>> {
        if self.api_key.is_empty() {
            return Err(anyhow!("API密钥未配置"));
        }

        let url = format!("{}/chat/completions", self.api_base);
        
        let request = TtsRequest {
            model: self.model.clone(),
            messages: vec![
                Message {
                    role: "user".to_string(),
                    content: "请朗读以下内容".to_string(),
                },
                Message {
                    role: "assistant".to_string(),
                    content: text.to_string(),
                },
            ],
            audio: AudioConfig {
                format: "wav".to_string(),
                voice: voice.to_string(),
            },
        };

        let response = self
            .client
            .post(&url)
            .header("api-key", &self.api_key)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow!("API请求失败: {} - {}", status, error_text));
        }

        let json_response: TtsResponse = response.json().await?;
        
        if let Some(err) = json_response.error {
            return Err(anyhow!("API错误: {}", err.message));
        }
        
        if let Some(choices) = json_response.choices {
            if let Some(choice) = choices.first() {
                if let Some(msg) = &choice.message {
                    if let Some(audio) = &msg.audio {
                        if let Some(base64_data) = &audio.data {
                            use base64::{Engine as _, engine::general_purpose};
                            let audio_data = general_purpose::STANDARD
                                .decode(base64_data)
                                .map_err(|e| anyhow!("Base64解码失败: {}", e))?;
                            return Ok(audio_data);
                        }
                    }
                }
            }
        }
        
        Err(anyhow!("API响应中未找到音频数据"))
    }

    pub async fn synthesize_to_file(
        &self,
        text: &str,
        voice: &str,
        output_path: &PathBuf,
    ) -> Result<()> {
        let audio_data = self.synthesize(text, voice).await?;
        
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        
        tokio::fs::write(output_path, &audio_data).await?;
        Ok(())
    }
}

pub fn get_available_voices() -> Vec<&'static str> {
    vec!["mimo_default", "default_zh", "default_en"]
}

pub fn generate_output_filename(extension: &str) -> String {
    let now = chrono::Local::now();
    format!("tts_{}.{}", now.format("%Y%m%d_%H%M%S"), extension)
}

pub fn generate_output_filename_with_index(extension: &str, index: usize) -> String {
    let now = chrono::Local::now();
    format!("tts_{}_{}.{}", now.format("%Y%m%d_%H%M%S"), index, extension)
}
