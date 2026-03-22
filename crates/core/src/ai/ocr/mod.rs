//! OCR providers

mod paddle;
mod vision;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub use paddle::PaddleOcrProvider;
pub use vision::VisionOcrProvider;

use super::{AiError, OcrResultData};

/// OCR provider trait
#[async_trait]
pub trait OcrProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;

    /// Check if the provider is available
    async fn is_available(&self) -> bool;

    /// Recognize text from an image (bytes)
    async fn recognize_image(&self, image_data: &[u8]) -> Result<OcrResultData, AiError>;

    /// Recognize text from a PDF (bytes)
    async fn recognize_pdf(&self, pdf_data: &[u8]) -> Result<Vec<OcrResultData>, AiError>;
}

/// OCR provider configuration
#[derive(Debug, Clone)]
pub struct OcrProviderConfig {
    pub enabled: bool,
    pub endpoint: Option<String>,
    pub api_key: Option<String>,
    pub secret_key: Option<String>,
    pub timeout: u64,
}

/// Registry for OCR providers
pub struct OcrRegistry {
    providers: HashMap<String, (bool, Arc<dyn OcrProvider>)>,
    default: Option<String>,
}

impl OcrRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default: None,
        }
    }

    /// Register a provider
    pub fn register<P: OcrProvider + 'static>(&mut self, provider: P, enabled: bool) {
        let name = provider.name().to_string();
        self.providers.insert(name.clone(), (enabled, Arc::new(provider)));
    }

    /// Set the default provider
    pub fn set_default(&mut self, name: &str) -> Result<(), AiError> {
        if self.providers.contains_key(name) {
            self.default = Some(name.to_string());
            Ok(())
        } else {
            Err(AiError::ProviderNotFound(name.to_string()))
        }
    }

    /// Get a provider by name
    pub fn get(&self, name: Option<&str>) -> Result<Arc<dyn OcrProvider>, AiError> {
        let provider_name = name
            .or(self.default.as_deref())
            .ok_or_else(|| AiError::ProviderNotFound("No default provider set".to_string()))?;

        let (enabled, provider) = self
            .providers
            .get(provider_name)
            .ok_or_else(|| AiError::ProviderNotFound(provider_name.to_string()))?;

        if !enabled {
            return Err(AiError::ProviderNotEnabled(provider_name.to_string()));
        }

        Ok(provider.clone())
    }

    /// List all registered providers
    pub fn list(&self) -> Vec<(&str, bool)> {
        self.providers
            .iter()
            .map(|(name, (enabled, _))| (name.as_str(), *enabled))
            .collect()
    }
}

impl Default for OcrRegistry {
    fn default() -> Self {
        Self::new()
    }
}