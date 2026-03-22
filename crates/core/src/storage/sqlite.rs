//! SQLite storage implementation

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use sqlx::Row;

use super::{Storage, StorageError, VisitFilters};
use crate::models::{
    Attachment, AttachmentType, ExtractedData, ExtractedDataType, Gender, OcrResult, Person,
    Relationship, Visit,
};

/// SQLite-based storage implementation
pub struct SqliteStorage {
    pool: SqlitePool,
}

impl SqliteStorage {
    /// Create a new SQLite storage with the given database URL
    pub async fn new(database_url: &str) -> Result<Self, StorageError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    /// Create an in-memory SQLite storage (for testing)
    pub async fn new_in_memory() -> Result<Self, StorageError> {
        Self::new("sqlite::memory:").await
    }
}

#[async_trait]
impl Storage for SqliteStorage {
    async fn create_person(&self, person: &Person) -> Result<String, StorageError> {
        let id = person.id.clone();
        let allergies = person
            .allergies
            .as_ref()
            .map(|a| serde_json::to_string(a).unwrap_or_default());

        sqlx::query(
            r#"
            INSERT INTO persons (id, name, relationship, birth_date, gender, blood_type, allergies, notes, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&person.name)
        .bind(person.relationship.to_string())
        .bind(person.birth_date)
        .bind(person.gender.as_ref().map(|g| g.to_string()))
        .bind(person.blood_type.as_ref().map(|b| b.to_string()))
        .bind(allergies)
        .bind(&person.notes)
        .bind(person.created_at)
        .bind(person.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_person(&self, id: &str) -> Result<Person, StorageError> {
        let row = sqlx::query("SELECT * FROM persons WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Person {
                id: row.get("id"),
                name: row.get("name"),
                relationship: row.get::<String, _>("relationship").parse().unwrap_or_default(),
                birth_date: row.get("birth_date"),
                gender: row.get::<Option<String>, _>("gender").and_then(|g| g.parse().ok()),
                blood_type: row.get::<Option<String>, _>("blood_type").and_then(|b| b.parse().ok()),
                allergies: row.get::<Option<String>, _>("allergies").and_then(|a| serde_json::from_str(&a).ok()),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(StorageError::NotFound(format!("Person not found: {}", id))),
        }
    }

    async fn list_persons(&self) -> Result<Vec<Person>, StorageError> {
        let rows = sqlx::query("SELECT * FROM persons ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| Person {
                id: row.get("id"),
                name: row.get("name"),
                relationship: row.get::<String, _>("relationship").parse().unwrap_or_default(),
                birth_date: row.get("birth_date"),
                gender: row.get::<Option<String>, _>("gender").and_then(|g| g.parse().ok()),
                blood_type: row.get::<Option<String>, _>("blood_type").and_then(|b| b.parse().ok()),
                allergies: row.get::<Option<String>, _>("allergies").and_then(|a| serde_json::from_str(&a).ok()),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }

    async fn update_person(&self, person: &Person) -> Result<(), StorageError> {
        let allergies = person
            .allergies
            .as_ref()
            .map(|a| serde_json::to_string(a).unwrap_or_default());

        let result = sqlx::query(
            r#"
            UPDATE persons
            SET name = ?, relationship = ?, birth_date = ?, gender = ?, blood_type = ?, allergies = ?, notes = ?
            WHERE id = ?
            "#,
        )
        .bind(&person.name)
        .bind(person.relationship.to_string())
        .bind(person.birth_date)
        .bind(person.gender.as_ref().map(|g| g.to_string()))
        .bind(person.blood_type.as_ref().map(|b| b.to_string()))
        .bind(allergies)
        .bind(&person.notes)
        .bind(&person.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Person not found: {}", person.id)));
        }

        Ok(())
    }

    async fn delete_person(&self, id: &str) -> Result<(), StorageError> {
        let result = sqlx::query("DELETE FROM persons WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Person not found: {}", id)));
        }

        Ok(())
    }

    async fn create_visit(&self, visit: &Visit) -> Result<String, StorageError> {
        let id = visit.id.clone();

        sqlx::query(
            r#"
            INSERT INTO visits (id, person_id, visit_date, hospital, department, doctor, chief_complaint, diagnosis, treatment, summary, notes, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&visit.person_id)
        .bind(visit.visit_date)
        .bind(&visit.hospital)
        .bind(&visit.department)
        .bind(&visit.doctor)
        .bind(&visit.chief_complaint)
        .bind(&visit.diagnosis)
        .bind(&visit.treatment)
        .bind(&visit.summary)
        .bind(&visit.notes)
        .bind(visit.created_at)
        .bind(visit.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_visit(&self, id: &str) -> Result<Visit, StorageError> {
        let row = sqlx::query("SELECT * FROM visits WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Visit {
                id: row.get("id"),
                person_id: row.get("person_id"),
                visit_date: row.get("visit_date"),
                hospital: row.get("hospital"),
                department: row.get("department"),
                doctor: row.get("doctor"),
                chief_complaint: row.get("chief_complaint"),
                diagnosis: row.get("diagnosis"),
                treatment: row.get("treatment"),
                summary: row.get("summary"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }),
            None => Err(StorageError::NotFound(format!("Visit not found: {}", id))),
        }
    }

    async fn list_visits(&self, person_id: Option<&str>) -> Result<Vec<Visit>, StorageError> {
        let rows = match person_id {
            Some(pid) => {
                sqlx::query("SELECT * FROM visits WHERE person_id = ? ORDER BY visit_date DESC")
                    .bind(pid)
                    .fetch_all(&self.pool)
                    .await?
            }
            None => {
                sqlx::query("SELECT * FROM visits ORDER BY visit_date DESC")
                    .fetch_all(&self.pool)
                    .await?
            }
        };

        Ok(rows
            .into_iter()
            .map(|row| Visit {
                id: row.get("id"),
                person_id: row.get("person_id"),
                visit_date: row.get("visit_date"),
                hospital: row.get("hospital"),
                department: row.get("department"),
                doctor: row.get("doctor"),
                chief_complaint: row.get("chief_complaint"),
                diagnosis: row.get("diagnosis"),
                treatment: row.get("treatment"),
                summary: row.get("summary"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }

    async fn list_visits_by_date_range(
        &self,
        person_id: &str,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Visit>, StorageError> {
        let rows = sqlx::query(
            "SELECT * FROM visits WHERE person_id = ? AND visit_date >= ? AND visit_date <= ? ORDER BY visit_date DESC",
        )
        .bind(person_id)
        .bind(start)
        .bind(end)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Visit {
                id: row.get("id"),
                person_id: row.get("person_id"),
                visit_date: row.get("visit_date"),
                hospital: row.get("hospital"),
                department: row.get("department"),
                doctor: row.get("doctor"),
                chief_complaint: row.get("chief_complaint"),
                diagnosis: row.get("diagnosis"),
                treatment: row.get("treatment"),
                summary: row.get("summary"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }

    async fn update_visit(&self, visit: &Visit) -> Result<(), StorageError> {
        let result = sqlx::query(
            r#"
            UPDATE visits
            SET visit_date = ?, hospital = ?, department = ?, doctor = ?, chief_complaint = ?, diagnosis = ?, treatment = ?, summary = ?, notes = ?
            WHERE id = ?
            "#,
        )
        .bind(visit.visit_date)
        .bind(&visit.hospital)
        .bind(&visit.department)
        .bind(&visit.doctor)
        .bind(&visit.chief_complaint)
        .bind(&visit.diagnosis)
        .bind(&visit.treatment)
        .bind(&visit.summary)
        .bind(&visit.notes)
        .bind(&visit.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Visit not found: {}", visit.id)));
        }

        Ok(())
    }

    async fn delete_visit(&self, id: &str) -> Result<(), StorageError> {
        let result = sqlx::query("DELETE FROM visits WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!("Visit not found: {}", id)));
        }

        Ok(())
    }

    async fn create_attachment(&self, attachment: &Attachment) -> Result<String, StorageError> {
        let id = attachment.id.clone();

        // Handle empty visit_id as NULL
        let visit_id = if attachment.visit_id.is_empty() {
            None
        } else {
            Some(&attachment.visit_id)
        };

        sqlx::query(
            r#"
            INSERT INTO attachments (id, visit_id, type, file_path, file_hash, file_size, mime_type, original_filename, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(visit_id)
        .bind(attachment.attachment_type.to_string())
        .bind(&attachment.file_path)
        .bind(&attachment.file_hash)
        .bind(attachment.file_size)
        .bind(&attachment.mime_type)
        .bind(&attachment.original_filename)
        .bind(attachment.created_at)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_attachment(&self, id: &str) -> Result<Attachment, StorageError> {
        let row = sqlx::query("SELECT * FROM attachments WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Attachment {
                id: row.get("id"),
                visit_id: row.get("visit_id"),
                attachment_type: row.get::<String, _>("type").parse().unwrap_or_default(),
                file_path: row.get("file_path"),
                file_hash: row.get("file_hash"),
                file_size: row.get("file_size"),
                mime_type: row.get("mime_type"),
                original_filename: row.get("original_filename"),
                created_at: row.get("created_at"),
            }),
            None => Err(StorageError::NotFound(format!(
                "Attachment not found: {}",
                id
            ))),
        }
    }

    async fn list_attachments(&self, visit_id: &str) -> Result<Vec<Attachment>, StorageError> {
        let rows = sqlx::query("SELECT * FROM attachments WHERE visit_id = ? ORDER BY created_at")
            .bind(visit_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| Attachment {
                id: row.get("id"),
                visit_id: row.get("visit_id"),
                attachment_type: row.get::<String, _>("type").parse().unwrap_or_default(),
                file_path: row.get("file_path"),
                file_hash: row.get("file_hash"),
                file_size: row.get("file_size"),
                mime_type: row.get("mime_type"),
                original_filename: row.get("original_filename"),
                created_at: row.get("created_at"),
            })
            .collect())
    }

    async fn update_attachment_visit(&self, attachment_id: &str, visit_id: &str) -> Result<(), StorageError> {
        sqlx::query("UPDATE attachments SET visit_id = ? WHERE id = ?")
            .bind(visit_id)
            .bind(attachment_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn delete_attachment(&self, id: &str) -> Result<(), StorageError> {
        let result = sqlx::query("DELETE FROM attachments WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound(format!(
                "Attachment not found: {}",
                id
            )));
        }

        Ok(())
    }

    async fn save_ocr_result(&self, result: &OcrResult) -> Result<String, StorageError> {
        let id = result.id.clone();

        sqlx::query(
            r#"
            INSERT INTO ocr_results (id, attachment_id, ocr_provider, recognized_text, confidence, processed_at)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&result.attachment_id)
        .bind(&result.ocr_provider)
        .bind(&result.recognized_text)
        .bind(result.confidence)
        .bind(result.processed_at)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_ocr_result(&self, attachment_id: &str) -> Result<Option<OcrResult>, StorageError> {
        let row = sqlx::query("SELECT * FROM ocr_results WHERE attachment_id = ? ORDER BY processed_at DESC LIMIT 1")
            .bind(attachment_id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(OcrResult {
                id: row.get("id"),
                attachment_id: row.get("attachment_id"),
                ocr_provider: row.get("ocr_provider"),
                recognized_text: row.get("recognized_text"),
                confidence: row.get("confidence"),
                processed_at: row.get("processed_at"),
            })),
            None => Ok(None),
        }
    }

    async fn save_extracted_data(&self, data: &ExtractedData) -> Result<String, StorageError> {
        let id = data.id.clone();

        sqlx::query(
            r#"
            INSERT INTO extracted_data (id, attachment_id, llm_provider, data_type, content, confidence, extracted_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&id)
        .bind(&data.attachment_id)
        .bind(&data.llm_provider)
        .bind(data.data_type.to_string())
        .bind(&data.content)
        .bind(data.confidence)
        .bind(data.extracted_at)
        .execute(&self.pool)
        .await?;

        Ok(id)
    }

    async fn get_extracted_data(
        &self,
        attachment_id: &str,
    ) -> Result<Vec<ExtractedData>, StorageError> {
        let rows = sqlx::query(
            "SELECT * FROM extracted_data WHERE attachment_id = ? ORDER BY extracted_at DESC",
        )
        .bind(attachment_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| ExtractedData {
                id: row.get("id"),
                attachment_id: row.get("attachment_id"),
                llm_provider: row.get("llm_provider"),
                data_type: row.get::<String, _>("data_type").parse().unwrap_or(ExtractedDataType::Summary),
                content: row.get("content"),
                confidence: row.get("confidence"),
                extracted_at: row.get("extracted_at"),
            })
            .collect())
    }

    async fn search(&self, query: &str) -> Result<Vec<Visit>, StorageError> {
        let rows = sqlx::query(
            r#"
            SELECT v.* FROM visits v
            JOIN visits_fts fts ON v.id = fts.id
            WHERE visits_fts MATCH ?
            ORDER BY v.visit_date DESC
            "#,
        )
        .bind(query)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Visit {
                id: row.get("id"),
                person_id: row.get("person_id"),
                visit_date: row.get("visit_date"),
                hospital: row.get("hospital"),
                department: row.get("department"),
                doctor: row.get("doctor"),
                chief_complaint: row.get("chief_complaint"),
                diagnosis: row.get("diagnosis"),
                treatment: row.get("treatment"),
                summary: row.get("summary"),
                notes: row.get("notes"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .collect())
    }

    async fn search_visits(&self, filters: VisitFilters) -> Result<Vec<Visit>, StorageError> {
        // Get all visits first, then filter in memory
        let all_visits = if let Some(ref query) = filters.query {
            if !query.is_empty() {
                // Use FTS for text search
                self.search(query).await?
            } else {
                self.list_visits(None).await?
            }
        } else {
            self.list_visits(None).await?
        };

        // Apply filters
        let filtered: Vec<Visit> = all_visits.into_iter().filter(|v| {
            let mut matches = true;

            if let Some(ref person_id) = filters.person_id {
                if !person_id.is_empty() && v.person_id != *person_id {
                    matches = false;
                }
            }

            if let Some(ref hospital) = filters.hospital {
                if !hospital.is_empty() {
                    if let Some(ref v_hospital) = v.hospital {
                        if !v_hospital.to_lowercase().contains(&hospital.to_lowercase()) {
                            matches = false;
                        }
                    } else {
                        matches = false;
                    }
                }
            }

            if let Some(ref doctor) = filters.doctor {
                if !doctor.is_empty() {
                    if let Some(ref v_doctor) = v.doctor {
                        if !v_doctor.to_lowercase().contains(&doctor.to_lowercase()) {
                            matches = false;
                        }
                    } else {
                        matches = false;
                    }
                }
            }

            matches
        }).collect();

        Ok(filtered)
    }

    async fn migrate(&self) -> Result<(), StorageError> {
        // Run migrations in order
        let migrations = vec![
            include_str!("../../../../migrations/001_init.sql"),
            include_str!("../../../../migrations/002_allow_null_visit_id.sql"),
            include_str!("../../../../migrations/003_add_summary.sql"),
        ];

        for migration_sql in migrations {
            Self::execute_migration(&self.pool, migration_sql).await?;
        }

        Ok(())
    }

    async fn close(&self) {
        self.pool.close().await;
    }
}

impl SqliteStorage {
    async fn execute_migration(pool: &sqlx::SqlitePool, migration_sql: &str) -> Result<(), StorageError> {
        // Execute the migration SQL
        // We need to handle triggers properly (they contain semicolons inside BEGIN...END)
        let mut current_statement = String::new();
        let mut in_trigger = false;

        for line in migration_sql.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with("--") {
                continue;
            }

            // Check if we're starting a trigger
            if trimmed.contains("CREATE TRIGGER") {
                in_trigger = true;
            }

            current_statement.push_str(line);
            current_statement.push('\n');

            // Check if this line ends the current statement
            let ends_statement = if in_trigger {
                trimmed == "END;" || trimmed == "END"
            } else {
                trimmed.ends_with(';')
            };

            if ends_statement {
                // Remove trailing semicolon for execution
                let sql = if current_statement.trim().ends_with(';') {
                    &current_statement[..current_statement.len()-1]
                } else {
                    &current_statement
                };

                let sql = sql.trim();
                if !sql.is_empty() {
                    let result = sqlx::query(sql)
                        .execute(pool)
                        .await;

                    match result {
                        Ok(_) => {}
                        Err(e) => {
                            let err_str = e.to_string();
                            // Ignore "duplicate column" and "already exists" errors (migration already applied)
                            if !err_str.contains("duplicate column") &&
                               !err_str.contains("already exists") &&
                               !err_str.contains("already another table") {
                                return Err(StorageError::Migration(err_str));
                            }
                        }
                    }
                }
                current_statement.clear();
                in_trigger = false;
            }
        }

        Ok(())
    }
}