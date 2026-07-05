ALTER TABLE `subscribe`
ADD COLUMN `nodes` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Node IDs',
ADD COLUMN `node_tags` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Node Tags',
DROP COLUMN `server`,
DROP COLUMN `server_group`;

DROP TABLE IF EXISTS `server_rule_group`;
