-- migrate:up
CREATE INDEX IF NOT EXISTS "idx_timestamp" ON "traffic_log" ("timestamp");

-- migrate:down
DROP INDEX IF EXISTS "idx_timestamp";

