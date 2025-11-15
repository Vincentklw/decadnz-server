CREATE TABLE IF NOT EXISTS object_database
(
    object_id  UUID,
    namespace  VARCHAR(265),
    name       VARCHAR(265),
    created_at DATETIME
);