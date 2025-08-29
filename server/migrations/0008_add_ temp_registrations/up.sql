CREATE TABLE temp_registrations (id SERIAL PRIMARY KEY, email VARCHAR(255) NOT NULL UNIQUE, password_hash VARCHAR(255) NOT NULL, secret_key VARCHAR(64) NOT NULL, created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, expires_at TIMESTAMP WITH TIME ZONE NOT NULL, attempts INTEGER DEFAULT 0, confirmed BOOLEAN DEFAULT FALSE);

CREATE INDEX idx_temp_registrations_email ON temp_registrations(email);
CREATE INDEX idx_temp_registrations_secret_key ON temp_registrations(secret_key);
CREATE INDEX idx_temp_registrations_expires_at ON temp_registrations(expires_at);