-- 允许 attachments.visit_id 为 NULL（支持先上传文件后关联就诊记录）
CREATE TABLE IF NOT EXISTS attachments_new (
    id TEXT PRIMARY KEY,
    visit_id TEXT,
    type TEXT NOT NULL DEFAULT 'other',
    file_path TEXT NOT NULL,
    file_hash TEXT,
    file_size INTEGER,
    mime_type TEXT,
    original_filename TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (visit_id) REFERENCES visits(id) ON DELETE CASCADE
);

INSERT INTO attachments_new SELECT * FROM attachments;
DROP TABLE attachments;
ALTER TABLE attachments_new RENAME TO attachments;

CREATE INDEX IF NOT EXISTS idx_attachments_visit_id ON attachments(visit_id);