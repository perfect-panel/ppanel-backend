-- migrate:up
ALTER TABLE `server_rule_group`
ADD COLUMN `default` TINYINT(1) NOT NULL DEFAULT 0 COMMENT 'Is Default Group',
ADD COLUMN `type` VARCHAR(100) NOT NULL DEFAULT '' COMMENT 'Rule Group Type';
-- migrate:down
ALTER TABLE `server_rule_group`
DROP COLUMN `default`,
DROP COLUMN `type`;

