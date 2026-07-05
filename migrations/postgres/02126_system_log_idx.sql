-- migrate:up
CREATE INDEX idx_type_date ON system_logs (type, date);

-- migrate:down
DROP INDEX IF EXISTS "idx_type_date";

