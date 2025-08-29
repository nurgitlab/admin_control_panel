ALTER TABLE users DROP email password;

DELETE FROM schema_migrations WHERE version = 7;
