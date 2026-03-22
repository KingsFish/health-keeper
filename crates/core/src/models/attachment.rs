//! Attachment model - represents an attached file (image, PDF, etc.)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::AttachmentType;

/// Attachment entity representing a file attached to a visit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// Unique identifier
    pub id: String,
    /// ID of the visit this attachment belongs to
    pub visit_id: String,
    /// Type of attachment
    #[serde(rename = "type")]
    pub attachment_type: AttachmentType,
    /// Relative path to the file
    pub file_path: String,
    /// SHA-256 hash of the file content
    pub file_hash: Option<String>,
    /// File size in bytes
    pub file_size: Option<i64>,
    /// MIME type
    pub mime_type: Option<String>,
    /// Original filename
    pub original_filename: Option<String>,
    /// Record creation time
    pub created_at: DateTime<Utc>,
}

impl Attachment {
    /// Create a new attachment with required fields
    pub fn new(visit_id: String, file_path: String, attachment_type: AttachmentType) -> Self {
        Self {
            id: super::common::generate_id(),
            visit_id,
            attachment_type,
            file_path,
            file_hash: None,
            file_size: None,
            mime_type: None,
            original_filename: None,
            created_at: Utc::now(),
        }
    }

    /// Set the file hash
    pub fn with_hash(mut self, hash: String) -> Self {
        self.file_hash = Some(hash);
        self
    }

    /// Set the file size
    pub fn with_size(mut self, size: i64) -> Self {
        self.file_size = Some(size);
        self
    }

    /// Set the MIME type
    pub fn with_mime_type(mut self, mime_type: String) -> Self {
        self.mime_type = Some(mime_type);
        self
    }

    /// Set the original filename
    pub fn with_original_filename(mut self, filename: String) -> Self {
        self.original_filename = Some(filename);
        self
    }
}