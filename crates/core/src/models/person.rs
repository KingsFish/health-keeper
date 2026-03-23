//! Person model - represents a person's health profile

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::common::{BloodType, DiseaseStatus, DrinkingStatus, Gender, Relationship, SmokingStatus};

// ============================================================================
// Health Information Structures
// ============================================================================

/// Chronic disease record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronicDisease {
    /// Disease name (e.g., "高血压")
    pub name: String,
    /// Date of diagnosis
    pub diagnosed_date: Option<NaiveDate>,
    /// Current status
    pub status: DiseaseStatus,
    /// Additional notes
    pub notes: Option<String>,
}

impl ChronicDisease {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            diagnosed_date: None,
            status: DiseaseStatus::default(),
            notes: None,
        }
    }

    pub fn with_diagnosed_date(mut self, date: NaiveDate) -> Self {
        self.diagnosed_date = Some(date);
        self
    }

    pub fn with_status(mut self, status: DiseaseStatus) -> Self {
        self.status = status;
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Past surgery record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastSurgery {
    /// Surgery name
    pub name: String,
    /// Date of surgery
    pub date: Option<NaiveDate>,
    /// Hospital where surgery was performed
    pub hospital: Option<String>,
    /// Additional notes
    pub notes: Option<String>,
}

impl PastSurgery {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            date: None,
            hospital: None,
            notes: None,
        }
    }

    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    pub fn with_hospital(mut self, hospital: impl Into<String>) -> Self {
        self.hospital = Some(hospital.into());
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Hospitalization record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hospitalization {
    /// Reason for hospitalization
    pub reason: String,
    /// Admission date
    pub admission_date: Option<NaiveDate>,
    /// Discharge date
    pub discharge_date: Option<NaiveDate>,
    /// Hospital name
    pub hospital: Option<String>,
    /// Additional notes
    pub notes: Option<String>,
}

impl Hospitalization {
    pub fn new(reason: impl Into<String>) -> Self {
        Self {
            reason: reason.into(),
            admission_date: None,
            discharge_date: None,
            hospital: None,
            notes: None,
        }
    }

    pub fn with_admission_date(mut self, date: NaiveDate) -> Self {
        self.admission_date = Some(date);
        self
    }

    pub fn with_discharge_date(mut self, date: NaiveDate) -> Self {
        self.discharge_date = Some(date);
        self
    }

    pub fn with_hospital(mut self, hospital: impl Into<String>) -> Self {
        self.hospital = Some(hospital.into());
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Major illness record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MajorIllness {
    /// Illness name
    pub name: String,
    /// Date of occurrence
    pub date: Option<NaiveDate>,
    /// Outcome (e.g., "治愈", "好转", "稳定")
    pub outcome: Option<String>,
    /// Additional notes
    pub notes: Option<String>,
}

impl MajorIllness {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            date: None,
            outcome: None,
            notes: None,
        }
    }

    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    pub fn with_outcome(mut self, outcome: impl Into<String>) -> Self {
        self.outcome = Some(outcome.into());
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Family history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyHistoryEntry {
    /// Relationship to the person (e.g., "父亲", "母亲")
    pub relation: String,
    /// Disease name
    pub disease: String,
    /// Additional notes
    pub notes: Option<String>,
}

impl FamilyHistoryEntry {
    pub fn new(relation: impl Into<String>, disease: impl Into<String>) -> Self {
        Self {
            relation: relation.into(),
            disease: disease.into(),
            notes: None,
        }
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Long-term medication record for personal health profile
/// Different from extracted_data::Medication which is for AI-extracted prescriptions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LongTermMedication {
    /// Medication name
    pub name: String,
    /// Dosage (e.g., "10mg")
    pub dosage: Option<String>,
    /// Frequency (e.g., "每日一次")
    pub frequency: Option<String>,
    /// Start date
    pub start_date: Option<NaiveDate>,
    /// Additional notes
    pub notes: Option<String>,
}

impl LongTermMedication {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            dosage: None,
            frequency: None,
            start_date: None,
            notes: None,
        }
    }

    pub fn with_dosage(mut self, dosage: impl Into<String>) -> Self {
        self.dosage = Some(dosage.into());
        self
    }

    pub fn with_frequency(mut self, frequency: impl Into<String>) -> Self {
        self.frequency = Some(frequency.into());
        self
    }

    pub fn with_start_date(mut self, date: NaiveDate) -> Self {
        self.start_date = Some(date);
        self
    }

    pub fn with_notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }
}

/// Lifestyle information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifestyle {
    /// Smoking status
    pub smoking: Option<SmokingStatus>,
    /// Drinking status
    pub drinking: Option<DrinkingStatus>,
    /// Occupation
    pub occupation: Option<String>,
}

impl Lifestyle {
    pub fn new() -> Self {
        Self {
            smoking: None,
            drinking: None,
            occupation: None,
        }
    }

    pub fn with_smoking(mut self, smoking: SmokingStatus) -> Self {
        self.smoking = Some(smoking);
        self
    }

    pub fn with_drinking(mut self, drinking: DrinkingStatus) -> Self {
        self.drinking = Some(drinking);
        self
    }

    pub fn with_occupation(mut self, occupation: impl Into<String>) -> Self {
        self.occupation = Some(occupation.into());
        self
    }
}

impl Default for Lifestyle {
    fn default() -> Self {
        Self::new()
    }
}

/// Body measurements (height, weight, etc.)
/// Different from extracted_data::VitalSigns which includes blood pressure, heart rate, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyMeasurements {
    /// Height in cm
    pub height: Option<f32>,
    /// Weight in kg
    pub weight: Option<f32>,
    /// Last updated date
    pub last_updated: Option<NaiveDate>,
}

impl BodyMeasurements {
    pub fn new() -> Self {
        Self {
            height: None,
            weight: None,
            last_updated: None,
        }
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = Some(weight);
        self
    }

    pub fn with_last_updated(mut self, date: NaiveDate) -> Self {
        self.last_updated = Some(date);
        self
    }

    /// Calculate BMI if height and weight are available
    pub fn bmi(&self) -> Option<f32> {
        match (self.height, self.weight) {
            (Some(h), Some(w)) if h > 0.0 => Some(w / ((h / 100.0).powi(2))),
            _ => None,
        }
    }
}

impl Default for BodyMeasurements {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Person Entity
// ============================================================================

/// Person entity representing a person in the health records system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    /// Unique identifier
    pub id: String,
    /// Full name
    pub name: String,
    /// Relationship to the primary user
    pub relationship: Relationship,
    /// Date of birth
    pub birth_date: Option<NaiveDate>,
    /// Gender
    pub gender: Option<Gender>,
    /// Blood type
    pub blood_type: Option<BloodType>,
    /// Known allergies (stored as JSON array)
    pub allergies: Option<Vec<String>>,
    /// Additional notes
    pub notes: Option<String>,
    // Health Information
    /// Chronic diseases
    pub chronic_diseases: Option<Vec<ChronicDisease>>,
    /// Past surgeries
    pub past_surgeries: Option<Vec<PastSurgery>>,
    /// Hospitalization history
    pub hospitalizations: Option<Vec<Hospitalization>>,
    /// Major illnesses
    pub major_illnesses: Option<Vec<MajorIllness>>,
    /// Family history
    pub family_history: Option<Vec<FamilyHistoryEntry>>,
    /// Current medications
    pub current_medications: Option<Vec<LongTermMedication>>,
    /// Lifestyle information
    pub lifestyle: Option<Lifestyle>,
    /// Body measurements (height, weight)
    pub body_measurements: Option<BodyMeasurements>,
    /// Record creation time
    pub created_at: DateTime<Utc>,
    /// Record last update time
    pub updated_at: DateTime<Utc>,
}

impl Person {
    /// Create a new person with required fields
    pub fn new(name: String, relationship: Relationship) -> Self {
        let now = Utc::now();
        Self {
            id: super::common::generate_id(),
            name,
            relationship,
            birth_date: None,
            gender: None,
            blood_type: None,
            allergies: None,
            notes: None,
            chronic_diseases: None,
            past_surgeries: None,
            hospitalizations: None,
            major_illnesses: None,
            family_history: None,
            current_medications: None,
            lifestyle: None,
            body_measurements: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the birth date
    pub fn with_birth_date(mut self, date: NaiveDate) -> Self {
        self.birth_date = Some(date);
        self
    }

    /// Set the gender
    pub fn with_gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    /// Set the blood type
    pub fn with_blood_type(mut self, blood_type: BloodType) -> Self {
        self.blood_type = Some(blood_type);
        self
    }

    /// Set the allergies
    pub fn with_allergies(mut self, allergies: Vec<String>) -> Self {
        self.allergies = Some(allergies);
        self
    }

    /// Set the notes
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }

    /// Set chronic diseases
    pub fn with_chronic_diseases(mut self, diseases: Vec<ChronicDisease>) -> Self {
        self.chronic_diseases = Some(diseases);
        self
    }

    /// Set past surgeries
    pub fn with_past_surgeries(mut self, surgeries: Vec<PastSurgery>) -> Self {
        self.past_surgeries = Some(surgeries);
        self
    }

    /// Set hospitalizations
    pub fn with_hospitalizations(mut self, hospitalizations: Vec<Hospitalization>) -> Self {
        self.hospitalizations = Some(hospitalizations);
        self
    }

    /// Set major illnesses
    pub fn with_major_illnesses(mut self, illnesses: Vec<MajorIllness>) -> Self {
        self.major_illnesses = Some(illnesses);
        self
    }

    /// Set family history
    pub fn with_family_history(mut self, history: Vec<FamilyHistoryEntry>) -> Self {
        self.family_history = Some(history);
        self
    }

    /// Set current medications
    pub fn with_current_medications(mut self, medications: Vec<LongTermMedication>) -> Self {
        self.current_medications = Some(medications);
        self
    }

    /// Set lifestyle
    pub fn with_lifestyle(mut self, lifestyle: Lifestyle) -> Self {
        self.lifestyle = Some(lifestyle);
        self
    }

    /// Set body measurements
    pub fn with_body_measurements(mut self, body_measurements: BodyMeasurements) -> Self {
        self.body_measurements = Some(body_measurements);
        self
    }
}

/// Builder for creating Person instances with optional fields
#[derive(Debug, Default)]
pub struct PersonBuilder {
    name: Option<String>,
    relationship: Relationship,
    birth_date: Option<NaiveDate>,
    gender: Option<Gender>,
    blood_type: Option<BloodType>,
    allergies: Vec<String>,
    notes: Option<String>,
}

impl PersonBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn relationship(mut self, relationship: Relationship) -> Self {
        self.relationship = relationship;
        self
    }

    pub fn birth_date(mut self, date: NaiveDate) -> Self {
        self.birth_date = Some(date);
        self
    }

    pub fn gender(mut self, gender: Gender) -> Self {
        self.gender = Some(gender);
        self
    }

    pub fn blood_type(mut self, blood_type: BloodType) -> Self {
        self.blood_type = Some(blood_type);
        self
    }

    pub fn allergy(mut self, allergy: impl Into<String>) -> Self {
        self.allergies.push(allergy.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn build(self) -> Result<Person, String> {
        let name = self.name.ok_or("Name is required")?;
        let mut person = Person::new(name, self.relationship);

        if let Some(date) = self.birth_date {
            person.birth_date = Some(date);
        }
        if let Some(gender) = self.gender {
            person.gender = Some(gender);
        }
        if let Some(blood_type) = self.blood_type {
            person.blood_type = Some(blood_type);
        }
        if !self.allergies.is_empty() {
            person.allergies = Some(self.allergies);
        }
        if let Some(notes) = self.notes {
            person.notes = Some(notes);
        }

        Ok(person)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_person_new() {
        let person = Person::new("张三".to_string(), Relationship::Self_);

        assert_eq!(person.name, "张三");
        assert_eq!(person.relationship, Relationship::Self_);
        assert!(person.id.len() > 0);
        assert!(person.birth_date.is_none());
        assert!(person.gender.is_none());
        assert!(person.blood_type.is_none());
        assert!(person.allergies.is_none());
        assert!(person.notes.is_none());
    }

    #[test]
    fn test_person_with_methods() {
        let birth_date = NaiveDate::from_ymd_opt(1990, 1, 15).unwrap();

        let person = Person::new("张三".to_string(), Relationship::Self_)
            .with_birth_date(birth_date)
            .with_gender(Gender::Male)
            .with_blood_type(BloodType::A)
            .with_allergies(vec!["花生".to_string(), "海鲜".to_string()])
            .with_notes("测试备注".to_string());

        assert_eq!(person.birth_date, Some(birth_date));
        assert_eq!(person.gender, Some(Gender::Male));
        assert_eq!(person.blood_type, Some(BloodType::A));
        assert_eq!(person.allergies, Some(vec!["花生".to_string(), "海鲜".to_string()]));
        assert_eq!(person.notes, Some("测试备注".to_string()));
    }

    #[test]
    fn test_person_serde() {
        let person = Person::new("张三".to_string(), Relationship::Spouse);

        let json = serde_json::to_string(&person).unwrap();
        assert!(json.contains("\"name\":\"张三\""));
        assert!(json.contains("\"relationship\":\"spouse\""));

        let parsed: Person = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.name, person.name);
        assert_eq!(parsed.relationship, person.relationship);
        assert_eq!(parsed.id, person.id);
    }

    #[test]
    fn test_person_builder_success() {
        let birth_date = NaiveDate::from_ymd_opt(1985, 6, 20).unwrap();

        let person = PersonBuilder::new()
            .name("李四")
            .relationship(Relationship::Parent)
            .birth_date(birth_date)
            .gender(Gender::Female)
            .blood_type(BloodType::O)
            .allergy("青霉素")
            .allergy("阿司匹林")
            .notes("有药物过敏史")
            .build()
            .unwrap();

        assert_eq!(person.name, "李四");
        assert_eq!(person.relationship, Relationship::Parent);
        assert_eq!(person.birth_date, Some(birth_date));
        assert_eq!(person.gender, Some(Gender::Female));
        assert_eq!(person.blood_type, Some(BloodType::O));
        assert_eq!(person.allergies, Some(vec!["青霉素".to_string(), "阿司匹林".to_string()]));
        assert_eq!(person.notes, Some("有药物过敏史".to_string()));
    }

    #[test]
    fn test_person_builder_missing_name() {
        let result = PersonBuilder::new()
            .relationship(Relationship::Self_)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name is required");
    }

    #[test]
    fn test_person_builder_default_relationship() {
        let person = PersonBuilder::new()
            .name("王五")
            .build()
            .unwrap();

        assert_eq!(person.relationship, Relationship::Self_);
    }
}