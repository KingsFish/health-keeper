//! Data models module

mod attachment;
mod common;
mod extracted_data;
mod person;
mod visit;

pub use attachment::Attachment;
pub use common::{generate_id, AttachmentType, BloodType, Gender, Relationship};
pub use extracted_data::{
    ExtractedData, ExtractedDataType, LabResult, Medication, OcrResult, VitalSigns,
};
pub use person::{Person, PersonBuilder};
pub use visit::{Visit, VisitBuilder};