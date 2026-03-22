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
    /// Summary of the visit
    pub summary: Option<String>,
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
            summary: None,
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

    /// Set the summary
    pub fn with_summary(mut self, summary: String) -> Self {
        self.summary = Some(summary);
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
    summary: Option<String>,
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

    pub fn summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
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
        if let Some(summary) = self.summary {
            visit.summary = Some(summary);
        }
        if let Some(notes) = self.notes {
            visit.notes = Some(notes);
        }

        Ok(visit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit_new() {
        let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).unwrap();
        let visit = Visit::new("person-123".to_string(), visit_date);

        assert_eq!(visit.person_id, "person-123");
        assert_eq!(visit.visit_date, visit_date);
        assert!(visit.id.len() > 0);
        assert!(visit.hospital.is_none());
        assert!(visit.department.is_none());
        assert!(visit.doctor.is_none());
        assert!(visit.diagnosis.is_none());
    }

    #[test]
    fn test_visit_with_methods() {
        let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).unwrap();

        let visit = Visit::new("person-123".to_string(), visit_date)
            .with_hospital("北京医院".to_string())
            .with_department("内科".to_string())
            .with_doctor("王医生".to_string())
            .with_chief_complaint("头痛三天".to_string())
            .with_diagnosis("偏头痛".to_string())
            .with_treatment("止痛药".to_string())
            .with_summary("症状缓解".to_string())
            .with_notes("随访两周".to_string());

        assert_eq!(visit.hospital, Some("北京医院".to_string()));
        assert_eq!(visit.department, Some("内科".to_string()));
        assert_eq!(visit.doctor, Some("王医生".to_string()));
        assert_eq!(visit.chief_complaint, Some("头痛三天".to_string()));
        assert_eq!(visit.diagnosis, Some("偏头痛".to_string()));
        assert_eq!(visit.treatment, Some("止痛药".to_string()));
        assert_eq!(visit.summary, Some("症状缓解".to_string()));
        assert_eq!(visit.notes, Some("随访两周".to_string()));
    }

    #[test]
    fn test_visit_serde() {
        let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).unwrap();
        let visit = Visit::new("person-123".to_string(), visit_date)
            .with_hospital("测试医院".to_string());

        let json = serde_json::to_string(&visit).unwrap();
        assert!(json.contains("\"person_id\":\"person-123\""));
        assert!(json.contains("\"visit_date\":\"2026-03-22\""));
        assert!(json.contains("\"hospital\":\"测试医院\""));

        let parsed: Visit = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.person_id, visit.person_id);
        assert_eq!(parsed.visit_date, visit.visit_date);
        assert_eq!(parsed.hospital, visit.hospital);
    }

    #[test]
    fn test_visit_builder_success() {
        let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).unwrap();

        let visit = VisitBuilder::new()
            .person_id("person-456")
            .visit_date(visit_date)
            .hospital("上海医院")
            .department("外科")
            .doctor("李医生")
            .diagnosis("骨折")
            .build()
            .unwrap();

        assert_eq!(visit.person_id, "person-456");
        assert_eq!(visit.visit_date, visit_date);
        assert_eq!(visit.hospital, Some("上海医院".to_string()));
        assert_eq!(visit.department, Some("外科".to_string()));
        assert_eq!(visit.doctor, Some("李医生".to_string()));
        assert_eq!(visit.diagnosis, Some("骨折".to_string()));
    }

    #[test]
    fn test_visit_builder_missing_person_id() {
        let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).unwrap();

        let result = VisitBuilder::new()
            .visit_date(visit_date)
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Person ID is required");
    }

    #[test]
    fn test_visit_builder_missing_visit_date() {
        let result = VisitBuilder::new()
            .person_id("person-123")
            .build();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Visit date is required");
    }
}