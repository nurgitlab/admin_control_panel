ALTER TABLE users DROP COLUMN password;

DELETE FROM schema_migrations WHERE version = 2;