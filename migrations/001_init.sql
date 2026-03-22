-- HealthKeeper Database Schema v1
-- Migration: 001_init.sql

-- 人物档案表
CREATE TABLE IF NOT EXISTS persons (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    relationship TEXT NOT NULL DEFAULT 'self',
    birth_date DATE,
    gender TEXT,
    blood_type TEXT,
    allergies TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- 就诊记录表
CREATE TABLE IF NOT EXISTS visits (
    id TEXT PRIMARY KEY,
    person_id TEXT NOT NULL,
    visit_date DATE NOT NULL,
    hospital TEXT,
    department TEXT,
    doctor TEXT,
    chief_complaint TEXT,
    diagnosis TEXT,
    treatment TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (person_id) REFERENCES persons(id) ON DELETE CASCADE
);

-- 附件文件表
CREATE TABLE IF NOT EXISTS attachments (
    id TEXT PRIMARY KEY,
    visit_id TEXT NOT NULL,
    type TEXT NOT NULL DEFAULT 'other',
    file_path TEXT NOT NULL,
    file_hash TEXT,
    file_size INTEGER,
    mime_type TEXT,
    original_filename TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (visit_id) REFERENCES visits(id) ON DELETE CASCADE
);

-- OCR 识别结果表
CREATE TABLE IF NOT EXISTS ocr_results (
    id TEXT PRIMARY KEY,
    attachment_id TEXT NOT NULL,
    ocr_provider TEXT NOT NULL,
    recognized_text TEXT NOT NULL,
    confidence REAL,
    processed_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (attachment_id) REFERENCES attachments(id) ON DELETE CASCADE
);

-- AI 提取数据表
CREATE TABLE IF NOT EXISTS extracted_data (
    id TEXT PRIMARY KEY,
    attachment_id TEXT NOT NULL,
    llm_provider TEXT NOT NULL,
    data_type TEXT NOT NULL,
    content TEXT NOT NULL,
    confidence REAL,
    extracted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (attachment_id) REFERENCES attachments(id) ON DELETE CASCADE
);

-- 全文搜索虚拟表
CREATE VIRTUAL TABLE IF NOT EXISTS visits_fts USING fts5(
    id UNINDEXED,
    person_id UNINDEXED,
    content,
    tokenize = 'unicode61'
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_visits_person_id ON visits(person_id);
CREATE INDEX IF NOT EXISTS idx_visits_date ON visits(visit_date);
CREATE INDEX IF NOT EXISTS idx_attachments_visit_id ON attachments(visit_id);
CREATE INDEX IF NOT EXISTS idx_ocr_attachment_id ON ocr_results(attachment_id);
CREATE INDEX IF NOT EXISTS idx_extracted_attachment_id ON extracted_data(attachment_id);

-- 触发器：自动更新 updated_at
CREATE TRIGGER IF NOT EXISTS update_persons_timestamp
    AFTER UPDATE ON persons
    BEGIN
        UPDATE persons SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;

CREATE TRIGGER IF NOT EXISTS update_visits_timestamp
    AFTER UPDATE ON visits
    BEGIN
        UPDATE visits SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
    END;

-- 触发器：同步全文搜索索引
CREATE TRIGGER IF NOT EXISTS visits_fts_insert
    AFTER INSERT ON visits
    BEGIN
        INSERT INTO visits_fts(id, person_id, content)
        VALUES (
            NEW.id,
            NEW.person_id,
            COALESCE(NEW.hospital, '') || ' ' ||
            COALESCE(NEW.department, '') || ' ' ||
            COALESCE(NEW.doctor, '') || ' ' ||
            COALESCE(NEW.chief_complaint, '') || ' ' ||
            COALESCE(NEW.diagnosis, '') || ' ' ||
            COALESCE(NEW.treatment, '') || ' ' ||
            COALESCE(NEW.notes, '')
        );
    END;

CREATE TRIGGER IF NOT EXISTS visits_fts_update
    AFTER UPDATE ON visits
    BEGIN
        UPDATE visits_fts SET
            content = COALESCE(NEW.hospital, '') || ' ' ||
                      COALESCE(NEW.department, '') || ' ' ||
                      COALESCE(NEW.doctor, '') || ' ' ||
                      COALESCE(NEW.chief_complaint, '') || ' ' ||
                      COALESCE(NEW.diagnosis, '') || ' ' ||
                      COALESCE(NEW.treatment, '') || ' ' ||
                      COALESCE(NEW.notes, '')
        WHERE id = NEW.id;
    END;

CREATE TRIGGER IF NOT EXISTS visits_fts_delete
    AFTER DELETE ON visits
    BEGIN
        DELETE FROM visits_fts WHERE id = OLD.id;
    END;