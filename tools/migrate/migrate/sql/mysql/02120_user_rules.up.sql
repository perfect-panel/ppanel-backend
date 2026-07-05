ALTER TABLE `user`
    ADD COLUMN `rules` TEXT NULL
  COMMENT 'User rules for subscription'
  AFTER `created_at`;
