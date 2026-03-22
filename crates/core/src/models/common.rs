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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relationship_display() {
        assert_eq!(Relationship::Self_.to_string(), "self");
        assert_eq!(Relationship::Spouse.to_string(), "spouse");
        assert_eq!(Relationship::Child.to_string(), "child");
        assert_eq!(Relationship::Parent.to_string(), "parent");
        assert_eq!(Relationship::Sibling.to_string(), "sibling");
        assert_eq!(Relationship::Other.to_string(), "other");
    }

    #[test]
    fn test_relationship_from_str() {
        assert_eq!("self".parse::<Relationship>(), Ok(Relationship::Self_));
        assert_eq!("spouse".parse::<Relationship>(), Ok(Relationship::Spouse));
        assert_eq!("child".parse::<Relationship>(), Ok(Relationship::Child));
        assert!("invalid".parse::<Relationship>().is_err());
    }

    #[test]
    fn test_relationship_default() {
        assert_eq!(Relationship::default(), Relationship::Self_);
    }

    #[test]
    fn test_relationship_serde() {
        let rel = Relationship::Spouse;
        let json = serde_json::to_string(&rel).unwrap();
        assert_eq!(json, "\"spouse\"");

        let parsed: Relationship = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Relationship::Spouse);
    }

    #[test]
    fn test_gender_display() {
        assert_eq!(Gender::Male.to_string(), "male");
        assert_eq!(Gender::Female.to_string(), "female");
        assert_eq!(Gender::Other.to_string(), "other");
    }

    #[test]
    fn test_gender_from_str() {
        assert_eq!("male".parse::<Gender>(), Ok(Gender::Male));
        assert_eq!("m".parse::<Gender>(), Ok(Gender::Male));
        assert_eq!("female".parse::<Gender>(), Ok(Gender::Female));
        assert_eq!("f".parse::<Gender>(), Ok(Gender::Female));
        assert_eq!("MALE".parse::<Gender>(), Ok(Gender::Male)); // case insensitive
        assert!("invalid".parse::<Gender>().is_err());
    }

    #[test]
    fn test_gender_serde() {
        let gender = Gender::Male;
        let json = serde_json::to_string(&gender).unwrap();
        assert_eq!(json, "\"male\"");

        let parsed: Gender = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, Gender::Male);
    }

    #[test]
    fn test_blood_type_display() {
        assert_eq!(BloodType::A.to_string(), "A");
        assert_eq!(BloodType::B.to_string(), "B");
        assert_eq!(BloodType::Ab.to_string(), "AB");
        assert_eq!(BloodType::O.to_string(), "O");
    }

    #[test]
    fn test_blood_type_from_str() {
        assert_eq!("A".parse::<BloodType>(), Ok(BloodType::A));
        assert_eq!("a".parse::<BloodType>(), Ok(BloodType::A)); // case insensitive
        assert_eq!("AB".parse::<BloodType>(), Ok(BloodType::Ab));
        assert_eq!("ab".parse::<BloodType>(), Ok(BloodType::Ab));
        assert!("invalid".parse::<BloodType>().is_err());
    }

    #[test]
    fn test_blood_type_serde() {
        let bt = BloodType::Ab;
        let json = serde_json::to_string(&bt).unwrap();
        assert_eq!(json, "\"AB\"");

        let parsed: BloodType = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, BloodType::Ab);
    }

    #[test]
    fn test_attachment_type_display() {
        assert_eq!(AttachmentType::MedicalRecord.to_string(), "medical_record");
        assert_eq!(AttachmentType::LabReport.to_string(), "lab_report");
        assert_eq!(AttachmentType::Prescription.to_string(), "prescription");
        assert_eq!(AttachmentType::Imaging.to_string(), "imaging");
        assert_eq!(AttachmentType::Invoice.to_string(), "invoice");
        assert_eq!(AttachmentType::Other.to_string(), "other");
    }

    #[test]
    fn test_attachment_type_from_str() {
        assert_eq!("medical_record".parse::<AttachmentType>(), Ok(AttachmentType::MedicalRecord));
        assert_eq!("lab_report".parse::<AttachmentType>(), Ok(AttachmentType::LabReport));
        assert!("invalid".parse::<AttachmentType>().is_err());
    }

    #[test]
    fn test_attachment_type_default() {
        assert_eq!(AttachmentType::default(), AttachmentType::Other);
    }

    #[test]
    fn test_generate_id() {
        let id1 = generate_id();
        let id2 = generate_id();

        // IDs should be different
        assert_ne!(id1, id2);

        // IDs should be valid UUIDs
        assert!(uuid::Uuid::parse_str(&id1).is_ok());
        assert!(uuid::Uuid::parse_str(&id2).is_ok());
    }
}