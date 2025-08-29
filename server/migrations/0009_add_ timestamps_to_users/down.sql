ALTER TABLE users DROP COLUMN IF EXISTS created_at;
ALTER TABLE users DROP COLUMN IF EXISTS updated_at;

ALTER TABLE temp_registrations DROP COLUMN IF EXISTS password;
ALTER TABLE temp_registrations ADD COLUMN IF NOT EXISTS attempts INTEGER DEFAULT 0;
ALTER TABLE temp_registrations ADD COLUMN IF NOT EXISTS password_hash VARCHAR(255);

DELETE FROM schema_migrations WHERE version = 9;
