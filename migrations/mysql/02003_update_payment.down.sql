-- migrations/02003_update_payment.down.sql
-- Purpose: Revert updates to payment and order tables
-- Author: PPanel Team, 2025-04-21

SET FOREIGN_KEY_CHECKS = 0;

-- Drop payment_id column from order table (if exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'order'
                        AND COLUMN_NAME = 'payment_id');
SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `order` DROP COLUMN `payment_id`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Drop platform column from payment table (if exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'platform');
SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `payment` DROP COLUMN `platform`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Drop description column from payment table (if exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'description');
SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `payment` DROP COLUMN `description`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Drop token column from payment table (if exists)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'token');
SET @sql = IF(@column_exists > 0,
              'ALTER TABLE `payment` DROP COLUMN `token`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- Optionally restore mark column (if needed, adjust definition as per original schema)
SET @column_exists = (SELECT COUNT(*)
                      FROM INFORMATION_SCHEMA.COLUMNS
                      WHERE TABLE_SCHEMA = DATABASE()
                        AND TABLE_NAME = 'payment'
                        AND COLUMN_NAME = 'mark');
SET @sql = IF(@column_exists = 0,
              'ALTER TABLE `payment` ADD COLUMN `mark` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci DEFAULT NULL COMMENT \'Payment Mark\' AFTER `name`',
              'SELECT 1');
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

SET FOREIGN_KEY_CHECKS = 1;