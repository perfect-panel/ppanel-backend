SET @column_exists = (
    SELECT COUNT(*)
    FROM INFORMATION_SCHEMA.COLUMNS
    WHERE TABLE_SCHEMA = DATABASE()
      AND TABLE_NAME = 'payment'
      AND COLUMN_NAME = 'sort'
);

SET @sql = IF(
    @column_exists > 0,
    'ALTER TABLE `payment` DROP COLUMN `sort`',
    'SELECT 1'
);

PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;
