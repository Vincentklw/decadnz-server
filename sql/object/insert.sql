INSERT INTO object_database(object_id, namespace, name, created_at, updated_at)
VALUES (UUID(), ?, ?, NOW(), NOW());