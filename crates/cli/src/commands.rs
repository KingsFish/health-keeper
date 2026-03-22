//! CLI command handlers

use crate::{
    ExtractCommand, ImportCommand, OcrCommand, PersonCommands, SearchCommand, VisitCommands,
};
use anyhow::{anyhow, Result};
use chrono::NaiveDate;
use health_keeper_core::{
    ai::{ExtractionContext, LlmRegistry, OcrRegistry},
    models::{Attachment, AttachmentType, PersonBuilder, Relationship, VisitBuilder},
    storage::Storage,
    AppConfig,
};
use std::path::Path;

pub async fn handle_person<S: Storage>(storage: &S, cmd: PersonCommands) -> Result<()> {
    match cmd {
        PersonCommands::Create {
            name,
            relationship,
            birth_date,
            gender,
            blood_type,
            notes,
        } => {
            let rel: Relationship = relationship.parse().map_err(|e: String| anyhow!(e))?;
            let mut builder = PersonBuilder::new().name(name).relationship(rel);

            if let Some(date) = birth_date {
                builder = builder.birth_date(NaiveDate::parse_from_str(&date, "%Y-%m-%d")?);
            }
            if let Some(g) = gender {
                builder = builder.gender(g.parse().map_err(|e: String| anyhow!(e))?);
            }
            if let Some(bt) = blood_type {
                builder = builder.blood_type(bt.parse().map_err(|e: String| anyhow!(e))?);
            }
            if let Some(n) = notes {
                builder = builder.notes(n);
            }

            let person = builder.build().map_err(|e: String| anyhow!(e))?;
            let id = storage.create_person(&person).await?;

            println!("Created person: {}", id);
            println!("  Name: {}", person.name);
            println!("  Relationship: {}", person.relationship);
        }
        PersonCommands::List => {
            let persons = storage.list_persons().await?;

            if persons.is_empty() {
                println!("No persons found.");
                return Ok(());
            }

            println!("{:<36} {:<20} {:<15}", "ID", "Name", "Relationship");
            println!("{}", "-".repeat(71));

            for p in persons {
                println!("{:<36} {:<20} {:<15}", p.id, p.name, p.relationship);
            }
        }
        PersonCommands::Show { id } => {
            let person = storage.get_person(&id).await?;

            println!("ID: {}", person.id);
            println!("Name: {}", person.name);
            println!("Relationship: {}", person.relationship);

            if let Some(date) = person.birth_date {
                println!("Birth Date: {}", date);
            }
            if let Some(gender) = person.gender {
                println!("Gender: {}", gender);
            }
            if let Some(bt) = person.blood_type {
                println!("Blood Type: {}", bt);
            }
            if let Some(allergies) = &person.allergies {
                println!("Allergies: {}", allergies.join(", "));
            }
            if let Some(notes) = &person.notes {
                println!("Notes: {}", notes);
            }
        }
        PersonCommands::Update {
            id,
            name,
            relationship,
            birth_date,
            gender,
            blood_type,
            notes,
        } => {
            let mut person = storage.get_person(&id).await?;

            if let Some(n) = name {
                person.name = n;
            }
            if let Some(r) = relationship {
                person.relationship = r.parse().map_err(|e: String| anyhow!(e))?;
            }
            if let Some(date) = birth_date {
                person.birth_date = Some(NaiveDate::parse_from_str(&date, "%Y-%m-%d")?);
            }
            if let Some(g) = gender {
                person.gender = Some(g.parse().map_err(|e: String| anyhow!(e))?);
            }
            if let Some(bt) = blood_type {
                person.blood_type = Some(bt.parse().map_err(|e: String| anyhow!(e))?);
            }
            if let Some(n) = notes {
                person.notes = Some(n);
            }

            storage.update_person(&person).await?;
            println!("Updated person: {}", id);
        }
        PersonCommands::Delete { id, yes } => {
            if !yes {
                println!("Are you sure you want to delete person {}? (y/N)", id);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }

            storage.delete_person(&id).await?;
            println!("Deleted person: {}", id);
        }
    }

    Ok(())
}

pub async fn handle_visit<S: Storage>(storage: &S, cmd: VisitCommands) -> Result<()> {
    match cmd {
        VisitCommands::Create {
            person,
            date,
            hospital,
            department,
            doctor,
            complaint,
            diagnosis,
            treatment,
            notes,
        } => {
            let visit_date = NaiveDate::parse_from_str(&date, "%Y-%m-%d")?;
            let mut builder = VisitBuilder::new()
                .person_id(person)
                .visit_date(visit_date);

            if let Some(h) = hospital {
                builder = builder.hospital(h);
            }
            if let Some(d) = department {
                builder = builder.department(d);
            }
            if let Some(d) = doctor {
                builder = builder.doctor(d);
            }
            if let Some(c) = complaint {
                builder = builder.chief_complaint(c);
            }
            if let Some(d) = diagnosis {
                builder = builder.diagnosis(d);
            }
            if let Some(t) = treatment {
                builder = builder.treatment(t);
            }
            if let Some(n) = notes {
                builder = builder.notes(n);
            }

            let visit = builder.build().map_err(|e: String| anyhow!(e))?;
            let id = storage.create_visit(&visit).await?;

            println!("Created visit: {}", id);
            println!("  Date: {}", visit.visit_date);
        }
        VisitCommands::List { person } => {
            let visits = storage.list_visits(person.as_deref()).await?;

            if visits.is_empty() {
                println!("No visits found.");
                return Ok(());
            }

            println!(
                "{:<36} {:<12} {:<20} {:<15}",
                "ID", "Date", "Hospital", "Department"
            );
            println!("{}", "-".repeat(83));

            for v in visits {
                println!(
                    "{:<36} {:<12} {:<20} {:<15}",
                    v.id,
                    v.visit_date.to_string(),
                    v.hospital.unwrap_or_default(),
                    v.department.unwrap_or_default()
                );
            }
        }
        VisitCommands::Show { id } => {
            let visit = storage.get_visit(&id).await?;

            println!("ID: {}", visit.id);
            println!("Person ID: {}", visit.person_id);
            println!("Date: {}", visit.visit_date);

            if let Some(h) = visit.hospital {
                println!("Hospital: {}", h);
            }
            if let Some(d) = visit.department {
                println!("Department: {}", d);
            }
            if let Some(d) = visit.doctor {
                println!("Doctor: {}", d);
            }
            if let Some(c) = visit.chief_complaint {
                println!("Chief Complaint: {}", c);
            }
            if let Some(d) = visit.diagnosis {
                println!("Diagnosis: {}", d);
            }
            if let Some(t) = visit.treatment {
                println!("Treatment: {}", t);
            }
            if let Some(n) = visit.notes {
                println!("Notes: {}", n);
            }

            // Show attachments
            let attachments = storage.list_attachments(&id).await?;
            if !attachments.is_empty() {
                println!("\nAttachments:");
                for a in attachments {
                    println!("  - {} [{}] {}", a.id, a.attachment_type, a.original_filename.unwrap_or_default());
                }
            }
        }
        VisitCommands::Update {
            id,
            date,
            hospital,
            department,
            doctor,
            complaint,
            diagnosis,
            treatment,
            notes,
        } => {
            let mut visit = storage.get_visit(&id).await?;

            if let Some(d) = date {
                visit.visit_date = NaiveDate::parse_from_str(&d, "%Y-%m-%d")?;
            }
            if let Some(h) = hospital {
                visit.hospital = Some(h);
            }
            if let Some(d) = department {
                visit.department = Some(d);
            }
            if let Some(d) = doctor {
                visit.doctor = Some(d);
            }
            if let Some(c) = complaint {
                visit.chief_complaint = Some(c);
            }
            if let Some(d) = diagnosis {
                visit.diagnosis = Some(d);
            }
            if let Some(t) = treatment {
                visit.treatment = Some(t);
            }
            if let Some(n) = notes {
                visit.notes = Some(n);
            }

            storage.update_visit(&visit).await?;
            println!("Updated visit: {}", id);
        }
        VisitCommands::Delete { id, yes } => {
            if !yes {
                println!("Are you sure you want to delete visit {}? (y/N)", id);
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Cancelled.");
                    return Ok(());
                }
            }

            storage.delete_visit(&id).await?;
            println!("Deleted visit: {}", id);
        }
    }

    Ok(())
}

pub async fn handle_import<S: Storage>(storage: &S, cmd: ImportCommand) -> Result<()> {
    // Verify visit exists
    storage.get_visit(&cmd.visit).await?;

    // Read file
    let file_path = cmd.file.clone();
    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    let content = std::fs::read(&file_path)?;
    let file_size = content.len() as i64;

    // Calculate hash
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let hash = hex::encode(hasher.finalize());

    // Determine MIME type
    let mime_type = mime_guess::from_path(&file_path)
        .first()
        .map(|m| m.to_string());

    // Determine attachment type
    let attachment_type: AttachmentType = cmd.attachment_type.parse().map_err(|e: String| anyhow!(e))?;

    // Generate storage path
    let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let stored_name = format!("{}.{}", hash, ext);
    let relative_path = format!("attachments/{}", stored_name);

    // Copy file to data directory
    let data_dir = Path::new("./data");
    let attachments_dir = data_dir.join("attachments");
    std::fs::create_dir_all(&attachments_dir)?;

    let dest_path = attachments_dir.join(&stored_name);
    std::fs::copy(&file_path, &dest_path)?;

    // Create attachment record
    let attachment = Attachment {
        id: health_keeper_core::models::generate_id(),
        visit_id: cmd.visit,
        attachment_type,
        file_path: relative_path,
        file_hash: Some(hash),
        file_size: Some(file_size),
        mime_type,
        original_filename: Some(file_name.clone()),
        created_at: chrono::Utc::now(),
    };

    let id = storage.create_attachment(&attachment).await?;

    println!("Imported file: {}", file_name);
    println!("  Attachment ID: {}", id);
    println!("  Type: {}", attachment.attachment_type);
    println!("  Size: {} bytes", file_size);

    Ok(())
}

pub async fn handle_ocr<S: Storage>(
    storage: &S,
    config: &AppConfig,
    cmd: OcrCommand,
) -> Result<()> {
    let attachment = storage.get_attachment(&cmd.attachment).await?;

    // Read file
    let file_path = Path::new("./data").join(&attachment.file_path);
    let content = std::fs::read(&file_path)?;

    // Initialize OCR registry
    let mut registry = OcrRegistry::new();

    // Add PaddleOCR if configured
    if let Some(paddle_config) = config.ocr.providers.get("paddle_local") {
        if paddle_config.enabled {
            let endpoint = paddle_config
                .endpoint
                .clone()
                .unwrap_or_else(|| "http://127.0.0.1:8868".to_string());
            let provider = health_keeper_core::ai::PaddleOcrProvider::new(
                endpoint,
                paddle_config.timeout,
            );
            registry.register(provider, true);
        }
    }

    // Add Vision-based OCR providers (qwen_vision, etc.)
    for (name, provider_config) in &config.ocr.providers {
        if name == "paddle_local" {
            continue; // Already handled
        }
        if provider_config.enabled {
            if let (Some(endpoint), Some(model), Some(api_key)) = (
                &provider_config.endpoint,
                &provider_config.model,
                &provider_config.api_key,
            ) {
                let provider = health_keeper_core::ai::VisionOcrProvider::new(
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

    // Set default provider
    if let Err(_) = registry.set_default(&config.ocr.default) {
        // Ignore if default not found
    }

    let provider = registry.get(cmd.provider.as_deref())?;

    println!("Running OCR with provider: {}...", provider.name());

    let result = match attachment.mime_type.as_deref() {
        Some("application/pdf") => {
            let results = provider.recognize_pdf(&content).await?;
            results.into_iter().next().unwrap_or_else(|| {
                health_keeper_core::ai::OcrResultData {
                    text: String::new(),
                    confidence: None,
                }
            })
        }
        _ => provider.recognize_image(&content).await?,
    };

    // Save OCR result
    let ocr_result = health_keeper_core::models::OcrResult::new(
        attachment.id.clone(),
        provider.name().to_string(),
        result.text.clone(),
    );

    storage.save_ocr_result(&ocr_result).await?;

    println!("OCR completed.");
    println!("Text length: {} characters", result.text.len());
    println!("\n--- Recognized Text ---\n{}", result.text);

    Ok(())
}

pub async fn handle_extract<S: Storage>(
    storage: &S,
    config: &AppConfig,
    cmd: ExtractCommand,
) -> Result<()> {
    let attachment = storage.get_attachment(&cmd.attachment).await?;

    // Get OCR result
    let ocr_result = storage
        .get_ocr_result(&cmd.attachment)
        .await?
        .ok_or_else(|| anyhow!("No OCR result found for this attachment. Run OCR first."))?;

    // Initialize LLM registry
    let mut registry = LlmRegistry::new();

    // Add Ollama if configured
    if let Some(ollama_config) = config.llm.providers.get("ollama_local") {
        if ollama_config.enabled {
            let endpoint = ollama_config
                .endpoint
                .clone()
                .unwrap_or_else(|| "http://127.0.0.1:11434".to_string());
            let model = ollama_config
                .model
                .clone()
                .unwrap_or_else(|| "qwen2.5:7b".to_string());
            let provider = health_keeper_core::ai::OllamaProvider::new(
                endpoint,
                model,
                ollama_config.timeout,
            );
            registry.register(provider, true);
        }
    }

    // Add Anthropic-compatible providers (dashscope, claude, etc.)
    for (name, provider_config) in &config.llm.providers {
        if name == "ollama_local" || name == "paddle_local" {
            continue; // Already handled
        }
        if provider_config.enabled {
            if let (Some(endpoint), Some(model), Some(api_key)) = (
                &provider_config.endpoint,
                &provider_config.model,
                &provider_config.api_key,
            ) {
                let provider = health_keeper_core::ai::AnthropicProvider::new(
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

    // Set default provider
    if let Err(_) = registry.set_default(&config.llm.default) {
        // Ignore if default not found
    }

    let provider = registry.get(cmd.provider.as_deref())?;

    println!("Extracting data with provider: {}...", provider.name());

    let context = ExtractionContext {
        ocr_text: ocr_result.recognized_text,
        document_type: Some(attachment.attachment_type.to_string()),
        person_name: None,
    };

    let result = provider.extract(context).await?;

    // Print results
    println!("\n--- Extraction Results ---");

    if let Some(ref d) = result.diagnosis {
        println!("Diagnosis: {}", d);
    }
    if let Some(ref c) = result.chief_complaint {
        println!("Chief Complaint: {}", c);
    }
    if let Some(ref t) = result.treatment {
        println!("Treatment: {}", t);
    }

    if !result.medications.is_empty() {
        println!("\nMedications:");
        for m in &result.medications {
            let dosage = m.dosage.as_deref().unwrap_or("");
            let freq = m.frequency.as_deref().unwrap_or("");
            let dur = m.duration.as_deref().unwrap_or("");
            println!("  - {} {} {} {}", m.name, dosage, freq, dur);
        }
    }

    if !result.lab_results.is_empty() {
        println!("\nLab Results:");
        for l in &result.lab_results {
            let unit = l.unit.as_deref().unwrap_or("");
            let range = l.reference_range.as_deref().unwrap_or("");
            println!("  - {}: {} {} ({})", l.name, l.value, unit, range);
        }
    }

    if let Some(ref f) = result.follow_up {
        println!("\nFollow-up: {}", f);
    }

    if let Some(ref s) = result.summary {
        println!("\nSummary: {}", s);
    }

    // Save extracted data
    let extracted = health_keeper_core::models::ExtractedData::new(
        attachment.id.clone(),
        provider.name().to_string(),
        health_keeper_core::models::ExtractedDataType::Summary,
        serde_json::to_string(&result)?,
    );

    storage.save_extracted_data(&extracted).await?;

    println!("\nExtraction saved.");

    Ok(())
}

pub async fn handle_search<S: Storage>(storage: &S, cmd: SearchCommand) -> Result<()> {
    let visits = storage.search(&cmd.query).await?;

    if visits.is_empty() {
        println!("No results found for: {}", cmd.query);
        return Ok(());
    }

    println!("Found {} result(s) for: {}", visits.len(), cmd.query);
    println!();

    for v in visits {
        println!("Visit: {} ({})", v.id, v.visit_date);
        if let Some(h) = v.hospital {
            println!("  Hospital: {}", h);
        }
        if let Some(d) = v.diagnosis {
            println!("  Diagnosis: {}", d);
        }
        if let Some(c) = v.chief_complaint {
            println!("  Complaint: {}", c);
        }
        println!();
    }

    Ok(())
}