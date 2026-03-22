//! Storage module - data persistence layer

mod sqlite;

pub use sqlite::SqliteStorage;

use async_trait::async_trait;
use chrono::NaiveDate;

use crate::models::{Attachment, ExtractedData, OcrResult, Person, Visit};

/// Storage error type
#[derive(thiserror::Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Invalid data: {0}")]
    InvalidData(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Migration error: {0}")]
    Migration(String),
}

/// Storage provider trait - defines the interface for data persistence
#[async_trait]
pub trait Storage: Send + Sync {
    // Person operations
    async fn create_person(&self, person: &Person) -> Result<String, StorageError>;
    async fn get_person(&self, id: &str) -> Result<Person, StorageError>;
    async fn list_persons(&self) -> Result<Vec<Person>, StorageError>;
    async fn update_person(&self, person: &Person) -> Result<(), StorageError>;
    async fn delete_person(&self, id: &str) -> Result<(), StorageError>;

    // Visit operations
    async fn create_visit(&self, visit: &Visit) -> Result<String, StorageError>;
    async fn get_visit(&self, id: &str) -> Result<Visit, StorageError>;
    async fn list_visits(&self, person_id: Option<&str>) -> Result<Vec<Visit>, StorageError>;
    async fn list_visits_by_date_range(
        &self,
        person_id: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Visit>, StorageError>;
    async fn update_visit(&self, visit: &Visit) -> Result<(), StorageError>;
    async fn delete_visit(&self, id: &str) -> Result<(), StorageError>;

    // Attachment operations
    async fn create_attachment(&self, attachment: &Attachment) -> Result<String, StorageError>;
    async fn get_attachment(&self, id: &str) -> Result<Attachment, StorageError>;
    async fn list_attachments(&self, visit_id: &str) -> Result<Vec<Attachment>, StorageError>;
    async fn delete_attachment(&self, id: &str) -> Result<(), StorageError>;

    // OCR operations
    async fn save_ocr_result(&self, result: &OcrResult) -> Result<String, StorageError>;
    async fn get_ocr_result(&self, attachment_id: &str) -> Result<Option<OcrResult>, StorageError>;

    // Extracted data operations
    async fn save_extracted_data(&self, data: &ExtractedData) -> Result<String, StorageError>;
    async fn get_extracted_data(
        &self,
        attachment_id: &str,
    ) -> Result<Vec<ExtractedData>, StorageError>;

    // Search operations
    async fn search(&self, query: &str) -> Result<Vec<Visit>, StorageError>;

    // Database operations
    async fn migrate(&self) -> Result<(), StorageError>;
    async fn close(&self);
}