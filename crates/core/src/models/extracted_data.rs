//! Extracted data model - represents AI-extracted structured data

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// OCR result entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    /// Unique identifier
    pub id: String,
    /// ID of the attachment this result belongs to
    pub attachment_id: String,
    /// Name of the OCR provider used
    pub ocr_provider: String,
    /// Recognized text content
    pub recognized_text: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: Option<f64>,
    /// Processing timestamp
    pub processed_at: DateTime<Utc>,
}

impl OcrResult {
    /// Create a new OCR result
    pub fn new(attachment_id: String, ocr_provider: String, recognized_text: String) -> Self {
        Self {
            id: super::common::generate_id(),
            attachment_id,
            ocr_provider,
            recognized_text,
            confidence: None,
            processed_at: Utc::now(),
        }
    }

    /// Set the confidence score
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = Some(confidence);
        self
    }
}

/// Extracted data entity - represents structured data extracted by LLM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedData {
    /// Unique identifier
    pub id: String,
    /// ID of the attachment this data was extracted from
    pub attachment_id: String,
    /// Name of the LLM provider used
    pub llm_provider: String,
    /// Type of extracted data
    pub data_type: ExtractedDataType,
    /// JSON content of the extracted data
    pub content: String,
    /// Confidence score (0.0 - 1.0)
    pub confidence: Option<f64>,
    /// Extraction timestamp
    pub extracted_at: DateTime<Utc>,
}

/// Types of data that can be extracted from medical documents
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtractedDataType {
    /// Vital signs (blood pressure, temperature, etc.)
    VitalSigns,
    /// Laboratory test results
    LabResults,
    /// Medications prescribed
    Medications,
    /// Diagnosis information
    Diagnosis,
    /// Follow-up instructions
    FollowUp,
    /// General summary
    Summary,
}

impl std::fmt::Display for ExtractedDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtractedDataType::VitalSigns => write!(f, "vital_signs"),
            ExtractedDataType::LabResults => write!(f, "lab_results"),
            ExtractedDataType::Medications => write!(f, "medications"),
            ExtractedDataType::Diagnosis => write!(f, "diagnosis"),
            ExtractedDataType::FollowUp => write!(f, "follow_up"),
            ExtractedDataType::Summary => write!(f, "summary"),
        }
    }
}

impl std::str::FromStr for ExtractedDataType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vital_signs" => Ok(ExtractedDataType::VitalSigns),
            "lab_results" => Ok(ExtractedDataType::LabResults),
            "medications" => Ok(ExtractedDataType::Medications),
            "diagnosis" => Ok(ExtractedDataType::Diagnosis),
            "follow_up" => Ok(ExtractedDataType::FollowUp),
            "summary" => Ok(ExtractedDataType::Summary),
            _ => Err(format!("Invalid extracted data type: {}", s)),
        }
    }
}

impl ExtractedData {
    /// Create a new extracted data entry
    pub fn new(
        attachment_id: String,
        llm_provider: String,
        data_type: ExtractedDataType,
        content: String,
    ) -> Self {
        Self {
            id: super::common::generate_id(),
            attachment_id,
            llm_provider,
            data_type,
            content,
            confidence: None,
            extracted_at: Utc::now(),
        }
    }

    /// Set the confidence score
    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = Some(confidence);
        self
    }
}

/// Structured medication information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Medication {
    /// Medication name
    pub name: String,
    /// Dosage (e.g., "200mg")
    pub dosage: Option<String>,
    /// Frequency (e.g., "3 times daily")
    pub frequency: Option<String>,
    /// Duration (e.g., "7 days")
    pub duration: Option<String>,
    /// Notes
    pub notes: Option<String>,
}

/// Structured lab result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LabResult {
    /// Test name
    pub name: String,
    /// Result value
    pub value: String,
    /// Unit (e.g., "mg/dL")
    pub unit: Option<String>,
    /// Reference range
    pub reference_range: Option<String>,
    /// Whether the result is abnormal
    pub is_abnormal: Option<bool>,
}

/// Structured vital signs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VitalSigns {
    /// Blood pressure (e.g., "120/80")
    pub blood_pressure: Option<String>,
    /// Heart rate (beats per minute)
    pub heart_rate: Option<i32>,
    /// Temperature (Celsius)
    pub temperature: Option<f64>,
    /// Weight (kg)
    pub weight: Option<f64>,
    /// Height (cm)
    pub height: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ocr_result_new() {
        let result = OcrResult::new(
            "attachment-123".to_string(),
            "qwen".to_string(),
            "识别的文本内容".to_string(),
        );

        assert_eq!(result.attachment_id, "attachment-123");
        assert_eq!(result.ocr_provider, "qwen");
        assert_eq!(result.recognized_text, "识别的文本内容");
        assert!(result.id.len() > 0);
        assert!(result.confidence.is_none());
    }

    #[test]
    fn test_ocr_result_with_confidence() {
        let result = OcrResult::new(
            "attachment-123".to_string(),
            "qwen".to_string(),
            "文本".to_string(),
        ).with_confidence(0.95);

        assert_eq!(result.confidence, Some(0.95));
    }

    #[test]
    fn test_extracted_data_type_display() {
        assert_eq!(ExtractedDataType::VitalSigns.to_string(), "vital_signs");
        assert_eq!(ExtractedDataType::LabResults.to_string(), "lab_results");
        assert_eq!(ExtractedDataType::Medications.to_string(), "medications");
        assert_eq!(ExtractedDataType::Diagnosis.to_string(), "diagnosis");
        assert_eq!(ExtractedDataType::FollowUp.to_string(), "follow_up");
        assert_eq!(ExtractedDataType::Summary.to_string(), "summary");
    }

    #[test]
    fn test_extracted_data_type_from_str() {
        assert_eq!("vital_signs".parse::<ExtractedDataType>(), Ok(ExtractedDataType::VitalSigns));
        assert_eq!("lab_results".parse::<ExtractedDataType>(), Ok(ExtractedDataType::LabResults));
        assert_eq!("medications".parse::<ExtractedDataType>(), Ok(ExtractedDataType::Medications));
        assert!("invalid".parse::<ExtractedDataType>().is_err());
    }

    #[test]
    fn test_extracted_data_type_serde() {
        let dt = ExtractedDataType::LabResults;
        let json = serde_json::to_string(&dt).unwrap();
        assert_eq!(json, "\"lab_results\"");

        let parsed: ExtractedDataType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, ExtractedDataType::LabResults);
    }

    #[test]
    fn test_extracted_data_new() {
        let data = ExtractedData::new(
            "attachment-123".to_string(),
            "kimi".to_string(),
            ExtractedDataType::Diagnosis,
            "{\"diagnosis\": \"感冒\"}".to_string(),
        );

        assert_eq!(data.attachment_id, "attachment-123");
        assert_eq!(data.llm_provider, "kimi");
        assert_eq!(data.data_type, ExtractedDataType::Diagnosis);
        assert_eq!(data.content, "{\"diagnosis\": \"感冒\"}");
        assert!(data.confidence.is_none());
    }

    #[test]
    fn test_extracted_data_with_confidence() {
        let data = ExtractedData::new(
            "attachment-123".to_string(),
            "kimi".to_string(),
            ExtractedDataType::Summary,
            "摘要内容".to_string(),
        ).with_confidence(0.88);

        assert_eq!(data.confidence, Some(0.88));
    }

    #[test]
    fn test_medication_serde() {
        let med = Medication {
            name: "阿莫西林".to_string(),
            dosage: Some("500mg".to_string()),
            frequency: Some("每日三次".to_string()),
            duration: Some("7天".to_string()),
            notes: Some("饭后服用".to_string()),
        };

        let json = serde_json::to_string(&med).unwrap();
        let parsed: Medication = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, "阿莫西林");
        assert_eq!(parsed.dosage, Some("500mg".to_string()));
    }

    #[test]
    fn test_lab_result_serde() {
        let lab = LabResult {
            name: "血糖".to_string(),
            value: "5.6".to_string(),
            unit: Some("mmol/L".to_string()),
            reference_range: Some("3.9-6.1".to_string()),
            is_abnormal: Some(false),
        };

        let json = serde_json::to_string(&lab).unwrap();
        let parsed: LabResult = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.name, "血糖");
        assert_eq!(parsed.value, "5.6");
        assert_eq!(parsed.is_abnormal, Some(false));
    }

    #[test]
    fn test_vital_signs_serde() {
        let vs = VitalSigns {
            blood_pressure: Some("120/80".to_string()),
            heart_rate: Some(72),
            temperature: Some(36.5),
            weight: Some(65.0),
            height: Some(170.0),
        };

        let json = serde_json::to_string(&vs).unwrap();
        let parsed: VitalSigns = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed.blood_pressure, Some("120/80".to_string()));
        assert_eq!(parsed.heart_rate, Some(72));
    }
}