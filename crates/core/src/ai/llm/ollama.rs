//! Ollama provider (local deployment)

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{AiError, ChatMessage, ExtractionContext, ExtractionResult, LlmProvider};

/// Ollama provider for local deployment
pub struct OllamaProvider {
    endpoint: String,
    model: String,
    timeout: Duration,
    client: Client,
}

impl OllamaProvider {
    /// Create a new Ollama provider
    pub fn new(endpoint: String, model: String, timeout: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap_or_default();

        Self {
            endpoint,
            model,
            timeout: Duration::from_secs(timeout),
            client,
        }
    }

    /// Create with default settings
    pub fn default_config() -> Self {
        Self::new(
            "http://127.0.0.1:11434".to_string(),
            "qwen2.5:7b".to_string(),
            120,
        )
    }

    /// Build the extraction prompt
    fn build_extraction_prompt(&self, context: &ExtractionContext) -> String {
        format!(
            r#"你是一个医疗文档分析助手。请分析以下医疗文档的OCR识别文本，提取关键信息。

文档类型提示: {}
患者姓名: {}

OCR文本:
```
{}
```

请提取以下信息并以JSON格式返回:
1. diagnosis: 诊断结果
2. chief_complaint: 主诉
3. treatment: 治疗方案
4. medications: 药物列表（数组，每项包含 name, dosage, frequency, duration, notes）
5. lab_results: 检查结果（数组，每项包含 name, value, unit, reference_range, is_abnormal）
6. follow_up: 复诊建议
7. summary: 病历摘要

如果某项信息不存在，请设为 null。
只返回JSON，不要其他解释。

```json
"#,
            context.document_type.as_deref().unwrap_or("未知"),
            context.person_name.as_deref().unwrap_or("未知"),
            context.ocr_text
        )
    }
}

#[async_trait]
impl LlmProvider for OllamaProvider {
    fn name(&self) -> &str {
        "ollama_local"
    }

    async fn is_available(&self) -> bool {
        match self.client.get(&format!("{}/api/tags", self.endpoint)).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    async fn extract(&self, context: ExtractionContext) -> Result<ExtractionResult, AiError> {
        let prompt = self.build_extraction_prompt(&context);

        let request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            format: Some("json".to_string()),
        };

        let response = self
            .client
            .post(&format!("{}/api/generate", self.endpoint))
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AiError::RequestFailed(format!(
                "Ollama request failed with status: {}",
                response.status()
            )));
        }

        let result: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        // Parse the JSON response
        let extraction: ExtractionResult = serde_json::from_str(&result.response)
            .unwrap_or_else(|_| ExtractionResult {
                visit_date: None,
                hospital: None,
                department: None,
                doctor: None,
                diagnosis: None,
                chief_complaint: None,
                treatment: None,
                medications: vec![],
                lab_results: vec![],
                follow_up: None,
                summary: Some(result.response),
                confidence: None,
            });

        Ok(extraction)
    }

    async fn summarize(&self, text: &str) -> Result<String, AiError> {
        let prompt = format!(
            r#"请用简洁的中文总结以下医疗文档内容：

{}
"#,
            text
        );

        let request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt,
            stream: false,
            format: None,
        };

        let response = self
            .client
            .post(&format!("{}/api/generate", self.endpoint))
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AiError::RequestFailed(format!(
                "Ollama request failed with status: {}",
                response.status()
            )));
        }

        let result: OllamaGenerateResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        Ok(result.response)
    }

    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, AiError> {
        let request = OllamaChatRequest {
            model: self.model.clone(),
            messages: messages
                .into_iter()
                .map(|m| OllamaMessage {
                    role: m.role,
                    content: m.content,
                })
                .collect(),
            stream: false,
        };

        let response = self
            .client
            .post(&format!("{}/api/chat", self.endpoint))
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AiError::RequestFailed(format!(
                "Ollama request failed with status: {}",
                response.status()
            )));
        }

        let result: OllamaChatResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        Ok(result.message.content)
    }
}

/// Ollama generate request
#[derive(Debug, Serialize)]
struct OllamaGenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    format: Option<String>,
}

/// Ollama generate response
#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    done: bool,
}

/// Ollama chat request
#[derive(Debug, Serialize)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<OllamaMessage>,
    stream: bool,
}

/// Ollama message
#[derive(Debug, Serialize, Deserialize)]
struct OllamaMessage {
    role: String,
    content: String,
}

/// Ollama chat response
#[derive(Debug, Deserialize)]
struct OllamaChatResponse {
    message: OllamaMessage,
}