//! Vision-based OCR provider using multimodal LLM

use async_trait::async_trait;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{AiError, OcrProvider, OcrResultData};

/// Vision-based OCR provider using multimodal LLM (qwen, gpt-4-vision, etc.)
pub struct VisionOcrProvider {
    name: String,
    endpoint: String,
    model: String,
    api_key: String,
    timeout: Duration,
    client: Client,
}

impl VisionOcrProvider {
    /// Create a new Vision OCR provider
    pub fn new(name: String, endpoint: String, model: String, api_key: String, timeout: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .http1_only()
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
}

#[async_trait]
impl OcrProvider for VisionOcrProvider {
    fn name(&self) -> &str {
        &self.name
    }

    async fn is_available(&self) -> bool {
        true
    }

    async fn recognize_image(&self, image_data: &[u8]) -> Result<OcrResultData, AiError> {
        // Encode image to base64
        let base64_image = base64::engine::general_purpose::STANDARD.encode(image_data);

        // Detect image format from magic bytes
        let mime_type = detect_image_type(image_data);

        let request = VisionRequest {
            model: self.model.clone(),
            max_tokens: 4096,
            messages: vec![VisionMessage {
                role: "user".to_string(),
                content: vec![
                    ContentBlock::Text {
                        text: r#"你是一个专业的医疗文档 OCR 识别助手。请仔细识别这张医疗文档图片中的所有文字内容。

识别要求：
1. 保持原始格式，包括表格、列表、分段
2. 准确识别医学术语、药品名称、检查指标
3. 对于表格，使用制表符或空格保持列对齐
4. 对于手写内容，尽量识别并标注[手写]
5. 数字和单位要准确（如血压 120/80、体温 36.5℃）
6. 如有印章或水印，标注[印章内容]或[水印]

输出格式：只输出识别的文字，不要添加任何解释或说明。"#.to_string(),
                    },
                    ContentBlock::Image {
                        source: ImageSource {
                            source_type: "url".to_string(),
                            url: format!("data:{};base64,{}", mime_type, base64_image),
                        },
                    },
                ],
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
                "Vision OCR request failed with status {}: {}",
                status, body
            )));
        }

        let result: VisionResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        // Extract text from response
        let text = result
            .content
            .iter()
            .find(|c| matches!(c, ContentBlockResponse::Text { .. }))
            .map(|c| {
                if let ContentBlockResponse::Text { text } = c {
                    text.clone()
                } else {
                    String::new()
                }
            })
            .unwrap_or_default();

        Ok(OcrResultData {
            text,
            confidence: None,
        })
    }

    async fn recognize_pdf(&self, pdf_data: &[u8]) -> Result<Vec<OcrResultData>, AiError> {
        // For PDF, convert to base64 and send
        let base64_pdf = base64::engine::general_purpose::STANDARD.encode(pdf_data);

        let request = VisionRequest {
            model: self.model.clone(),
            max_tokens: 8192,
            messages: vec![VisionMessage {
                role: "user".to_string(),
                content: vec![
                    ContentBlock::Text {
                        text: r#"你是一个专业的医疗文档 OCR 识别助手。请仔细识别这份医疗文档PDF中的所有文字内容。

识别要求：
1. 保持原始格式，包括表格、列表、分段
2. 准确识别医学术语、药品名称、检查指标
3. 对于表格，使用制表符或空格保持列对齐
4. 对于手写内容，尽量识别并标注[手写]
5. 数字和单位要准确（如血压 120/80、体温 36.5℃）
6. 如有印章或水印，标注[印章内容]或[水印]
7. 如有多页，用 === 第 N 页 === 分隔

输出格式：只输出识别的文字，不要添加任何解释或说明。"#.to_string(),
                    },
                    ContentBlock::Image {
                        source: ImageSource {
                            source_type: "url".to_string(),
                            url: format!("data:application/pdf;base64,{}", base64_pdf),
                        },
                    },
                ],
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
                "Vision OCR request failed with status {}: {}",
                status, body
            )));
        }

        let result: VisionResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        let text = result
            .content
            .iter()
            .find(|c| matches!(c, ContentBlockResponse::Text { .. }))
            .map(|c| {
                if let ContentBlockResponse::Text { text } = c {
                    text.clone()
                } else {
                    String::new()
                }
            })
            .unwrap_or_default();

        Ok(vec![OcrResultData {
            text,
            confidence: None,
        }])
    }
}

/// Detect image type from magic bytes
fn detect_image_type(data: &[u8]) -> &'static str {
    if data.len() < 4 {
        return "image/jpeg";
    }

    match &data[0..4] {
        [0xFF, 0xD8, 0xFF, _] => "image/jpeg",
        [0x89, 0x50, 0x4E, 0x47] => "image/png",
        [0x47, 0x49, 0x46, _] => "image/gif",
        [0x52, 0x49, 0x46, 0x46] => "image/webp", // WebP starts with RIFF
        _ => "image/jpeg",
    }
}

/// Vision API request structure
#[derive(Debug, Serialize)]
struct VisionRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<VisionMessage>,
}

/// Vision message with mixed content
#[derive(Debug, Serialize)]
struct VisionMessage {
    role: String,
    content: Vec<ContentBlock>,
}

/// Content block (text or image)
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
enum ContentBlock {
    Text { text: String },
    Image { source: ImageSource },
}

/// Image source
#[derive(Debug, Serialize)]
struct ImageSource {
    #[serde(rename = "type")]
    source_type: String,
    url: String,
}

/// Vision API response
#[derive(Debug, Deserialize)]
struct VisionResponse {
    content: Vec<ContentBlockResponse>,
}

/// Response content block
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum ContentBlockResponse {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
}