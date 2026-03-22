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