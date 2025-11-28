INSERT INTO attribute_database(attribute_id, object_id, name, description, type, default_value,
                               created_at, updated_at)
VALUES (UUID(), ?, ?, ?, ?, ?, NOW(), NOW())