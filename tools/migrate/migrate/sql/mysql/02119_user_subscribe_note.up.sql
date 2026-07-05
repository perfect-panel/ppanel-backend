ALTER TABLE `user_subscribe`
ADD COLUMN `note` VARCHAR(500) NOT NULL DEFAULT ''
  COMMENT 'User note for subscription'
  AFTER `status`;
