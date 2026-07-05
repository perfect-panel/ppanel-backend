-- 2025-04-22 16:16:00
-- Purpose: Update payment table
-- Author: PPanel Team, 2025-04-21

SET FOREIGN_KEY_CHECKS = 0;

-- Alter the order table to add a payment_id column (if not exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'order'
                        AND COLUMN_NAME = 'payment_id');
SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `order` ADD COLUMN `payment_id` bigint NOT NULL DEFAULT \'-1\' COMMENT \'Payment Id\' AFTER `commission`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Alter the payment table to add a platform column (if not exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'platform');
SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `payment` ADD COLUMN `platform` varchar(100) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL COMMENT \'Payment Platform\' AFTER `name`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Drop the mark column from the payment table (only if exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'mark');
SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `payment` DROP COLUMN `mark`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Alter the payment table to add a description column (if not exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'description');
SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `payment` ADD COLUMN `description` text CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci COMMENT \'Payment Description\' AFTER `platform`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Alter the payment table to add a token column (if not exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'token');
SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `payment` ADD COLUMN `token` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT \'Payment Token\' AFTER `description`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET FOREIGN_KEY_CHECKS = 1;