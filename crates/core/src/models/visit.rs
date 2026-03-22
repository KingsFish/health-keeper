//! Visit model - represents a medical visit/appointment

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Visit entity representing a medical visit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visit {
    /// Unique identifier
    pub id: String,
    /// ID of the person this visit belongs to
    pub person_id: String,
    /// Date of the visit
    pub visit_date: NaiveDate,
    /// Hospital or clinic name
    pub hospital: Option<String>,
    /// Medical department
    pub department: Option<String>,
    /// Doctor's name
    pub doctor: Option<String>,
    /// Chief complaint / reason for visit
    pub chief_complaint: Option<String>,
    /// Diagnosis
    pub diagnosis: Option<String>,
    /// Treatment prescribed
    pub treatment: Option<String>,
    /// Additional notes
    pub notes: Option<String>,
    /// Record creation time
    pub created_at: DateTime<Utc>,
    /// Record last update time
    pub updated_at: DateTime<Utc>,
}

impl Visit {
    /// Create a new visit with required fields
    pub fn new(person_id: String, visit_date: NaiveDate) -> Self {
        let now = Utc::now();
        Self {
            id: super::common::generate_id(),
            person_id,
            visit_date,
            hospital: None,
            department: None,
            doctor: None,
            chief_complaint: None,
            diagnosis: None,
            treatment: None,
            notes: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Set the hospital
    pub fn with_hospital(mut self, hospital: String) -> Self {
        self.hospital = Some(hospital);
        self
    }

    /// Set the department
    pub fn with_department(mut self, department: String) -> Self {
        self.department = Some(department);
        self
    }

    /// Set the doctor
    pub fn with_doctor(mut self, doctor: String) -> Self {
        self.doctor = Some(doctor);
        self
    }

    /// Set the chief complaint
    pub fn with_chief_complaint(mut self, complaint: String) -> Self {
        self.chief_complaint = Some(complaint);
        self
    }

    /// Set the diagnosis
    pub fn with_diagnosis(mut self, diagnosis: String) -> Self {
        self.diagnosis = Some(diagnosis);
        self
    }

    /// Set the treatment
    pub fn with_treatment(mut self, treatment: String) -> Self {
        self.treatment = Some(treatment);
        self
    }

    /// Set the notes
    pub fn with_notes(mut self, notes: String) -> Self {
        self.notes = Some(notes);
        self
    }
}

/// Builder for creating Visit instances
#[derive(Debug, Default)]
pub struct VisitBuilder {
    person_id: Option<String>,
    visit_date: Option<NaiveDate>,
    hospital: Option<String>,
    department: Option<String>,
    doctor: Option<String>,
    chief_complaint: Option<String>,
    diagnosis: Option<String>,
    treatment: Option<String>,
    notes: Option<String>,
}

impl VisitBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn person_id(mut self, id: impl Into<String>) -> Self {
        self.person_id = Some(id.into());
        self
    }

    pub fn visit_date(mut self, date: NaiveDate) -> Self {
        self.visit_date = Some(date);
        self
    }

    pub fn hospital(mut self, hospital: impl Into<String>) -> Self {
        self.hospital = Some(hospital.into());
        self
    }

    pub fn department(mut self, department: impl Into<String>) -> Self {
        self.department = Some(department.into());
        self
    }

    pub fn doctor(mut self, doctor: impl Into<String>) -> Self {
        self.doctor = Some(doctor.into());
        self
    }

    pub fn chief_complaint(mut self, complaint: impl Into<String>) -> Self {
        self.chief_complaint = Some(complaint.into());
        self
    }

    pub fn diagnosis(mut self, diagnosis: impl Into<String>) -> Self {
        self.diagnosis = Some(diagnosis.into());
        self
    }

    pub fn treatment(mut self, treatment: impl Into<String>) -> Self {
        self.treatment = Some(treatment.into());
        self
    }

    pub fn notes(mut self, notes: impl Into<String>) -> Self {
        self.notes = Some(notes.into());
        self
    }

    pub fn build(self) -> Result<Visit, String> {
        let person_id = self.person_id.ok_or("Person ID is required")?;
        let visit_date = self.visit_date.ok_or("Visit date is required")?;

        let mut visit = Visit::new(person_id, visit_date);

        if let Some(hospital) = self.hospital {
            visit.hospital = Some(hospital);
        }
        if let Some(department) = self.department {
            visit.department = Some(department);
        }
        if let Some(doctor) = self.doctor {
            visit.doctor = Some(doctor);
        }
        if let Some(chief_complaint) = self.chief_complaint {
            visit.chief_complaint = Some(chief_complaint);
        }
        if let Some(diagnosis) = self.diagnosis {
            visit.diagnosis = Some(diagnosis);
        }
        if let Some(treatment) = self.treatment {
            visit.treatment = Some(treatment);
        }
        if let Some(notes) = self.notes {
            visit.notes = Some(notes);
        }

        Ok(visit)
    }
}