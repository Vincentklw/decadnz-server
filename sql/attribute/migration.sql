CREATE TABLE IF NOT EXISTS attribute_database
(
    attribute_id  UUID PRIMARY KEY,
    object_id     UUID         NOT NULL,
    name          VARCHAR(256) NOT NULL,
    description   TEXT,
    type          VARCHAR(32)  NOT NULL,
    default_value TEXT,
    created_at    DATETIME     NOT NULL,
    updated_at    DATETIME     NOT NULL,
    FOREIGN KEY (object_id) REFERENCES object_database (object_id),
    UNIQUE (object_id, name)
)