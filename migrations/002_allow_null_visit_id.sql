-- 允许 attachments.visit_id 为 NULL（支持先上传文件后关联就诊记录）
-- 仅在表结构需要修改时执行

-- 检查 visit_id 列是否已经是可空的（通过重建表来实现）
-- SQLite 不支持 ALTER COLUMN，所以使用重建表的方式

-- 如果已有数据，先检查 visit_id 是否已允许 NULL
-- 如果允许，此迁移会因表名冲突而失败，但这是预期的（迁移已完成）
-- 因此我们使用安全的迁移方式

-- 备份原表（如果需要）
-- ALTER TABLE attachments RENAME TO attachments_old;

-- 创建新表结构
-- CREATE TABLE IF NOT EXISTS attachments (
--     id TEXT PRIMARY KEY,
--     visit_id TEXT,
--     type TEXT NOT NULL DEFAULT 'other',
--     file_path TEXT NOT NULL,
--     file_hash TEXT,
--     file_size INTEGER,
--     mime_type TEXT,
--     original_filename TEXT,
--     created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
--     FOREIGN KEY (visit_id) REFERENCES visits(id) ON DELETE CASCADE
-- );

-- 迁移数据
-- INSERT INTO attachments SELECT * FROM attachments_old;

-- 删除旧表
-- DROP TABLE attachments_old;

-- 重建索引
-- CREATE INDEX IF NOT EXISTS idx_attachments_visit_id ON attachments(visit_id);

-- 由于 SQLite 迁移复杂，我们改用程序化方式在 migrate 中处理
-- 此文件保留为空（迁移已在代码中处理）
SELECT 1;