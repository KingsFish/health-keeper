//! Person model - represents a person's health profile

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::common::{BloodType, Gender, Relationship};

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