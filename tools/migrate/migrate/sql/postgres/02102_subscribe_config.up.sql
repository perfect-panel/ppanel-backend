INSERT INTO "system" ("id", "category", "key", "value", "type", "desc", "created_at", "updated_at")
VALUES
    (42, 'subscribe', 'UserAgentLimit', 'false', 'bool', 'User Agent Limit', '2025-04-22 14:25:16.637', '2025-04-22 14:25:16.637'),
    (43, 'subscribe', 'UserAgentList', '', 'string', 'User Agent List', '2025-04-22 14:25:16.637','2025-04-22 14:25:16.637') ON CONFLICT DO NOTHING;
SELECT setval(pg_get_serial_sequence('"system"', 'id'), COALESCE((SELECT MAX("id") FROM "system"), 1), true);
