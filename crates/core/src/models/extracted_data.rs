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