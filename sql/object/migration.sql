CREATE TABLE IF NOT EXISTS object_database
(
    object_id  UUID PRIMARY KEY,
    namespace  VARCHAR(265),
    name       VARCHAR(265),
    created_at DATETIME
);