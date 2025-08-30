DROP INDEX IF EXISTS idx_temp_registrations_email;
DROP INDEX IF EXISTS idx_temp_registrations_secret_key;
DROP INDEX IF EXISTS idx_temp_registrations_expires_at;

DROP TABLE IF EXISTS temp_registrations;

DELETE FROM schema_migrations WHERE version = 4;
