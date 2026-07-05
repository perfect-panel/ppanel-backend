-- migrate:up
ALTER TABLE traffic_log ADD INDEX idx_timestamp (timestamp);

-- migrate:down
ALTER TABLE traffic_log DROP INDEX idx_timestamp;

