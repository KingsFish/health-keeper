//! HealthKeeper Web Server

use axum::{
    extract::{Multipart, Path, Query, State},
    http::{header, StatusCode},
    response::{Html, Json, sse::Event, Sse},
    routing::{delete, get, post, put, Router},
};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

use health_keeper_core::{
    ai::{ExtractionContext, LlmRegistry, OcrRegistry, AnthropicProvider, VisionOcrProvider},
    models::{AttachmentType, OcrResult, Person, Relationship, Visit},
    storage::{SqliteStorage, Storage},
    AppConfig,
};

/// Application state shared across handlers
struct AppState {
    storage: SqliteStorage,
    config: AppConfig,
}

// ==================== DTOs ====================

#[derive(Debug, Deserialize)]
struct CreatePersonRequest {
    name: String,
    #[serde(default)]
    relationship: String,
    birth_date: Option<String>,
    gender: Option<String>,
    blood_type: Option<String>,
    allergies: Option<Vec<String>>,
    notes: Option<String>,
    // Health information
    chronic_diseases: Option<Vec<health_keeper_core::models::ChronicDisease>>,
    past_surgeries: Option<Vec<health_keeper_core::models::PastSurgery>>,
    hospitalizations: Option<Vec<health_keeper_core::models::Hospitalization>>,
    major_illnesses: Option<Vec<health_keeper_core::models::MajorIllness>>,
    family_history: Option<Vec<health_keeper_core::models::FamilyHistoryEntry>>,
    current_medications: Option<Vec<health_keeper_core::models::LongTermMedication>>,
    lifestyle: Option<health_keeper_core::models::Lifestyle>,
    body_measurements: Option<health_keeper_core::models::BodyMeasurements>,
}

#[derive(Debug, Serialize)]
struct PersonResponse {
    id: String,
    name: String,
    relationship: String,
    birth_date: Option<String>,
    gender: Option<String>,
    blood_type: Option<String>,
    allergies: Option<Vec<String>>,
    notes: Option<String>,
    // Health information
    chronic_diseases: Option<Vec<health_keeper_core::models::ChronicDisease>>,
    past_surgeries: Option<Vec<health_keeper_core::models::PastSurgery>>,
    hospitalizations: Option<Vec<health_keeper_core::models::Hospitalization>>,
    major_illnesses: Option<Vec<health_keeper_core::models::MajorIllness>>,
    family_history: Option<Vec<health_keeper_core::models::FamilyHistoryEntry>>,
    current_medications: Option<Vec<health_keeper_core::models::LongTermMedication>>,
    lifestyle: Option<health_keeper_core::models::Lifestyle>,
    body_measurements: Option<health_keeper_core::models::BodyMeasurements>,
    created_at: String,
    updated_at: String,
}

impl From<Person> for PersonResponse {
    fn from(p: Person) -> Self {
        Self {
            id: p.id,
            name: p.name,
            relationship: p.relationship.to_string(),
            birth_date: p.birth_date.map(|d| d.to_string()),
            gender: p.gender.map(|g| g.to_string()),
            blood_type: p.blood_type.map(|b| b.to_string()),
            allergies: p.allergies,
            notes: p.notes,
            chronic_diseases: p.chronic_diseases,
            past_surgeries: p.past_surgeries,
            hospitalizations: p.hospitalizations,
            major_illnesses: p.major_illnesses,
            family_history: p.family_history,
            current_medications: p.current_medications,
            lifestyle: p.lifestyle,
            body_measurements: p.body_measurements,
            created_at: p.created_at.to_rfc3339(),
            updated_at: p.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpdatePersonRequest {
    name: Option<String>,
    relationship: Option<String>,
    birth_date: Option<String>,
    gender: Option<String>,
    blood_type: Option<String>,
    allergies: Option<Vec<String>>,
    notes: Option<String>,
    // Health information
    chronic_diseases: Option<Vec<health_keeper_core::models::ChronicDisease>>,
    past_surgeries: Option<Vec<health_keeper_core::models::PastSurgery>>,
    hospitalizations: Option<Vec<health_keeper_core::models::Hospitalization>>,
    major_illnesses: Option<Vec<health_keeper_core::models::MajorIllness>>,
    family_history: Option<Vec<health_keeper_core::models::FamilyHistoryEntry>>,
    current_medications: Option<Vec<health_keeper_core::models::LongTermMedication>>,
    lifestyle: Option<health_keeper_core::models::Lifestyle>,
    body_measurements: Option<health_keeper_core::models::BodyMeasurements>,
}

#[derive(Debug, Deserialize)]
struct CreateVisitRequest {
    person_id: String,
    visit_date: String,
    hospital: Option<String>,
    department: Option<String>,
    doctor: Option<String>,
    chief_complaint: Option<String>,
    diagnosis: Option<String>,
    treatment: Option<String>,
    summary: Option<String>,
    notes: Option<String>,
    #[serde(default)]
    attachment_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateVisitRequest {
    hospital: Option<String>,
    department: Option<String>,
    doctor: Option<String>,
    chief_complaint: Option<String>,
    diagnosis: Option<String>,
    treatment: Option<String>,
    summary: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
struct VisitResponse {
    id: String,
    person_id: String,
    visit_date: String,
    hospital: Option<String>,
    department: Option<String>,
    doctor: Option<String>,
    chief_complaint: Option<String>,
    diagnosis: Option<String>,
    treatment: Option<String>,
    summary: Option<String>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
    attachments: Vec<AttachmentResponse>,
}

#[derive(Debug, Serialize)]
struct AttachmentResponse {
    id: String,
    visit_id: String,
    attachment_type: String,
    file_path: String,
    original_filename: Option<String>,
    file_size: Option<i64>,
    mime_type: Option<String>,
    created_at: String,
    has_ocr: bool,
    has_extraction: bool,
}

#[derive(Debug, Serialize)]
struct OcrResultResponse {
    id: String,
    attachment_id: String,
    ocr_provider: String,
    recognized_text: String,
    confidence: Option<f64>,
    processed_at: String,
}

#[derive(Debug, Serialize)]
struct ExtractionResponse {
    diagnosis: Option<String>,
    chief_complaint: Option<String>,
    treatment: Option<String>,
    medications: Vec<MedicationResponse>,
    follow_up: Option<String>,
    summary: Option<String>,
}

#[derive(Debug, Serialize)]
struct QuickImportResponse {
    visit_date: Option<String>,
    hospital: Option<String>,
    department: Option<String>,
    doctor: Option<String>,
    chief_complaint: Option<String>,
    diagnosis: Option<String>,
    treatment: Option<String>,
    medications: Vec<MedicationResponse>,
    lab_results: Vec<LabResultResponse>,
    follow_up: Option<String>,
    summary: Option<String>,
    ocr_text: String,
    attachment_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
struct LabResultResponse {
    name: String,
    value: String,
    unit: Option<String>,
    reference_range: Option<String>,
    is_abnormal: Option<bool>,
}

#[derive(Debug, Serialize)]
struct MedicationResponse {
    name: String,
    dosage: Option<String>,
    frequency: Option<String>,
    duration: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    q: Option<String>,
    person_id: Option<String>,
    hospital: Option<String>,
    doctor: Option<String>,
}

// ==================== Handlers ====================

async fn list_persons(State(state): State<Arc<AppState>>) -> Json<Vec<PersonResponse>> {
    let persons = state.storage.list_persons().await.unwrap_or_default();
    Json(persons.into_iter().map(PersonResponse::from).collect())
}

async fn create_person(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreatePersonRequest>,
) -> Result<Json<PersonResponse>, StatusCode> {
    let relationship: Relationship = req.relationship.parse().unwrap_or(Relationship::Self_);

    let mut person = Person::new(req.name, relationship);

    if let Some(date) = req.birth_date {
        if let Ok(d) = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
            person.birth_date = Some(d);
        }
    }
    if let Some(g) = req.gender {
        person.gender = g.parse().ok();
    }
    if let Some(bt) = req.blood_type {
        person.blood_type = bt.parse().ok();
    }
    if let Some(a) = req.allergies {
        person.allergies = Some(a);
    }
    if let Some(n) = req.notes {
        person.notes = Some(n);
    }
    // Health information
    if let Some(cd) = req.chronic_diseases {
        person.chronic_diseases = Some(cd);
    }
    if let Some(ps) = req.past_surgeries {
        person.past_surgeries = Some(ps);
    }
    if let Some(h) = req.hospitalizations {
        person.hospitalizations = Some(h);
    }
    if let Some(mi) = req.major_illnesses {
        person.major_illnesses = Some(mi);
    }
    if let Some(fh) = req.family_history {
        person.family_history = Some(fh);
    }
    if let Some(cm) = req.current_medications {
        person.current_medications = Some(cm);
    }
    if let Some(l) = req.lifestyle {
        person.lifestyle = Some(l);
    }
    if let Some(bm) = req.body_measurements {
        person.body_measurements = Some(bm);
    }

    match state.storage.create_person(&person).await {
        Ok(_) => Ok(Json(PersonResponse::from(person))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_person(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<PersonResponse>, StatusCode> {
    match state.storage.get_person(&id).await {
        Ok(person) => Ok(Json(PersonResponse::from(person))),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_person(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    match state.storage.delete_person(&id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_person_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdatePersonRequest>,
) -> Result<Json<PersonResponse>, StatusCode> {
    // Get existing person
    let mut person = match state.storage.get_person(&id).await {
        Ok(p) => p,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    // Update fields
    if let Some(name) = req.name {
        person.name = name;
    }
    if let Some(relationship) = req.relationship {
        if let Ok(r) = relationship.parse() {
            person.relationship = r;
        }
    }
    if let Some(date) = req.birth_date {
        person.birth_date = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d").ok();
    }
    if let Some(gender) = req.gender {
        person.gender = gender.parse().ok();
    }
    if let Some(bt) = req.blood_type {
        person.blood_type = bt.parse().ok();
    }
    if let Some(a) = req.allergies {
        person.allergies = Some(a);
    }
    if let Some(n) = req.notes {
        person.notes = Some(n);
    }
    if let Some(cd) = req.chronic_diseases {
        person.chronic_diseases = Some(cd);
    }
    if let Some(ps) = req.past_surgeries {
        person.past_surgeries = Some(ps);
    }
    if let Some(h) = req.hospitalizations {
        person.hospitalizations = Some(h);
    }
    if let Some(mi) = req.major_illnesses {
        person.major_illnesses = Some(mi);
    }
    if let Some(fh) = req.family_history {
        person.family_history = Some(fh);
    }
    if let Some(cm) = req.current_medications {
        person.current_medications = Some(cm);
    }
    if let Some(l) = req.lifestyle {
        person.lifestyle = Some(l);
    }
    if let Some(bm) = req.body_measurements {
        person.body_measurements = Some(bm);
    }

    // Update timestamp
    person.updated_at = chrono::Utc::now();

    match state.storage.update_person(&person).await {
        Ok(_) => Ok(Json(PersonResponse::from(person))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn list_visits(
    State(state): State<Arc<AppState>>,
    Query(params): Query<VisitQueryParams>,
) -> Json<Vec<VisitResponse>> {
    let visits = state.storage.list_visits(params.person_id.as_deref()).await.unwrap_or_default();

    let mut responses = Vec::new();
    for v in visits {
        let attachments = state.storage.list_attachments(&v.id).await.unwrap_or_default();
        let mut attachment_responses = Vec::new();

        for a in &attachments {
            let has_ocr = state.storage.get_ocr_result(&a.id).await.ok().flatten().is_some();
            let has_extraction = !state.storage.get_extracted_data(&a.id).await.unwrap_or_default().is_empty();

            attachment_responses.push(AttachmentResponse {
                id: a.id.clone(),
                visit_id: a.visit_id.clone(),
                attachment_type: a.attachment_type.to_string(),
                file_path: a.file_path.clone(),
                original_filename: a.original_filename.clone(),
                file_size: a.file_size,
                mime_type: a.mime_type.clone(),
                created_at: a.created_at.to_rfc3339(),
                has_ocr,
                has_extraction,
            });
        }

        responses.push(VisitResponse {
            id: v.id,
            person_id: v.person_id,
            visit_date: v.visit_date.to_string(),
            hospital: v.hospital,
            department: v.department,
            doctor: v.doctor,
            chief_complaint: v.chief_complaint,
            diagnosis: v.diagnosis,
            treatment: v.treatment,
            summary: v.summary,
            notes: v.notes,
            created_at: v.created_at.to_rfc3339(),
            updated_at: v.updated_at.to_rfc3339(),
            attachments: attachment_responses,
        });
    }

    Json(responses)
}

#[derive(Debug, Deserialize)]
struct VisitQueryParams {
    person_id: Option<String>,
}

async fn create_visit(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateVisitRequest>,
) -> Result<Json<VisitResponse>, StatusCode> {
    let visit_date = chrono::NaiveDate::parse_from_str(&req.visit_date, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut visit = Visit::new(req.person_id, visit_date);

    visit.hospital = req.hospital;
    visit.department = req.department;
    visit.doctor = req.doctor;
    visit.chief_complaint = req.chief_complaint;
    visit.diagnosis = req.diagnosis;
    visit.treatment = req.treatment;
    visit.summary = req.summary;
    visit.notes = req.notes;

    // Create visit
    state.storage.create_visit(&visit).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Link attachments to visit
    for attachment_id in &req.attachment_ids {
        if let Err(e) = state.storage.update_attachment_visit(attachment_id, &visit.id).await {
            eprintln!("[WARN] Failed to link attachment {}: {}", attachment_id, e);
        }
    }

    Ok(Json(VisitResponse {
            id: visit.id,
            person_id: visit.person_id,
            visit_date: visit.visit_date.to_string(),
            hospital: visit.hospital,
            department: visit.department,
            doctor: visit.doctor,
            chief_complaint: visit.chief_complaint,
            diagnosis: visit.diagnosis,
            treatment: visit.treatment,
            summary: visit.summary,
            notes: visit.notes,
            created_at: visit.created_at.to_rfc3339(),
            updated_at: visit.updated_at.to_rfc3339(),
            attachments: vec![],
    }))
}

async fn get_visit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<VisitResponse>, StatusCode> {
    println!("[DEBUG] get_visit called with id: {}", id);
    let visit = match state.storage.get_visit(&id).await {
        Ok(v) => v,
        Err(e) => {
            println!("[DEBUG] get_visit error: {:?}", e);
            return Err(StatusCode::NOT_FOUND);
        }
    };
    println!("[DEBUG] get_visit found: {:?}", visit.id);
    let attachments = state.storage.list_attachments(&id).await.unwrap_or_default();

    let mut attachment_responses = Vec::new();
    for a in &attachments {
        let has_ocr = state.storage.get_ocr_result(&a.id).await.ok().flatten().is_some();
        let has_extraction = !state.storage.get_extracted_data(&a.id).await.unwrap_or_default().is_empty();

        attachment_responses.push(AttachmentResponse {
            id: a.id.clone(),
            visit_id: a.visit_id.clone(),
            attachment_type: a.attachment_type.to_string(),
            file_path: a.file_path.clone(),
            original_filename: a.original_filename.clone(),
            file_size: a.file_size,
            mime_type: a.mime_type.clone(),
            created_at: a.created_at.to_rfc3339(),
            has_ocr,
            has_extraction,
        });
    }

    Ok(Json(VisitResponse {
        id: visit.id,
        person_id: visit.person_id,
        visit_date: visit.visit_date.to_string(),
        hospital: visit.hospital,
        department: visit.department,
        doctor: visit.doctor,
        chief_complaint: visit.chief_complaint,
        diagnosis: visit.diagnosis,
        treatment: visit.treatment,
        summary: visit.summary,
        notes: visit.notes,
        created_at: visit.created_at.to_rfc3339(),
        updated_at: visit.updated_at.to_rfc3339(),
        attachments: attachment_responses,
    }))
}

async fn update_visit_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(req): Json<UpdateVisitRequest>,
) -> Result<Json<VisitResponse>, StatusCode> {
    // Get existing visit
    let mut visit = match state.storage.get_visit(&id).await {
        Ok(v) => v,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };

    // Update fields
    if let Some(hospital) = req.hospital {
        visit.hospital = Some(hospital);
    }
    if let Some(department) = req.department {
        visit.department = Some(department);
    }
    if let Some(doctor) = req.doctor {
        visit.doctor = Some(doctor);
    }
    if let Some(chief_complaint) = req.chief_complaint {
        visit.chief_complaint = Some(chief_complaint);
    }
    if let Some(diagnosis) = req.diagnosis {
        visit.diagnosis = Some(diagnosis);
    }
    if let Some(treatment) = req.treatment {
        visit.treatment = Some(treatment);
    }
    if let Some(summary) = req.summary {
        visit.summary = Some(summary);
    }
    if let Some(notes) = req.notes {
        visit.notes = Some(notes);
    }

    // Update timestamp
    visit.updated_at = chrono::Utc::now();

    // Save to storage
    state.storage.update_visit(&visit).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get attachments for response
    let attachments = state.storage.list_attachments(&id).await.unwrap_or_default();
    let attachment_responses: Vec<AttachmentResponse> = attachments.into_iter().map(|a| {
        AttachmentResponse {
            id: a.id.clone(),
            visit_id: a.visit_id.clone(),
            attachment_type: a.attachment_type.to_string(),
            file_path: a.file_path.clone(),
            original_filename: a.original_filename.clone(),
            file_size: a.file_size,
            mime_type: a.mime_type.clone(),
            created_at: a.created_at.to_rfc3339(),
            has_ocr: false, // We don't check this here for simplicity
            has_extraction: false,
        }
    }).collect();

    Ok(Json(VisitResponse {
        id: visit.id,
        person_id: visit.person_id,
        visit_date: visit.visit_date.to_string(),
        hospital: visit.hospital,
        department: visit.department,
        doctor: visit.doctor,
        chief_complaint: visit.chief_complaint,
        diagnosis: visit.diagnosis,
        treatment: visit.treatment,
        summary: visit.summary,
        notes: visit.notes,
        created_at: visit.created_at.to_rfc3339(),
        updated_at: visit.updated_at.to_rfc3339(),
        attachments: attachment_responses,
    }))
}

async fn delete_visit(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    // First get attachments to delete files from disk
    let attachments = state.storage.list_attachments(&id).await.unwrap_or_default();

    // Delete attachment files from disk
    for attachment in &attachments {
        let file_path = std::path::Path::new("./data").join(&attachment.file_path);
        if file_path.exists() {
            if let Err(e) = std::fs::remove_file(&file_path) {
                eprintln!("[WARN] Failed to delete file {:?}: {}", file_path, e);
            }
        }
    }

    // Then delete the visit (cascade will delete attachments from DB)
    state.storage.delete_visit(&id).await.map_err(|_| StatusCode::NOT_FOUND)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn upload_attachment(
    State(state): State<Arc<AppState>>,
    Path(visit_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<AttachmentResponse>, StatusCode> {
    // Verify visit exists
    state.storage.get_visit(&visit_id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    while let Some(field) = multipart.next_field().await.ok().flatten() {
        let file_name = field.file_name().map(|s| s.to_string()).unwrap_or_else(|| "unknown".to_string());
        let content_type = field.content_type().map(|s| s.to_string());

        if let Ok(data) = field.bytes().await {
            let file_size = data.len() as i64;

            // Calculate hash
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(&data);
            let hash = hex::encode(hasher.finalize());

            // Determine extension and type
            let ext = std::path::Path::new(&file_name)
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("");

            let attachment_type = determine_attachment_type(&file_name, content_type.as_deref());

            // Generate storage path
            let stored_name = format!("{}.{}", hash, ext);
            let relative_path = format!("attachments/{}", stored_name);

            // Save file
            let data_dir = std::path::Path::new("./data");
            let attachments_dir = data_dir.join("attachments");
            std::fs::create_dir_all(&attachments_dir).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            let dest_path = attachments_dir.join(&stored_name);
            std::fs::write(&dest_path, &data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            // Create attachment record
            let attachment = health_keeper_core::models::Attachment::new(
                visit_id.clone(),
                relative_path.clone(),
                attachment_type,
            );

            let mut attachment = attachment;
            attachment.file_hash = Some(hash);
            attachment.file_size = Some(file_size);
            attachment.mime_type = content_type.clone();
            attachment.original_filename = Some(file_name.clone());

            match state.storage.create_attachment(&attachment).await {
                Ok(_) => return Ok(Json(AttachmentResponse {
                    id: attachment.id,
                    visit_id: attachment.visit_id,
                    attachment_type: attachment.attachment_type.to_string(),
                    file_path: attachment.file_path,
                    original_filename: attachment.original_filename,
                    file_size: attachment.file_size,
                    mime_type: attachment.mime_type,
                    created_at: attachment.created_at.to_rfc3339(),
                    has_ocr: false,
                    has_extraction: false,
                })),
                Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }

    Err(StatusCode::BAD_REQUEST)
}

fn determine_attachment_type(filename: &str, content_type: Option<&str>) -> AttachmentType {
    let filename_lower = filename.to_lowercase();

    if filename_lower.contains("病历") || filename_lower.contains("record") {
        return AttachmentType::MedicalRecord;
    }
    if filename_lower.contains("化验") || filename_lower.contains("检验") || filename_lower.contains("lab") {
        return AttachmentType::LabReport;
    }
    if filename_lower.contains("处方") || filename_lower.contains("prescription") || filename_lower.contains("药") {
        return AttachmentType::Prescription;
    }
    if filename_lower.contains("影像") || filename_lower.contains("ct") || filename_lower.contains("x光") || filename_lower.contains("mri") {
        return AttachmentType::Imaging;
    }
    if filename_lower.contains("发票") || filename_lower.contains("invoice") {
        return AttachmentType::Invoice;
    }

    if let Some(ct) = content_type {
        if ct.starts_with("image/") {
            // Default images to medical record
            return AttachmentType::MedicalRecord;
        }
    }

    AttachmentType::Other
}

async fn run_ocr(
    State(state): State<Arc<AppState>>,
    Path(attachment_id): Path<String>,
) -> Result<Json<OcrResultResponse>, StatusCode> {
    let attachment = state.storage.get_attachment(&attachment_id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    // Read file
    let file_path = std::path::Path::new("./data").join(&attachment.file_path);
    let content = std::fs::read(&file_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Initialize OCR registry
    let mut registry = OcrRegistry::new();

    // Add Vision OCR providers
    for (name, provider_config) in &state.config.ocr.providers {
        if provider_config.enabled {
            if let (Some(endpoint), Some(model), Some(api_key)) = (
                &provider_config.endpoint,
                &provider_config.model,
                &provider_config.api_key,
            ) {
                let provider = VisionOcrProvider::new(
                    name.clone(),
                    endpoint.clone(),
                    model.clone(),
                    api_key.clone(),
                    provider_config.timeout,
                );
                registry.register(provider, true);
            }
        }
    }

    let provider = registry.get(Some(&state.config.ocr.default))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Run OCR
    let result = match attachment.mime_type.as_deref() {
        Some("application/pdf") => {
            let results = provider.recognize_pdf(&content).await.map_err(|e| {
                eprintln!("OCR error: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            results.into_iter().next().unwrap_or_else(|| health_keeper_core::ai::OcrResultData {
                text: String::new(),
                confidence: None,
            })
        }
        _ => provider.recognize_image(&content).await.map_err(|e| {
            eprintln!("OCR error: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?,
    };

    // Save OCR result
    let ocr_result = OcrResult::new(
        attachment_id.clone(),
        provider.name().to_string(),
        result.text.clone(),
    );

    state.storage.save_ocr_result(&ocr_result).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(OcrResultResponse {
        id: ocr_result.id,
        attachment_id: ocr_result.attachment_id,
        ocr_provider: ocr_result.ocr_provider,
        recognized_text: ocr_result.recognized_text,
        confidence: ocr_result.confidence,
        processed_at: ocr_result.processed_at.to_rfc3339(),
    }))
}

async fn get_ocr(
    State(state): State<Arc<AppState>>,
    Path(attachment_id): Path<String>,
) -> Result<Json<OcrResultResponse>, StatusCode> {
    let result = state.storage.get_ocr_result(&attachment_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(OcrResultResponse {
        id: result.id,
        attachment_id: result.attachment_id,
        ocr_provider: result.ocr_provider,
        recognized_text: result.recognized_text,
        confidence: result.confidence,
        processed_at: result.processed_at.to_rfc3339(),
    }))
}

async fn run_extraction(
    State(state): State<Arc<AppState>>,
    Path(attachment_id): Path<String>,
) -> Result<Json<ExtractionResponse>, StatusCode> {
    let attachment = state.storage.get_attachment(&attachment_id).await.map_err(|_| StatusCode::NOT_FOUND)?;

    // Get OCR result
    let ocr_result = state.storage.get_ocr_result(&attachment_id).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Initialize LLM registry
    let mut registry = LlmRegistry::new();

    for (name, provider_config) in &state.config.llm.providers {
        if provider_config.enabled {
            if let (Some(endpoint), Some(model), Some(api_key)) = (
                &provider_config.endpoint,
                &provider_config.model,
                &provider_config.api_key,
            ) {
                let provider = AnthropicProvider::new(
                    name.clone(),
                    endpoint.clone(),
                    model.clone(),
                    api_key.clone(),
                    provider_config.timeout,
                );
                registry.register(provider, true);
            }
        }
    }

    let provider = registry.get(Some(&state.config.llm.default))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Run extraction
    let context = ExtractionContext {
        ocr_text: ocr_result.recognized_text,
        document_type: Some(attachment.attachment_type.to_string()),
        person_name: None,
    };

    let result = provider.extract(context).await.map_err(|e| {
        eprintln!("Extraction error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Save extraction
    let extracted = health_keeper_core::models::ExtractedData::new(
        attachment_id.clone(),
        provider.name().to_string(),
        health_keeper_core::models::ExtractedDataType::Summary,
        serde_json::to_string(&result).unwrap_or_default(),
    );

    state.storage.save_extracted_data(&extracted).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ExtractionResponse {
        diagnosis: result.diagnosis,
        chief_complaint: result.chief_complaint,
        treatment: result.treatment,
        medications: result.medications.into_iter().map(|m| MedicationResponse {
            name: m.name,
            dosage: m.dosage,
            frequency: m.frequency,
            duration: m.duration,
        }).collect(),
        follow_up: result.follow_up,
        summary: result.summary,
    }))
}

async fn search(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Json<Vec<VisitResponse>> {
    use health_keeper_core::storage::VisitFilters;

    let filters = VisitFilters {
        query: query.q,
        person_id: query.person_id,
        hospital: query.hospital,
        doctor: query.doctor,
    };

    let visits = state.storage.search_visits(filters).await.unwrap_or_default();

    let responses: Vec<VisitResponse> = visits.into_iter().map(|v| VisitResponse {
        id: v.id,
        person_id: v.person_id,
        visit_date: v.visit_date.to_string(),
        hospital: v.hospital,
        department: v.department,
        doctor: v.doctor,
        chief_complaint: v.chief_complaint,
        diagnosis: v.diagnosis,
        treatment: v.treatment,
        summary: v.summary,
        notes: v.notes,
        created_at: v.created_at.to_rfc3339(),
        updated_at: v.updated_at.to_rfc3339(),
        attachments: vec![],
    }).collect();

    Json(responses)
}

// SSE progress event
#[derive(Debug, Serialize)]
struct ProgressEvent {
    stage: String,
    message: String,
    progress: u8,  // 0-100
}

#[derive(Debug, Serialize)]
struct ProgressComplete {
    #[serde(flatten)]
    data: QuickImportResponse,
}

async fn quick_import(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let mut person_id: Option<String> = None;
    // (data, content_type, original_filename)
    let mut files: Vec<(Vec<u8>, Option<String>, Option<String>)> = Vec::new();

    // Parse multipart form
    println!("[DEBUG] 开始解析 multipart 表单...");
    while let Some(field) = multipart.next_field().await.ok().flatten() {
        let field_name = field.name().map(|s| s.to_string()).unwrap_or_default();

        match field_name.as_str() {
            "person_id" => {
                person_id = field.text().await.ok();
                println!("[DEBUG] 收到 person_id: {:?}", person_id);
            }
            "files" => {
                let ct = field.content_type().map(|s| s.to_string());
                let original_name = field.file_name().map(|s| s.to_string());
                if let Ok(data) = field.bytes().await {
                    println!("[DEBUG] 收到文件, 大小: {} bytes, 类型: {:?}, 原始文件名: {:?}", data.len(), ct, original_name);
                    files.push((data.to_vec(), ct, original_name));
                }
            }
            _ => {}
        }
    }

    // Create progress stream
    let stream = async_stream::stream! {
        // Error helper
        let error_event = |msg: &str| -> Result<Event, axum::Error> {
            Ok(Event::default().json_data(ProgressEvent {
                stage: "error".to_string(),
                message: msg.to_string(),
                progress: 0,
            }).unwrap_or_else(|_| Event::default().data(r#"{"stage":"error","message":"未知错误","progress":0}"#)))
        };

        // Progress helper
        let progress_event = |stage: &str, message: &str, progress: u8| -> Result<Event, axum::Error> {
            Ok(Event::default().json_data(ProgressEvent {
                stage: stage.to_string(),
                message: message.to_string(),
                progress,
            }).unwrap_or_else(|_| Event::default().data("{}")))
        };

        // Validate files
        if files.is_empty() {
            yield error_event("没有收到文件");
            return;
        }

        let total_files = files.len();
        yield progress_event("upload", &format!("已接收 {} 个文件", total_files), 10);

        // Initialize OCR
        yield progress_event("init", "初始化 OCR 服务...", 15);

        let mut ocr_registry = OcrRegistry::new();
        for (name, provider_config) in &state.config.ocr.providers {
            if provider_config.enabled {
                if let (Some(endpoint), Some(model), Some(api_key)) = (
                    &provider_config.endpoint,
                    &provider_config.model,
                    &provider_config.api_key,
                ) {
                    let provider = VisionOcrProvider::new(
                        name.clone(),
                        endpoint.clone(),
                        model.clone(),
                        api_key.clone(),
                        provider_config.timeout,
                    );
                    ocr_registry.register(provider, true);
                }
            }
        }

        let ocr_provider = match ocr_registry.get(Some(&state.config.ocr.default)) {
            Ok(p) => p,
            Err(e) => {
                yield error_event(&format!("OCR 服务初始化失败: {}", e));
                return;
            }
        };

        // Run OCR on each file
        let mut all_ocr_texts = Vec::new();
        let mut attachment_ids: Vec<String> = Vec::new();
        let base_progress = 20;
        let ocr_progress_range = 50; // 20% -> 70%

        // Save files and run OCR
        for (i, (file_data, content_type, original_filename)) in files.iter().enumerate() {
            let file_num = i + 1;
            let progress = base_progress + (file_num * ocr_progress_range / total_files) as u8;

            println!("[DEBUG] 开始处理第 {} 个文件, 大小: {} bytes", file_num, file_data.len());

            // Get file extension from original filename or content type
            let ext = if let Some(name) = original_filename {
                std::path::Path::new(name)
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or_else(|| match content_type.as_deref() {
                        Some("application/pdf") => "pdf",
                        Some("image/png") => "png",
                        Some("image/gif") => "gif",
                        Some("image/webp") => "webp",
                        _ => "jpg",
                    })
            } else {
                match content_type.as_deref() {
                    Some("application/pdf") => "pdf",
                    Some("image/png") => "png",
                    Some("image/gif") => "gif",
                    Some("image/webp") => "webp",
                    _ => "jpg",
                }
            };
            println!("[DEBUG] 文件扩展名: {}", ext);

            // Calculate hash
            println!("[DEBUG] 计算文件哈希...");
            use sha2::{Digest, Sha256};
            let mut hasher = Sha256::new();
            hasher.update(file_data);
            let hash = hex::encode(hasher.finalize());
            println!("[DEBUG] 哈希计算完成: {}...", &hash[..16]);

            let stored_name = format!("{}.{}", hash, ext);
            let relative_path = format!("attachments/{}", stored_name);

            // Save to disk
            println!("[DEBUG] 保存文件到磁盘...");
            let data_dir = std::path::Path::new("./data");
            let attachments_dir = data_dir.join("attachments");
            if let Err(e) = std::fs::create_dir_all(&attachments_dir) {
                println!("[ERROR] 创建目录失败: {}", e);
                yield error_event(&format!("创建目录失败: {}", e));
                return;
            }

            let dest_path = attachments_dir.join(&stored_name);
            if let Err(e) = std::fs::write(&dest_path, file_data) {
                println!("[ERROR] 保存文件失败: {}", e);
                yield error_event(&format!("保存文件失败: {}", e));
                return;
            }
            println!("[DEBUG] 文件已保存: {:?}", dest_path);

            // Create attachment record (without visit_id)
            println!("[DEBUG] 创建附件记录...");
            let attachment = health_keeper_core::models::Attachment::new(
                String::new(), // Empty visit_id, will be linked later
                relative_path.clone(),
                health_keeper_core::models::AttachmentType::MedicalRecord,
            );
            let mut attachment = attachment;
            attachment.file_hash = Some(hash);
            attachment.file_size = Some(file_data.len() as i64);
            attachment.mime_type = content_type.clone();
            attachment.original_filename = original_filename.clone();
            println!("[DEBUG] 附件对象创建完成, file_path: {}", attachment.file_path);

            println!("[DEBUG] 调用 storage.create_attachment...");
            let attachment_id = match state.storage.create_attachment(&attachment).await {
                Ok(id) => {
                    println!("[DEBUG] 附件记录创建成功, id: {}", id);
                    id
                },
                Err(e) => {
                    println!("[ERROR] 保存附件记录失败: {:?}", e);
                    yield error_event(&format!("保存附件记录失败: {}", e));
                    return;
                }
            };
            attachment_ids.push(attachment_id);
            println!("[DEBUG] 保存附件: {}", attachment_ids.last().unwrap());

            yield Ok(Event::default().json_data(ProgressEvent {
                stage: "ocr".to_string(),
                message: format!("OCR 识别第 {}/{} 个文件...", file_num, total_files),
                progress,
            }).unwrap());

            println!("[DEBUG] OCR 识别第 {} 个文件...", file_num);
            let ocr_result = match content_type.as_deref() {
                Some("application/pdf") => {
                    match ocr_provider.recognize_pdf(file_data).await {
                        Ok(results) => results.into_iter().next().unwrap_or_else(|| health_keeper_core::ai::OcrResultData {
                            text: String::new(),
                            confidence: None,
                        }),
                        Err(e) => {
                            yield error_event(&format!("PDF OCR 失败: {}", e));
                            return;
                        }
                    }
                }
                _ => {
                    match ocr_provider.recognize_image(file_data).await {
                        Ok(r) => r,
                        Err(e) => {
                            yield error_event(&format!("图片 OCR 失败: {}", e));
                            return;
                        }
                    }
                }
            };

            println!("[DEBUG] OCR 第 {} 个文件完成, 文本长度: {}", file_num, ocr_result.text.len());
            if !ocr_result.text.is_empty() {
                all_ocr_texts.push(format!("=== 文档 {} ===\n{}", file_num, ocr_result.text));
            }
        }

        let combined_text = all_ocr_texts.join("\n\n");
        println!("[DEBUG] 合并后 OCR 文本总长度: {}", combined_text.len());

        // Initialize LLM
        yield Ok(Event::default().json_data(ProgressEvent {
            stage: "llm_init".to_string(),
            message: "初始化 AI 提取服务...".to_string(),
            progress: 75,
        }).unwrap());

        let mut llm_registry = LlmRegistry::new();
        for (name, provider_config) in &state.config.llm.providers {
            if provider_config.enabled {
                if let (Some(endpoint), Some(model), Some(api_key)) = (
                    &provider_config.endpoint,
                    &provider_config.model,
                    &provider_config.api_key,
                ) {
                    let provider = AnthropicProvider::new(
                        name.clone(),
                        endpoint.clone(),
                        model.clone(),
                        api_key.clone(),
                        provider_config.timeout,
                    );
                    llm_registry.register(provider, true);
                }
            }
        }

        let llm_provider = match llm_registry.get(Some(&state.config.llm.default)) {
            Ok(p) => p,
            Err(e) => {
                yield error_event(&format!("LLM 服务初始化失败: {}", e));
                return;
            }
        };

        // Run extraction
        yield Ok(Event::default().json_data(ProgressEvent {
            stage: "extract".to_string(),
            message: "AI 正在提取医疗信息...".to_string(),
            progress: 80,
        }).unwrap());

        println!("[DEBUG] 开始 LLM 提取...");
        let context = ExtractionContext {
            ocr_text: combined_text.clone(),
            document_type: None,
            person_name: None,
        };

        let extraction = match llm_provider.extract(context).await {
            Ok(e) => e,
            Err(e) => {
                yield error_event(&format!("AI 提取失败: {}", e));
                return;
            }
        };

        println!("[DEBUG] 提取完成:");
        println!("[DEBUG]   - 就诊日期: {:?}", extraction.visit_date);
        println!("[DEBUG]   - 医院: {:?}", extraction.hospital);
        println!("[DEBUG]   - 科室: {:?}", extraction.department);
        println!("[DEBUG]   - 医生: {:?}", extraction.doctor);
        println!("[DEBUG]   - 主诉: {:?}", extraction.chief_complaint);
        println!("[DEBUG]   - 诊断: {:?}", extraction.diagnosis);
        println!("[DEBUG]   - 治疗: {:?}", extraction.treatment);
        println!("[DEBUG]   - 药物: {:?}", extraction.medications);
        println!("[DEBUG]   - 检查结果: {:?}", extraction.lab_results);
        println!("[DEBUG]   - 复诊: {:?}", extraction.follow_up);
        println!("[DEBUG]   - 摘要: {:?}", extraction.summary);

        yield Ok(Event::default().json_data(ProgressEvent {
            stage: "complete".to_string(),
            message: "识别完成！".to_string(),
            progress: 100,
        }).unwrap());

        // Send final result
        yield Ok(Event::default().json_data(ProgressComplete {
            data: QuickImportResponse {
                visit_date: extraction.visit_date,
                hospital: extraction.hospital,
                department: extraction.department,
                doctor: extraction.doctor,
                chief_complaint: extraction.chief_complaint,
                diagnosis: extraction.diagnosis,
                treatment: extraction.treatment,
                medications: extraction.medications.into_iter().map(|m| MedicationResponse {
                    name: m.name,
                    dosage: m.dosage,
                    frequency: m.frequency,
                    duration: m.duration,
                }).collect(),
                lab_results: extraction.lab_results.into_iter().map(|l| LabResultResponse {
                    name: l.name,
                    value: l.value,
                    unit: l.unit,
                    reference_range: l.reference_range,
                    is_abnormal: l.is_abnormal,
                }).collect(),
                follow_up: extraction.follow_up,
                summary: extraction.summary,
                ocr_text: combined_text,
                attachment_ids,
            },
        }).unwrap());
    };

    Sse::new(stream)
}

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../../../apps/web/index.html"))
}

// ==================== Main ====================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = AppConfig::load()?;

    // Initialize storage
    let storage = SqliteStorage::new(&config.database_url()).await?;
    storage.migrate().await?;

    // Create app state
    let state = Arc::new(AppState { storage, config });

    // Build router
    let app = Router::new()
        // Serve index
        .route("/", get(serve_index))
        // API routes
        .route("/api/persons", get(list_persons).post(create_person))
        .route("/api/persons/:id", get(get_person).put(update_person_handler).delete(delete_person))
        .route("/api/visits", get(list_visits).post(create_visit))
        .route("/api/visits/:id", get(get_visit).put(update_visit_handler).delete(delete_visit))
        .route("/api/visits/:id/attachments", post(upload_attachment))
        .route("/api/attachments/:id/ocr", post(run_ocr).get(get_ocr))
        .route("/api/attachments/:id/extract", post(run_extraction))
        .route("/api/search", get(search))
        .route("/api/quick-import", post(quick_import))
        // Static files
        .nest_service("/data", ServeDir::new("./data"))
        // CORS
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any))
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🏥 HealthKeeper Web UI running at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}