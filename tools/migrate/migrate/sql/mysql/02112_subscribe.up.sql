ALTER TABLE `subscribe`
DROP COLUMN `group_id`,
ADD COLUMN `language` VARCHAR(255) NOT NULL DEFAULT ''
  COMMENT 'Language'
  AFTER `name`;

DROP TABLE IF EXISTS `subscribe_group`;