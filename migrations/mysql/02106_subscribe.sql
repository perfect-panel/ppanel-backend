-- migrate:up
ALTER TABLE `subscribe`
ADD COLUMN `nodes` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Node IDs',
ADD COLUMN `node_tags` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Node Tags',
DROP COLUMN `server`,
DROP COLUMN `server_group`;

DROP TABLE IF EXISTS `server_rule_group`;

-- migrate:down
ALTER TABLE `subscribe`
DROP COLUMN `nodes`,
  DROP COLUMN `node_tags`,
  ADD COLUMN `server` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Server',
  ADD COLUMN `server_group` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Server Group';

