-- 添加 summary 字段到 visits 表
-- 使用安全的添加列方式（如果列已存在会静默失败）

-- SQLite 不支持 IF NOT EXISTS 用于列，所以我们捕获错误
-- 在代码中处理：如果列已存在，忽略错误
ALTER TABLE visits ADD COLUMN summary TEXT;

-- 更新全文搜索触发器以包含 summary
DROP TRIGGER IF EXISTS visits_fts_insert;
DROP TRIGGER IF EXISTS visits_fts_update;

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
            COALESCE(NEW.summary, '') || ' ' ||
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
                      COALESCE(NEW.summary, '') || ' ' ||
                      COALESCE(NEW.notes, '')
        WHERE id = NEW.id;
    END;