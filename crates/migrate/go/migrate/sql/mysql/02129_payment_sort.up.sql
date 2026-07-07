SET @column_exists = (
    SELECT COUNT(*)
    FROM INFORMATION_SCHEMA.COLUMNS
    WHERE TABLE_SCHEMA = DATABASE()
      AND TABLE_NAME = 'payment'
      AND COLUMN_NAME = 'sort'
);

SET @sql = IF(
    @column_exists = 0,
    'ALTER TABLE `payment` ADD COLUMN `sort` bigint NOT NULL DEFAULT 0 COMMENT ''Sort'' AFTER `fee_amount`',
    'SELECT 1'
);

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

UPDATE `payment`
SET `sort` = `id`
WHERE `sort` = 0;
