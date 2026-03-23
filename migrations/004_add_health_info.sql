-- Migration: 004_add_health_info.sql
-- Add health information fields to persons table

-- Add health information columns
ALTER TABLE persons ADD COLUMN chronic_diseases TEXT;
ALTER TABLE persons ADD COLUMN past_surgeries TEXT;
ALTER TABLE persons ADD COLUMN hospitalizations TEXT;
ALTER TABLE persons ADD COLUMN major_illnesses TEXT;
ALTER TABLE persons ADD COLUMN family_history TEXT;
ALTER TABLE persons ADD COLUMN current_medications TEXT;
ALTER TABLE persons ADD COLUMN lifestyle TEXT;
ALTER TABLE persons ADD COLUMN body_measurements TEXT;