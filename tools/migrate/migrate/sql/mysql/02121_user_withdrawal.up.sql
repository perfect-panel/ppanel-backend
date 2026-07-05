CREATE TABLE IF NOT EXISTS `withdrawals` (
    `id` BIGINT NOT NULL AUTO_INCREMENT COMMENT 'Primary Key',
    `user_id` BIGINT NOT NULL COMMENT 'User ID',
    `amount` BIGINT NOT NULL COMMENT 'Withdrawal Amount',
    `content` TEXT COMMENT 'Withdrawal Content',
    `status` TINYINT(1) NOT NULL DEFAULT 0 COMMENT 'Withdrawal Status',
    `reason` VARCHAR(500) NOT NULL DEFAULT '' COMMENT 'Rejection Reason',
    `created_at` DATETIME NOT NULL COMMENT 'Creation Time',
    `updated_at` DATETIME NOT NULL COMMENT 'Update Time',
    PRIMARY KEY (`id`),
    KEY `idx_user_id` (`user_id`)
    ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

INSERT IGNORE INTO `system` (`category`, `key`, `value`, `type`, `desc`, `created_at`, `updated_at`)
VALUES
    ('invite', 'WithdrawalMethod', '', 'string', 'withdrawal method', '2025-04-22 14:25:16.637', '2025-04-22 14:25:16.637');