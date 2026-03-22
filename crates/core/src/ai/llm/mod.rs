//! LLM providers

mod anthropic;
mod ollama;

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

pub use anthropic::AnthropicProvider;
pub use ollama::OllamaProvider;

use super::{AiError, ExtractionContext, ExtractionResult};

/// LLM provider trait
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Provider name
    fn name(&self) -> &str;

    /// Check if the provider is available
    async fn is_available(&self) -> bool;

    /// Extract structured data from OCR text
    async fn extract(&self, context: ExtractionContext) -> Result<ExtractionResult, AiError>;

    /// Summarize text
    async fn summarize(&self, text: &str) -> Result<String, AiError>;

    /// Chat completion (for interactive use)
    async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, AiError>;
}

/// Chat message structure
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }
}

/// LLM provider configuration
#[derive(Debug, Clone)]
pub struct LlmProviderConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub model: String,
    pub api_key: Option<String>,
    pub timeout: u64,
}

/// Registry for LLM providers
pub struct LlmRegistry {
    providers: HashMap<String, (bool, Arc<dyn LlmProvider>)>,
    default: Option<String>,
}

impl LlmRegistry {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
            default: None,
        }
    }

    /// Register a provider
    pub fn register<P: LlmProvider + 'static>(&mut self, provider: P, enabled: bool) {
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
    pub fn get(&self, name: Option<&str>) -> Result<Arc<dyn LlmProvider>, AiError> {
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

impl Default for LlmRegistry {
    fn default() -> Self {
        Self::new()
    }
}