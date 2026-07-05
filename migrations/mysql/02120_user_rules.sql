-- migrate:up
ALTER TABLE `user`
    ADD COLUMN `rules` TEXT NULL
  COMMENT 'User rules for subscription'
  AFTER `created_at`;

-- migrate:down
ALTER TABLE `user`
DROP COLUMN IF EXISTS `rules`;

