//! Data models module

mod attachment;
mod common;
mod extracted_data;
mod person;
mod visit;

pub use attachment::Attachment;
pub use common::{
    generate_id, AttachmentType, BloodType, DiseaseStatus, DrinkingStatus, Gender, Relationship,
    SmokingStatus,
};
pub use extracted_data::{
    ExtractedData, ExtractedDataType, LabResult, Medication, OcrResult, VitalSigns,
};
pub use person::{
    BodyMeasurements, ChronicDisease, FamilyHistoryEntry, Hospitalization, Lifestyle,
    LongTermMedication, MajorIllness, PastSurgery, Person, PersonBuilder,
};
pub use visit::{Visit, VisitBuilder};