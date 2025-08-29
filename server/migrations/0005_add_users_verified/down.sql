ALTER TABLE users DROP COLUMN is_verified;

DELETE FROM schema_migrations WHERE version = 5;
