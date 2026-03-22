//! Anthropic-compatible API provider

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{AiError, ChatMessage, ExtractionContext, ExtractionResult, LlmProvider};

/// Anthropic-compatible API provider (works with DashScope, etc.)
pub struct AnthropicProvider {
    name: String,
    endpoint: String,
    model: String,
    api_key: String,
    timeout: Duration,
    client: Client,
}

impl AnthropicProvider {
    /// Create a new Anthropic-compatible provider
    pub fn new(name: String, endpoint: String, model: String, api_key: String, timeout: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .http1_only()  // Force HTTP/1.1 for better compatibility
            .build()
            .unwrap_or_default();

        Self {
            name,
            endpoint,
            model,
            api_key,
            timeout: Duration::from_secs(timeout),
            client,
        }
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
1. visit_date: 就诊日期（格式: YYYY-MM-DD，如2026-03-22）
2. hospital: 医院名称
3. department: 科室
4. doctor: 医生姓名
5. diagnosis: 诊断结果
6. chief_complaint: 主诉
7. treatment: 治疗方案
8. medications: 药物列表（数组，每项包含 name, dosage, frequency, duration, notes）
9. lab_results: 检查结果（数组，每项包含 name, value, unit, reference_range, is_abnormal）
10. follow_up: 复诊建议
11. summary: 病历摘要

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
impl LlmProvider for AnthropicProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn is_available(&self) -> bool {
        // Always return true for cloud providers
        true
    }

    async fn extract(&self, context: ExtractionContext) -> Result<ExtractionResult, AiError> {
        let prompt = self.build_extraction_prompt(&context);

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
            .post(&format!("{}/v1/messages", self.endpoint))
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("User-Agent", "curl/8.0")
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AiError::RequestFailed(format!(
                "API request failed with status {}: {}",
                status, body
            )));
        }

        let result: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        // Extract text from response
        let text = result
            .content
            .iter()
            .find(|c| c.content_type == "text")
            .map(|c| c.text.clone())
            .unwrap_or_default();

        // Strip markdown code blocks if present
        let json_text = text
            .trim()
            .strip_prefix("```json")
            .or_else(|| text.trim().strip_prefix("```"))
            .and_then(|s| s.strip_suffix("```"))
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| text.clone());

        // Parse the JSON response
        let extraction: ExtractionResult = serde_json::from_str(&json_text)
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
                summary: Some(json_text.clone()),
                confidence: None,
            });

        Ok(extraction)
    }

    async fn summarize(&self, text: &str) -> Result<String, AiError> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 2048,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: format!(
                    "请用简洁的中文总结以下医疗文档内容：\n\n{}",
                    text
                ),
            }],
        };

        let response = self
            .client
            .post(&format!("{}/v1/messages", self.endpoint))
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("User-Agent", "curl/8.0")
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AiError::RequestFailed(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }

        let result: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        Ok(result
            .content
            .iter()
            .find(|c| c.content_type == "text")
            .map(|c| c.text.clone())
            .unwrap_or_default())
    }

    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, AiError> {
        let anthropic_messages: Vec<AnthropicMessage> = messages
            .into_iter()
            .map(|m| AnthropicMessage {
                role: m.role,
                content: m.content,
            })
            .collect();

        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: anthropic_messages,
        };

        let response = self
            .client
            .post(&format!("{}/v1/messages", self.endpoint))
            .header("Content-Type", "application/json")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("User-Agent", "curl/8.0")
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AiError::RequestFailed(format!(
                "API request failed with status: {}",
                response.status()
            )));
        }

        let result: AnthropicResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        Ok(result
            .content
            .iter()
            .find(|c| c.content_type == "text")
            .map(|c| c.text.clone())
            .unwrap_or_default())
    }
}

/// Anthropic API request structure
#[derive(Debug, Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
}

/// Anthropic message
#[derive(Debug, Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

/// Anthropic API response structure
#[derive(Debug, Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

/// Content block in response
#[derive(Debug, Deserialize)]
struct AnthropicContent {
    #[serde(rename = "type")]
    content_type: String,
    #[serde(default)]
    text: String,
}