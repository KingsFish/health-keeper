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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MedicationInfo {
    pub name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub duration: Option<String>,
    pub notes: Option<String>,
}

/// Lab result information extracted by LLM
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabResultInfo {
    pub name: String,
    pub value: String,
    pub unit: Option<String>,
    pub reference_range: Option<String>,
    pub is_abnormal: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_error_display() {
        let err = AiError::ProviderNotFound("test".to_string());
        assert!(err.to_string().contains("Provider not found"));

        let err = AiError::ProviderNotEnabled("test".to_string());
        assert!(err.to_string().contains("Provider not enabled"));

        let err = AiError::RequestFailed("timeout".to_string());
        assert!(err.to_string().contains("Request failed"));

        let err = AiError::InvalidResponse("bad json".to_string());
        assert!(err.to_string().contains("Invalid response"));

        let err = AiError::Configuration("missing key".to_string());
        assert!(err.to_string().contains("Configuration error"));
    }

    #[test]
    fn test_ocr_result_data_serde() {
        let data = OcrResultData {
            text: "识别的文本".to_string(),
            confidence: Some(0.95),
        };

        let json = serde_json::to_string(&data).unwrap();
        let parsed: OcrResultData = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.text, "识别的文本");
        assert_eq!(parsed.confidence, Some(0.95));
    }

    #[test]
    fn test_extraction_context() {
        let ctx = ExtractionContext {
            ocr_text: "病历内容".to_string(),
            document_type: Some("门诊病历".to_string()),
            person_name: Some("张三".to_string()),
        };

        assert_eq!(ctx.ocr_text, "病历内容");
        assert_eq!(ctx.document_type, Some("门诊病历".to_string()));
    }

    #[test]
    fn test_extraction_result_serde() {
        let result = ExtractionResult {
            visit_date: Some("2026-03-22".to_string()),
            hospital: Some("北京医院".to_string()),
            department: Some("内科".to_string()),
            doctor: Some("王医生".to_string()),
            diagnosis: Some("感冒".to_string()),
            chief_complaint: Some("头痛".to_string()),
            treatment: Some("休息".to_string()),
            medications: vec![MedicationInfo {
                name: "阿莫西林".to_string(),
                dosage: Some("500mg".to_string()),
                frequency: Some("每日三次".to_string()),
                duration: Some("7天".to_string()),
                notes: None,
            }],
            lab_results: vec![],
            follow_up: Some("一周后复诊".to_string()),
            summary: Some("症状缓解".to_string()),
            confidence: Some(0.9),
        };

        let json = serde_json::to_string(&result).unwrap();

        // Verify JSON contains expected fields
        assert!(json.contains("\"hospital\":\"北京医院\""));
        assert!(json.contains("\"diagnosis\":\"感冒\""));
        assert!(json.contains("\"阿莫西林\""));

        // Parse back
        let parsed: ExtractionResult = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.hospital, Some("北京医院".to_string()));
        assert_eq!(parsed.medications.len(), 1);
        assert_eq!(parsed.medications[0].name, "阿莫西林");
    }

    #[test]
    fn test_extraction_result_null_medications() {
        // Test that null medications/labs are deserialized as empty vec
        let json = r#"{
            "visit_date": "2026-03-22",
            "medications": null,
            "lab_results": null
        }"#;

        let result: ExtractionResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.medications, Vec::<MedicationInfo>::new());
        assert_eq!(result.lab_results, Vec::<LabResultInfo>::new());
    }

    #[test]
    fn test_extraction_result_defaults() {
        let json = r#"{}"#;

        let result: ExtractionResult = serde_json::from_str(json).unwrap();
        assert!(result.visit_date.is_none());
        assert!(result.hospital.is_none());
        assert!(result.medications.is_empty());
        assert!(result.lab_results.is_empty());
    }

    #[test]
    fn test_medication_info_serde() {
        let med = MedicationInfo {
            name: "布洛芬".to_string(),
            dosage: Some("200mg".to_string()),
            frequency: Some("需要时".to_string()),
            duration: None,
            notes: Some("饭后服用".to_string()),
        };

        let json = serde_json::to_string(&med).unwrap();
        let parsed: MedicationInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, "布洛芬");
        assert_eq!(parsed.dosage, Some("200mg".to_string()));
    }

    #[test]
    fn test_lab_result_info_serde() {
        let lab = LabResultInfo {
            name: "白细胞".to_string(),
            value: "8.5".to_string(),
            unit: Some("10^9/L".to_string()),
            reference_range: Some("4-10".to_string()),
            is_abnormal: Some(false),
        };

        let json = serde_json::to_string(&lab).unwrap();
        let parsed: LabResultInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, "白细胞");
        assert_eq!(parsed.is_abnormal, Some(false));
    }
}