//! Integration tests for SQLite storage

use chrono::NaiveDate;
use health_keeper_core::models::{Attachment, AttachmentType, Person, Relationship, Visit};
use health_keeper_core::storage::{SqliteStorage, Storage, VisitFilters};

async fn create_test_storage() -> SqliteStorage {
    let storage = SqliteStorage::new_in_memory().await.expect("Failed to create in-memory storage");
    storage.migrate().await.expect("Failed to run migrations");
    storage
}

#[tokio::test]
async fn test_person_crud() {
    let storage = create_test_storage().await;

    // Create
    let person = Person::new("张三".to_string(), Relationship::Self_);
    let id = storage.create_person(&person).await.expect("Failed to create person");
    assert_eq!(id, person.id);

    // Read
    let retrieved = storage.get_person(&id).await.expect("Failed to get person");
    assert_eq!(retrieved.name, "张三");
    assert_eq!(retrieved.relationship, Relationship::Self_);

    // Update
    let mut updated = retrieved.clone();
    updated.name = "李四".to_string();
    storage.update_person(&updated).await.expect("Failed to update person");
    let retrieved = storage.get_person(&id).await.expect("Failed to get person");
    assert_eq!(retrieved.name, "李四");

    // Delete
    storage.delete_person(&id).await.expect("Failed to delete person");
    let result = storage.get_person(&id).await;
    assert!(result.is_err());

    storage.close().await;
}

#[tokio::test]
async fn test_person_list() {
    let storage = create_test_storage().await;

    // Create multiple persons
    let person1 = Person::new("张三".to_string(), Relationship::Self_);
    let person2 = Person::new("李四".to_string(), Relationship::Spouse);
    storage.create_person(&person1).await.expect("Failed to create person1");
    storage.create_person(&person2).await.expect("Failed to create person2");

    // List all
    let persons = storage.list_persons().await.expect("Failed to list persons");
    assert_eq!(persons.len(), 2);

    storage.close().await;
}

#[tokio::test]
async fn test_visit_crud() {
    let storage = create_test_storage().await;

    // First create a person
    let person = Person::new("张三".to_string(), Relationship::Self_);
    storage.create_person(&person).await.expect("Failed to create person");

    // Create visit
    let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).expect("Invalid date");
    let visit = Visit::new(person.id.clone(), visit_date)
        .with_hospital("测试医院".to_string())
        .with_department("内科".to_string())
        .with_diagnosis("感冒".to_string());

    let id = storage.create_visit(&visit).await.expect("Failed to create visit");
    assert_eq!(id, visit.id);

    // Read
    let retrieved = storage.get_visit(&id).await.expect("Failed to get visit");
    assert_eq!(retrieved.hospital, Some("测试医院".to_string()));
    assert_eq!(retrieved.diagnosis, Some("感冒".to_string()));

    // Update
    let mut updated = retrieved.clone();
    updated.diagnosis = Some("流感".to_string());
    storage.update_visit(&updated).await.expect("Failed to update visit");
    let retrieved = storage.get_visit(&id).await.expect("Failed to get visit");
    assert_eq!(retrieved.diagnosis, Some("流感".to_string()));

    // Delete
    storage.delete_visit(&id).await.expect("Failed to delete visit");
    let result = storage.get_visit(&id).await;
    assert!(result.is_err());

    storage.close().await;
}

#[tokio::test]
async fn test_visit_list_by_person() {
    let storage = create_test_storage().await;

    // Create two persons
    let person1 = Person::new("张三".to_string(), Relationship::Self_);
    let person2 = Person::new("李四".to_string(), Relationship::Spouse);
    storage.create_person(&person1).await.expect("Failed to create person1");
    storage.create_person(&person2).await.expect("Failed to create person2");

    // Create visits for each person
    let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).expect("Invalid date");
    let visit1 = Visit::new(person1.id.clone(), visit_date);
    let visit2 = Visit::new(person2.id.clone(), visit_date);
    storage.create_visit(&visit1).await.expect("Failed to create visit1");
    storage.create_visit(&visit2).await.expect("Failed to create visit2");

    // List visits for person1
    let visits = storage.list_visits(Some(&person1.id)).await.expect("Failed to list visits");
    assert_eq!(visits.len(), 1);
    assert_eq!(visits[0].person_id, person1.id);

    // List all visits
    let all_visits = storage.list_visits(None).await.expect("Failed to list all visits");
    assert_eq!(all_visits.len(), 2);

    storage.close().await;
}

#[tokio::test]
async fn test_visit_date_range() {
    let storage = create_test_storage().await;

    // Create person
    let person = Person::new("张三".to_string(), Relationship::Self_);
    storage.create_person(&person).await.expect("Failed to create person");

    // Create visits with different dates
    let visit1_date = NaiveDate::from_ymd_opt(2026, 3, 10).expect("Invalid date");
    let visit2_date = NaiveDate::from_ymd_opt(2026, 3, 20).expect("Invalid date");
    let visit3_date = NaiveDate::from_ymd_opt(2026, 3, 30).expect("Invalid date");

    let visit1 = Visit::new(person.id.clone(), visit1_date);
    let visit2 = Visit::new(person.id.clone(), visit2_date);
    let visit3 = Visit::new(person.id.clone(), visit3_date);

    storage.create_visit(&visit1).await.expect("Failed to create visit1");
    storage.create_visit(&visit2).await.expect("Failed to create visit2");
    storage.create_visit(&visit3).await.expect("Failed to create visit3");

    // Query date range
    let start = NaiveDate::from_ymd_opt(2026, 3, 15).expect("Invalid date");
    let end = NaiveDate::from_ymd_opt(2026, 3, 25).expect("Invalid date");

    let visits = storage.list_visits_by_date_range(&person.id, start, end)
        .await
        .expect("Failed to list visits by date range");

    assert_eq!(visits.len(), 1);
    assert_eq!(visits[0].visit_date, visit2_date);

    storage.close().await;
}

#[tokio::test]
async fn test_attachment_crud() {
    let storage = create_test_storage().await;

    // Create person and visit
    let person = Person::new("张三".to_string(), Relationship::Self_);
    storage.create_person(&person).await.expect("Failed to create person");

    let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).expect("Invalid date");
    let visit = Visit::new(person.id.clone(), visit_date);
    storage.create_visit(&visit).await.expect("Failed to create visit");

    // Create attachment
    let attachment = Attachment::new(
        visit.id.clone(),
        "/data/test.jpg".to_string(),
        AttachmentType::MedicalRecord,
    )
    .with_hash("abc123".to_string())
    .with_size(1024)
    .with_mime_type("image/jpeg".to_string())
    .with_original_filename("test.jpg".to_string());

    let id = storage.create_attachment(&attachment).await.expect("Failed to create attachment");
    assert_eq!(id, attachment.id);

    // Read
    let retrieved = storage.get_attachment(&id).await.expect("Failed to get attachment");
    assert_eq!(retrieved.file_path, "/data/test.jpg");
    assert_eq!(retrieved.attachment_type, AttachmentType::MedicalRecord);

    // List by visit
    let attachments = storage.list_attachments(&visit.id).await.expect("Failed to list attachments");
    assert_eq!(attachments.len(), 1);

    // Delete
    storage.delete_attachment(&id).await.expect("Failed to delete attachment");
    let result = storage.get_attachment(&id).await;
    assert!(result.is_err());

    storage.close().await;
}

#[tokio::test]
async fn test_search_visits_with_filters() {
    let storage = create_test_storage().await;

    // Create person
    let person = Person::new("张三".to_string(), Relationship::Self_);
    storage.create_person(&person).await.expect("Failed to create person");

    // Create visits
    let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).expect("Invalid date");
    let visit1 = Visit::new(person.id.clone(), visit_date)
        .with_hospital("北京医院".to_string())
        .with_doctor("王医生".to_string());
    let visit2 = Visit::new(person.id.clone(), visit_date)
        .with_hospital("上海医院".to_string())
        .with_doctor("李医生".to_string());

    storage.create_visit(&visit1).await.expect("Failed to create visit1");
    storage.create_visit(&visit2).await.expect("Failed to create visit2");

    // Search by person_id
    let filters = VisitFilters {
        query: None,
        person_id: Some(person.id.clone()),
        hospital: None,
        doctor: None,
    };
    let results = storage.search_visits(filters).await.expect("Failed to search visits");
    assert_eq!(results.len(), 2);

    // Search by hospital
    let filters = VisitFilters {
        query: None,
        person_id: None,
        hospital: Some("北京".to_string()),
        doctor: None,
    };
    let results = storage.search_visits(filters).await.expect("Failed to search visits");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].hospital, Some("北京医院".to_string()));

    // Search by doctor
    let filters = VisitFilters {
        query: None,
        person_id: None,
        hospital: None,
        doctor: Some("李".to_string()),
    };
    let results = storage.search_visits(filters).await.expect("Failed to search visits");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].doctor, Some("李医生".to_string()));

    storage.close().await;
}

#[tokio::test]
async fn test_not_found_errors() {
    let storage = create_test_storage().await;

    // Test get non-existent person
    let result = storage.get_person("non-existent-id").await;
    assert!(result.is_err());

    // Test get non-existent visit
    let result = storage.get_visit("non-existent-id").await;
    assert!(result.is_err());

    // Test get non-existent attachment
    let result = storage.get_attachment("non-existent-id").await;
    assert!(result.is_err());

    // Test delete non-existent person
    let result = storage.delete_person("non-existent-id").await;
    assert!(result.is_err());

    // Test delete non-existent visit
    let result = storage.delete_visit("non-existent-id").await;
    assert!(result.is_err());

    storage.close().await;
}

#[tokio::test]
async fn test_cascade_delete_person_with_visits() {
    let storage = create_test_storage().await;

    // Create person
    let person = Person::new("张三".to_string(), Relationship::Self_);
    storage.create_person(&person).await.expect("Failed to create person");

    // Create visits for the person
    let visit_date = NaiveDate::from_ymd_opt(2026, 3, 22).expect("Invalid date");
    let visit1 = Visit::new(person.id.clone(), visit_date)
        .with_hospital("医院A".to_string());
    let visit2 = Visit::new(person.id.clone(), visit_date)
        .with_hospital("医院B".to_string());
    storage.create_visit(&visit1).await.expect("Failed to create visit1");
    storage.create_visit(&visit2).await.expect("Failed to create visit2");

    // Create attachments for visit1
    let attachment = Attachment::new(
        visit1.id.clone(),
        "/data/test.jpg".to_string(),
        AttachmentType::MedicalRecord,
    );
    storage.create_attachment(&attachment).await.expect("Failed to create attachment");

    // Verify visits exist
    let visits = storage.list_visits(Some(&person.id)).await.expect("Failed to list visits");
    assert_eq!(visits.len(), 2);

    // Delete person
    storage.delete_person(&person.id).await.expect("Failed to delete person");

    // Verify person is deleted
    let result = storage.get_person(&person.id).await;
    assert!(result.is_err());

    // Verify visits are cascade deleted
    let visits = storage.list_visits(Some(&person.id)).await.expect("Failed to list visits");
    assert_eq!(visits.len(), 0, "Visits should be cascade deleted when person is deleted");

    // Verify individual visits are gone
    let result = storage.get_visit(&visit1.id).await;
    assert!(result.is_err(), "Visit1 should be deleted");

    let result = storage.get_visit(&visit2.id).await;
    assert!(result.is_err(), "Visit2 should be deleted");

    // Verify attachments are cascade deleted
    let result = storage.get_attachment(&attachment.id).await;
    assert!(result.is_err(), "Attachment should be cascade deleted");

    storage.close().await;
}