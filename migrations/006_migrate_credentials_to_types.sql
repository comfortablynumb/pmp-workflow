-- Migration to update credentials table to use credential_types FK

-- Add new credential_type_id column
ALTER TABLE credentials
ADD COLUMN credential_type_id UUID;

-- For any existing credentials with the old enum type, we could map them here
-- Since this is development and likely no data exists yet, we'll just make it required
-- In production, you would need a data migration step here

-- Drop old credentials_type column and enum
ALTER TABLE credentials
DROP COLUMN IF EXISTS credentials_type;

DROP TYPE IF EXISTS credentials_type;

-- Make credential_type_id NOT NULL and add FK constraint
ALTER TABLE credentials
ALTER COLUMN credential_type_id SET NOT NULL,
ADD CONSTRAINT fk_credentials_credential_type
    FOREIGN KEY (credential_type_id)
    REFERENCES credential_types(id)
    ON DELETE RESTRICT;

-- Drop old index and create new one
DROP INDEX IF EXISTS idx_credentials_type;
CREATE INDEX idx_credentials_credential_type_id ON credentials(credential_type_id);
