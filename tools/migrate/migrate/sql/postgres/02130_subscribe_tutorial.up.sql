INSERT INTO "system" ("category", "key", "value", "type", "desc", "created_at", "updated_at")
SELECT 'subscribe', 'ShowTutorial', 'true', 'bool', 'Show tutorial section on the user document page', '2025-04-22 14:25:16.639', '2025-04-22 14:25:16.639'
    WHERE NOT EXISTS (
    SELECT 1 FROM "system" WHERE "category" = 'subscribe' AND "key" = 'ShowTutorial'
);
