-- migrations/02004_create_user_device_online_record.down.sql
-- Purpose: Drop user device online record table
-- Author: PPanel Team, 2025-04-22

DROP TABLE IF EXISTS `user_device_online_record`;

-- User subscribe table migration for removing finished_at column
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'user_subscribe'
                        AND COLUMN_NAME = 'finished_at');
SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `user_subscribe` DROP COLUMN `finished_at`',
              'SELECT 1'
           );
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Application config table migration for removing invitation_link column

SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'application_config'
                        AND COLUMN_NAME = 'invitation_link');

SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `application_config` DROP COLUMN `invitation_link`',
              'SELECT 1'
           );

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Application config table migration for removing kr_website_id column
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'application_config'
                        AND COLUMN_NAME = 'kr_website_id');

SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `application_config` DROP COLUMN `kr_website_id`',
              'SELECT 1'
           );

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;