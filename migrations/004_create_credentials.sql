-- Create credentials type enum
CREATE TYPE credentials_type AS ENUM ('database', 'apikey', 'oauth', 'basicauth', 'custom');

-- Create credentials table
CREATE TABLE IF NOT EXISTS credentials (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    credentials_type credentials_type NOT NULL,
    encrypted_data BYTEA NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(name)
);

-- Create index on credentials name for faster lookups
CREATE INDEX idx_credentials_name ON credentials(name);

-- Create index on credentials type
CREATE INDEX idx_credentials_type ON credentials(credentials_type);
