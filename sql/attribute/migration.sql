CREATE TABLE IF NOT EXISTS attribute_database
(
    attribute_id  UUID PRIMARY KEY,
    object_id     UUID,
    name          VARCHAR(256),
    description   TEXT,
    type          VARCHAR(32),
    default_value TEXT,
    FOREIGN KEY (object_id) REFERENCES object_database (object_id)
)