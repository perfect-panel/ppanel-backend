-- migrations/02005_create_user_device_online_record.up.sql
-- Purpose: Create table for tracking user device online records
-- Author: PPanel Team, 2025-04-22

CREATE TABLE IF NOT EXISTS `user_device_online_record`
(
    `id`             BIGINT       NOT NULL AUTO_INCREMENT PRIMARY KEY,
    `user_id`        BIGINT       NOT NULL COMMENT 'User ID',
    `identifier`     VARCHAR(255) NOT NULL COMMENT 'Device Identifier',
    `online_time`    DATETIME COMMENT 'Online Time',
    `offline_time`   DATETIME COMMENT 'Offline Time',
    `online_seconds` BIGINT COMMENT 'Offline Seconds',
    `duration_days`  BIGINT COMMENT 'Duration Days',
    `created_at`     DATETIME COMMENT 'Creation Time'
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_general_ci;


-- User subscribe table migration for adding finished_at column

SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'user_subscribe'
                        AND COLUMN_NAME = 'finished_at');

SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `user_subscribe` ADD COLUMN `finished_at` DATETIME NULL COMMENT ''Subscribe Finished Time'' AFTER `expire_time`',
              'SELECT 1'
           );

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;


-- Application config table migration for adding Link column

SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'application_config'
                        AND COLUMN_NAME = 'invitation_link');

SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `application_config` ADD COLUMN `invitation_link` TEXT NULL DEFAULT NULL COMMENT ''Invitation Link'' AFTER `startup_picture_skip_time`',
              'SELECT 1'
           );

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Application config table migration for adding kr_website_id column
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'application_config'
                        AND COLUMN_NAME = 'kr_website_id');

SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `application_config` ADD COLUMN `kr_website_id` VARCHAR(255) NULL DEFAULT NULL COMMENT ''KR Website ID'' AFTER `invitation_link`',
              'SELECT 1'
           );

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;
