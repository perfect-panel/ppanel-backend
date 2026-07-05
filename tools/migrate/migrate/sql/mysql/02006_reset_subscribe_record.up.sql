-- migrations/02008_create_user_reset_subscribe_log.up.sql
-- Purpose: Create user_reset_subscribe_log table
-- Author: PPanel Team, 2025-04-22

CREATE TABLE IF NOT EXISTS `user_reset_subscribe_log`
(
    `id`                BIGINT     NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `user_id`           BIGINT     NOT NULL COMMENT 'User ID',
    `type`              TINYINT(1) NOT NULL COMMENT 'Type: 1: Auto 2: Advance 3: Paid',
    `order_no`          VARCHAR(255)        DEFAULT NULL COMMENT 'Order No.',
    `user_subscribe_id` BIGINT     NOT NULL COMMENT 'User Subscribe ID',
    `created_at`        DATETIME   NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT 'Creation Time',
    INDEX `idx_user_id` (`user_id`),
    INDEX `idx_user_subscribe_id` (`user_subscribe_id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;
