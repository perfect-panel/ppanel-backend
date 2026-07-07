-- 添加 algo 列（如果不存在）
SET @dbname = DATABASE();
SET @tablename = 'user';
SET @colname = 'algo';
SET @sql = (
    SELECT IF(
        COUNT(*) = 0,
        'ALTER TABLE `user` ADD COLUMN `algo` VARCHAR(20) NOT NULL DEFAULT ''default'' COMMENT ''Encryption Algorithm'' AFTER `password`;',
        'SELECT "Column `algo` already exists";'
    )
    FROM information_schema.COLUMNS
    WHERE TABLE_SCHEMA = @dbname
      AND TABLE_NAME = @tablename
      AND COLUMN_NAME = @colname
);
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;

-- 添加 salt 列（如果不存在）
SET @colname = 'salt';
SET @sql = (
    SELECT IF(
        COUNT(*) = 0,
        'ALTER TABLE `user` ADD COLUMN `salt` VARCHAR(20) NOT NULL DEFAULT ''default'' COMMENT ''Password Salt'' AFTER `algo`;',
        'SELECT "Column `salt` already exists";'
    )
    FROM information_schema.COLUMNS
    WHERE TABLE_SCHEMA = @dbname
      AND TABLE_NAME = @tablename
      AND COLUMN_NAME = @colname
);
PREPARE stmt FROM @sql;
EXECUTE stmt;
DEALLOCATE PREPARE stmt;
