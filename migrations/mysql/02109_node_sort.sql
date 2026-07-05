-- migrate:up
ALTER TABLE `nodes`
    ADD COLUMN `sort` INT UNSIGNED NOT NULL DEFAULT 0
    COMMENT 'Sort' AFTER `enabled`;
-- migrate:down
ALTER TABLE `nodes`
DROP COLUMN `sort`;
