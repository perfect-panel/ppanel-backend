ALTER TABLE `subscribe`
DROP COLUMN `nodes`,
  DROP COLUMN `node_tags`,
  ADD COLUMN `server` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Server',
  ADD COLUMN `server_group` VARCHAR(255) NOT NULL DEFAULT '' COMMENT 'Server Group';
