//! Common types and utilities

use serde::{Deserialize, Serialize};

/// Relationship type for a person in the system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Relationship {
    Self_,
    Spouse,
    Child,
    Parent,
    Sibling,
    Other,
}

impl Default for Relationship {
    fn default() -> Self {
        Self::Self_
    }
}

impl std::fmt::Display for Relationship {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Relationship::Self_ => write!(f, "self"),
            Relationship::Spouse => write!(f, "spouse"),
            Relationship::Child => write!(f, "child"),
            Relationship::Parent => write!(f, "parent"),
            Relationship::Sibling => write!(f, "sibling"),
            Relationship::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for Relationship {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "self" => Ok(Relationship::Self_),
            "spouse" => Ok(Relationship::Spouse),
            "child" => Ok(Relationship::Child),
            "parent" => Ok(Relationship::Parent),
            "sibling" => Ok(Relationship::Sibling),
            "other" => Ok(Relationship::Other),
            _ => Err(format!("Invalid relationship: {}", s)),
        }
    }
}

/// Gender type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Gender {
    Male,
    Female,
    Other,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::Male => write!(f, "male"),
            Gender::Female => write!(f, "female"),
            Gender::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "male" | "m" => Ok(Gender::Male),
            "female" | "f" => Ok(Gender::Female),
            "other" => Ok(Gender::Other),
            _ => Err(format!("Invalid gender: {}", s)),
        }
    }
}

/// Blood type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum BloodType {
    A,
    B,
    Ab,
    O,
}

impl std::fmt::Display for BloodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BloodType::A => write!(f, "A"),
            BloodType::B => write!(f, "B"),
            BloodType::Ab => write!(f, "AB"),
            BloodType::O => write!(f, "O"),
        }
    }
}

impl std::str::FromStr for BloodType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(BloodType::A),
            "B" => Ok(BloodType::B),
            "AB" => Ok(BloodType::Ab),
            "O" => Ok(BloodType::O),
            _ => Err(format!("Invalid blood type: {}", s)),
        }
    }
}

/// Attachment type
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentType {
    MedicalRecord,
    LabReport,
    Prescription,
    Imaging,
    Invoice,
    Other,
}

impl Default for AttachmentType {
    fn default() -> Self {
        Self::Other
    }
}

impl std::fmt::Display for AttachmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttachmentType::MedicalRecord => write!(f, "medical_record"),
            AttachmentType::LabReport => write!(f, "lab_report"),
            AttachmentType::Prescription => write!(f, "prescription"),
            AttachmentType::Imaging => write!(f, "imaging"),
            AttachmentType::Invoice => write!(f, "invoice"),
            AttachmentType::Other => write!(f, "other"),
        }
    }
}

impl std::str::FromStr for AttachmentType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "medical_record" => Ok(AttachmentType::MedicalRecord),
            "lab_report" => Ok(AttachmentType::LabReport),
            "prescription" => Ok(AttachmentType::Prescription),
            "imaging" => Ok(AttachmentType::Imaging),
            "invoice" => Ok(AttachmentType::Invoice),
            "other" => Ok(AttachmentType::Other),
            _ => Err(format!("Invalid attachment type: {}", s)),
        }
    }
}

/// Generate a new UUID string
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}