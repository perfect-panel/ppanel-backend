-- migrate:up
ALTER TABLE `user`
    ADD COLUMN `referral_percentage` TINYINT UNSIGNED NOT NULL DEFAULT 0
      COMMENT 'Referral Percentage'
      AFTER `commission`,
    ADD COLUMN `only_first_purchase` TINYINT(1) NOT NULL DEFAULT 1
      COMMENT 'Only First Purchase'
      AFTER `referral_percentage`;

-- migrate:down
ALTER TABLE `user`
DROP COLUMN `referral_percentage`,
DROP COLUMN `only_first_purchase`;
