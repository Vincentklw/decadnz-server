CREATE TABLE IF NOT EXISTS provider_database
(
    provider_id               UUID PRIMARY KEY,
    provider_name             VARCHAR(265) NOT NULL,
    api_endpoint              TEXT         NOT NULL,
    api_endpoint_access_token TEXT         NOT NULL
)