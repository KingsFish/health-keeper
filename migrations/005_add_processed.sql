-- Add processed column to attachments table
-- Migration: 005_add_processed.sql

ALTER TABLE attachments ADD COLUMN processed BOOLEAN NOT NULL DEFAULT 0;