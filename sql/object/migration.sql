CREATE TABLE IF NOT EXISTS object_database
(
    object_id  UUID PRIMARY KEY,
    namespace  VARCHAR(265) NOT NULL,
    name       VARCHAR(265) NOT NULL,
    created_at DATETIME     NOT NULL,
    UNIQUE (namespace, name)
);