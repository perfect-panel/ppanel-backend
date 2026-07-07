-- 只有当 ads 表中不存在 description 字段时才添加
SET
@col_exists := (
    SELECT COUNT(*)
    FROM INFORMATION_SCHEMA.COLUMNS
    WHERE TABLE_SCHEMA = DATABASE()
      AND TABLE_NAME = 'ads'
      AND COLUMN_NAME = 'description'
);

SET
@query := IF(
    @col_exists = 0,
    'ALTER TABLE `ads` ADD COLUMN `description` VARCHAR(255) DEFAULT '''' COMMENT ''Description'';',
    'SELECT "Column `description` already exists"'
);

PREPARE stmt FROM @query;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;
