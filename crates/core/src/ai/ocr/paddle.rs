//! PaddleOCR provider (local deployment)

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::{AiError, OcrProvider, OcrResultData};

/// PaddleOCR provider for local deployment
pub struct PaddleOcrProvider {
    endpoint: String,
    timeout: Duration,
    client: Client,
}

impl PaddleOcrProvider {
    /// Create a new PaddleOCR provider
    pub fn new(endpoint: String, timeout: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout))
            .build()
            .unwrap_or_default();

        Self {
            endpoint,
            timeout: Duration::from_secs(timeout),
            client,
        }
    }

    /// Create with default settings
    pub fn default_config() -> Self {
        Self::new("http://127.0.0.1:8868".to_string(), 30)
    }
}

#[async_trait]
impl OcrProvider for PaddleOcrProvider {
    fn name(&self) -> &str {
        "paddle_local"
    }

    async fn is_available(&self) -> bool {
        match self.client.get(&format!("{}/health", self.endpoint)).send().await {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }

    async fn recognize_image(&self, image_data: &[u8]) -> Result<OcrResultData, AiError> {
        // Encode image to base64
        let base64_image = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_data);

        let request = PaddleOcrRequest {
            image: base64_image,
        };

        let response = self
            .client
            .post(&format!("{}/predict/ocr_system", self.endpoint))
            .json(&request)
            .send()
            .await
            .map_err(|e| AiError::RequestFailed(e.to_string()))?;

        if !response.status().is_success() {
            return Err(AiError::RequestFailed(format!(
                "OCR request failed with status: {}",
                response.status()
            )));
        }

        let result: PaddleOcrResponse = response
            .json()
            .await
            .map_err(|e| AiError::InvalidResponse(e.to_string()))?;

        // Parse the result
        let text = result
            .results
            .map(|r| r.into_iter().map(|item| item.text).collect::<Vec<_>>().join("\n"))
            .unwrap_or_default();

        Ok(OcrResultData {
            text,
            confidence: None,
        })
    }

    async fn recognize_pdf(&self, pdf_data: &[u8]) -> Result<Vec<OcrResultData>, AiError> {
        // For PDF, we need to convert to images first
        // This is a placeholder - in a real implementation, we would use a PDF library
        // to convert pages to images and then OCR each image

        // For now, return a single result with a message
        Ok(vec![OcrResultData {
            text: "PDF OCR not yet implemented. Please convert to image first.".to_string(),
            confidence: None,
        }])
    }
}

/// PaddleOCR request structure
#[derive(Debug, Serialize)]
struct PaddleOcrRequest {
    image: String,
}

/// PaddleOCR response structure
#[derive(Debug, Deserialize)]
struct PaddleOcrResponse {
    results: Option<Vec<PaddleOcrResult>>,
}

/// Single OCR result item
#[derive(Debug, Deserialize)]
struct PaddleOcrResult {
    text: String,
    confidence: Option<f64>,
}