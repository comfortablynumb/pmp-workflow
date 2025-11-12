-- Create credential_types table
CREATE TABLE IF NOT EXISTS credential_types (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    json_schema JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_credential_types_name ON credential_types(name);

-- Insert common credential types
INSERT INTO credential_types (id, name, description, json_schema) VALUES
(
    '00000000-0000-0000-0000-000000000001',
    'github_token',
    'GitHub Personal Access Token',
    '{
        "type": "object",
        "properties": {
            "token": {
                "type": "string",
                "description": "GitHub Personal Access Token (classic or fine-grained)"
            }
        },
        "required": ["token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000002',
    'gitlab_token',
    'GitLab Personal Access Token',
    '{
        "type": "object",
        "properties": {
            "token": {
                "type": "string",
                "description": "GitLab Personal Access Token"
            },
            "base_url": {
                "type": "string",
                "description": "GitLab instance URL (default: https://gitlab.com)"
            }
        },
        "required": ["token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000003',
    'slack_bot_token',
    'Slack Bot Token',
    '{
        "type": "object",
        "properties": {
            "bot_token": {
                "type": "string",
                "description": "Slack Bot User OAuth Token (starts with xoxb-)"
            }
        },
        "required": ["bot_token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000004',
    'gmail_oauth',
    'Gmail OAuth2 Credentials',
    '{
        "type": "object",
        "properties": {
            "client_id": {
                "type": "string",
                "description": "OAuth2 Client ID"
            },
            "client_secret": {
                "type": "string",
                "description": "OAuth2 Client Secret"
            },
            "refresh_token": {
                "type": "string",
                "description": "OAuth2 Refresh Token"
            },
            "access_token": {
                "type": "string",
                "description": "OAuth2 Access Token (can be refreshed)"
            }
        },
        "required": ["client_id", "client_secret", "refresh_token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000005',
    'openai_api_key',
    'OpenAI API Key',
    '{
        "type": "object",
        "properties": {
            "api_key": {
                "type": "string",
                "description": "OpenAI API Key (starts with sk-)"
            },
            "organization": {
                "type": "string",
                "description": "Optional OpenAI Organization ID"
            }
        },
        "required": ["api_key"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000006',
    'gemini_api_key',
    'Google Gemini API Key',
    '{
        "type": "object",
        "properties": {
            "api_key": {
                "type": "string",
                "description": "Google AI Studio API Key"
            }
        },
        "required": ["api_key"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000007',
    'aws_credentials',
    'AWS Credentials',
    '{
        "type": "object",
        "properties": {
            "access_key_id": {
                "type": "string",
                "description": "AWS Access Key ID"
            },
            "secret_access_key": {
                "type": "string",
                "description": "AWS Secret Access Key"
            },
            "region": {
                "type": "string",
                "description": "AWS Region (e.g., us-east-1)"
            },
            "session_token": {
                "type": "string",
                "description": "Optional AWS Session Token for temporary credentials"
            }
        },
        "required": ["access_key_id", "secret_access_key", "region"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000008',
    'database_connection',
    'Database Connection String',
    '{
        "type": "object",
        "properties": {
            "connection_string": {
                "type": "string",
                "description": "Database connection string (e.g., postgresql://user:pass@host:port/db)"
            }
        },
        "required": ["connection_string"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000009',
    'redis_connection',
    'Redis Connection',
    '{
        "type": "object",
        "properties": {
            "host": {
                "type": "string",
                "description": "Redis host"
            },
            "port": {
                "type": "integer",
                "description": "Redis port (default: 6379)"
            },
            "password": {
                "type": "string",
                "description": "Optional Redis password"
            },
            "database": {
                "type": "integer",
                "description": "Redis database number (default: 0)"
            }
        },
        "required": ["host"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000010',
    'http_basic_auth',
    'HTTP Basic Authentication',
    '{
        "type": "object",
        "properties": {
            "username": {
                "type": "string",
                "description": "Username for basic auth"
            },
            "password": {
                "type": "string",
                "description": "Password for basic auth"
            }
        },
        "required": ["username", "password"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000011',
    'google_drive_oauth',
    'Google Drive OAuth2 Credentials',
    '{
        "type": "object",
        "properties": {
            "client_id": {
                "type": "string",
                "description": "OAuth2 Client ID"
            },
            "client_secret": {
                "type": "string",
                "description": "OAuth2 Client Secret"
            },
            "refresh_token": {
                "type": "string",
                "description": "OAuth2 Refresh Token"
            },
            "access_token": {
                "type": "string",
                "description": "OAuth2 Access Token (can be refreshed)"
            }
        },
        "required": ["client_id", "client_secret", "refresh_token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000012',
    'dropbox_oauth',
    'Dropbox OAuth2 Token',
    '{
        "type": "object",
        "properties": {
            "access_token": {
                "type": "string",
                "description": "Dropbox OAuth2 Access Token"
            },
            "refresh_token": {
                "type": "string",
                "description": "Optional Refresh Token for long-lived access"
            }
        },
        "required": ["access_token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000013',
    'telegram_bot_token',
    'Telegram Bot Token',
    '{
        "type": "object",
        "properties": {
            "bot_token": {
                "type": "string",
                "description": "Telegram Bot API Token"
            }
        },
        "required": ["bot_token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000014',
    'twilio_api_key',
    'Twilio API Credentials',
    '{
        "type": "object",
        "properties": {
            "account_sid": {
                "type": "string",
                "description": "Twilio Account SID"
            },
            "auth_token": {
                "type": "string",
                "description": "Twilio Auth Token"
            }
        },
        "required": ["account_sid", "auth_token"]
    }'::jsonb
),
(
    '00000000-0000-0000-0000-000000000015',
    'ftp_credentials',
    'FTP/SFTP Credentials',
    '{
        "type": "object",
        "properties": {
            "host": {
                "type": "string",
                "description": "FTP/SFTP server hostname"
            },
            "port": {
                "type": "integer",
                "description": "Port number (default: 21 for FTP, 22 for SFTP)"
            },
            "username": {
                "type": "string",
                "description": "Username"
            },
            "password": {
                "type": "string",
                "description": "Password"
            },
            "protocol": {
                "type": "string",
                "description": "Protocol: ftp, ftps, or sftp",
                "enum": ["ftp", "ftps", "sftp"]
            },
            "private_key": {
                "type": "string",
                "description": "Private key for SFTP (alternative to password)"
            }
        },
        "required": ["host", "username", "protocol"]
    }'::jsonb
);
