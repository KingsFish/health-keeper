//! AI module - OCR and LLM capabilities

mod llm;
mod ocr;

pub use llm::{AnthropicProvider, LlmProvider, LlmRegistry, OllamaProvider};
pub use ocr::{OcrProvider, OcrRegistry, PaddleOcrProvider, VisionOcrProvider};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// AI error type
#[derive(thiserror::Error, Debug)]
pub enum AiError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),

    #[error("Provider not enabled: {0}")]
    ProviderNotEnabled(String),

    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// OCR result from provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResultData {
    /// Recognized text
    pub text: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: Option<f64>,
}

/// Context for LLM extraction
#[derive(Debug, Clone)]
pub struct ExtractionContext {
    /// OCR recognized text
    pub ocr_text: String,
    /// Document type hint
    pub document_type: Option<String>,
    /// Person's name for context
    pub person_name: Option<String>,
}

/// Result from LLM extraction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractionResult {
    /// Visit date
    #[serde(default)]
    pub visit_date: Option<String>,
    /// Hospital name
    #[serde(default)]
    pub hospital: Option<String>,
    /// Department
    #[serde(default)]
    pub department: Option<String>,
    /// Doctor name
    #[serde(default)]
    pub doctor: Option<String>,
    /// Diagnosis information
    #[serde(default)]
    pub diagnosis: Option<String>,
    /// Chief complaint
    #[serde(default)]
    pub chief_complaint: Option<String>,
    /// Treatment prescribed
    #[serde(default)]
    pub treatment: Option<String>,
    /// Medications
    #[serde(default, deserialize_with = "deserialize_null_vec")]
    pub medications: Vec<MedicationInfo>,
    /// Lab results
    #[serde(default, deserialize_with = "deserialize_null_vec")]
    pub lab_results: Vec<LabResultInfo>,
    /// Follow-up instructions
    #[serde(default)]
    pub follow_up: Option<String>,
    /// Summary
    #[serde(default)]
    pub summary: Option<String>,
    /// Confidence score
    #[serde(default)]
    pub confidence: Option<f64>,
}

/// Helper function to deserialize null as empty vec
fn deserialize_null_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: serde::Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

/// Medication information extracted by LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MedicationInfo {
    pub name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub duration: Option<String>,
    pub notes: Option<String>,
}

/// Lab result information extracted by LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabResultInfo {
    pub name: String,
    pub value: String,
    pub unit: Option<String>,
    pub reference_range: Option<String>,
    pub is_abnormal: Option<bool>,
}